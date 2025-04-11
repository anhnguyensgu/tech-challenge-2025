import {HardhatUserConfig} from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import 'dotenv/config';

const endpointUrl = process.env["RPC_URL"];
const privateKey = process.env["PRIVATE_KEY"] || "";
const config: HardhatUserConfig = {
    solidity: "0.8.28",
    networks: {
        sepolia: {
            url: endpointUrl,
            accounts: [privateKey],
        },
    },
};

export default config;
