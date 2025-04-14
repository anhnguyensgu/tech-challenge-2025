use backend::current_block::{current_offset, update_offset, BlockOffset};
use backend::database;
use backend::state::new_web3_client;
use backend::token::upsert_token_owner;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::sleep;
use web3::types::{Address, BlockNumber, FilterBuilder, H160, H256, U256};

#[tokio::main]
async fn main() {
    let web3_client = Arc::new(new_web3_client().await);
    let pg_pool = database::init_pg_pool().await;

    let contract_address = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS is not set");
    let current_offset = current_offset(&pg_pool, &contract_address)
        .await
        .expect("should query successfully");
    let current_offset = current_offset.unwrap_or_else(|| {
        let start_block = env::var("START_BLOCK")
            .expect("START_BLOCK is not set")
            .parse::<i64>()
            .expect("should parse to i64");
        BlockOffset {
            current_offset: start_block,
        }
    });
    println!("Current offset: {:?}", current_offset);

    let contract_address_param: H160 = contract_address.parse().expect("Invalid contract address");
    // let current_offset = RwLock::new(current_offset.expect("should work"));
    let transfer_event_sig = H256::from_slice(&web3::signing::keccak256(
        b"Transfer(address,address,uint256)",
    ));

    let last_block = web3_client
        .eth()
        .block_number()
        .await
        .expect("should get block number")
        .as_u64() as i64;
    let last_block = Arc::new(RwLock::new(last_block));
    let cloned_last_block = last_block.clone();

    let cloned_client = web3_client.clone();
    let _sync_latest_block = tokio::task::spawn(async move {
        loop {
            let new_last_block = cloned_client
                .clone()
                .eth()
                .block_number()
                .await
                .expect("should get block number")
                .as_u64() as i64;
            let mut last_block = cloned_last_block.write().await;
            *last_block = new_last_block;
            println!("Last block: {:?}", new_last_block);
            sleep(tokio::time::Duration::from_secs(3)).await;
        }
    });

    let last_block = last_block.clone();

    let client = web3_client.clone();
    let sync_nft_task = tokio::task::spawn(async move {
        let mut from = current_offset.current_offset + 1;
        let pool = pg_pool.clone();
        loop {
            let last_block = last_block.read().await;
            //should handle reorg
            let safe_block = *last_block;
            let to = safe_block.min(from + 500);
            println!("Syncing NFT... from {from} to {to} with latest {last_block}");
            if from > to {
                println!("Current offset is greater than last {from} > {to}");
                sleep(tokio::time::Duration::from_secs(3)).await;
                continue;
            }
            let target_block = BlockNumber::Number(from.into());
            let next_target_block = BlockNumber::Number(to.into());
            let filter = FilterBuilder::default()
                .address(vec![contract_address_param])
                .topics(Some(vec![transfer_event_sig]), None, None, None)
                .from_block(target_block)
                .to_block(next_target_block)
                .build();
            let logs = client.eth().logs(filter).await.unwrap();

            // Print the logs
            for log in logs {
                let Some(block_number) = log.block_number else {
                    println!("No block number found in log");
                    continue;
                };
                if let (Some(to_topic), Some(token_id_topic)) =
                    (log.topics.get(2), log.topics.get(3))
                {
                    // Extract `to` address (last 20 bytes of 32-byte topic)
                    let to_bytes = &to_topic.as_bytes()[12..];
                    let to_address = Address::from_slice(to_bytes);

                    // Extract token ID as U256
                    let token_id = U256::from_big_endian(token_id_topic.as_bytes());

                    println!(
                        "| Block {block_number} | To: {:?} | Token ID: {}",
                        to_address, token_id
                    );
                    match upsert_token_owner(
                        &pool,
                        token_id.as_u32() as i32,
                        &format!("{:?}", to_address),
                        block_number.as_u64() as i64,
                    )
                    .await
                    {
                        Ok(_) => println!("Upserted token owner"),
                        Err(err) => println!("Error upserting token owner: {:?}", err),
                    };
                }
            }
            let contract_address = contract_address.to_string();
            if let Err(e) = update_offset(&pool, &contract_address, to).await {
                println!("Error updating offset: {:?}", e);
            }

            from = to + 1;
            sleep(tokio::time::Duration::from_secs(3)).await;
        }
    });

    let _ = tokio::join!(sync_nft_task);
}
