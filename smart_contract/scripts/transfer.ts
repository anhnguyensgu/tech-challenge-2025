// scripts/call.js
import { ethers } from "ethers";
import fs from "fs";

require("dotenv").config();
const endpointUrl = process.env["RPC_URL"];
const privateKey = process.env["PRIVATE_KEY"] || "";
const contractAddress = process.env["CONTRACT_ADDRESS"] || "";

async function main() {
  const abi = JSON.parse(fs.readFileSync("./abi/SimpleNFT.json", "utf8"));

  const provider = new ethers.JsonRpcProvider(endpointUrl);
  const wallet = new ethers.Wallet(privateKey, provider);

  const contract = new ethers.Contract(contractAddress, abi.abi, wallet);

  const name = await contract.name(); // ERC721 example
  console.log("Contract name:", name);

  const tx = await contract.transfer(
    "0x32812d493CD0A0A4a5af5D48CDe4c88DB37916a1",
    9
  );

  await tx.wait();
  console.log("Minted! Tx hash:", tx.hash);
}

main().catch(console.error);
