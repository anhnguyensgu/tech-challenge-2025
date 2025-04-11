// scripts/call.js
const {ethers} = require("ethers");
const fs = require("fs");

require("dotenv").config();
const privateKey = process.env["PRIVATE_KEY"] || "";

async function main() {
    // Load ABI
    const abi = JSON.parse(fs.readFileSync("./abi/SimpleNFT.json", "utf8"));

    // Contract address
    const contractAddress = "0x21b06BEc125803635f0a9221655E731f6b0DB478";

    // Set up provider and wallet
    const provider = new ethers.JsonRpcProvider("https://ethereum-sepolia-rpc.publicnode.com");
    const wallet = new ethers.Wallet(privateKey, provider);

    // Create contract instance
    const contract = new ethers.Contract(contractAddress, abi.abi, wallet);

    // 👇 Call a read function (e.g. name, symbol, balanceOf)
    const name = await contract.name(); // ERC721 example
    console.log("Contract name:", name);

    // 👇 Call a write function (e.g. mint)
    const tx = await contract.mint(3);
    await tx.wait();
    console.log("Minted! Tx hash:", tx.hash);
}

main().catch(console.error);
