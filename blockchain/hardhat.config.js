require("@nomicfoundation/hardhat-toolbox");
require("dotenv").config();

// Your Infura API key
const INFURA_API_KEY = "3636868543c14064a73d249d3af3794d";

// You'll need to add a private key for deployment
// NEVER commit your private key - use environment variables
const SEPOLIA_PRIVATE_KEY = process.env.SEPOLIA_PRIVATE_KEY || "0x0000000000000000000000000000000000000000000000000000000000000000";

module.exports = {
  solidity: {
    version: "0.8.20",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
  networks: {
    hardhat: {
      chainId: 1337,
      // ENHANCED: Galaxy-level gas limits
      gas: 30000000,        // 30M gas limit (vs default 12M)
      blockGasLimit: 30000000,
      allowUnlimitedContractSize: true, // Allow large contracts
      timeout: 1800000,     // 30 minutes timeout
      accounts: {
        count: 20,          // More test accounts
        accountsBalance: "1000000000000000000000" // 1000 ETH each
      }
    },
    sepolia: {
      url: `https://sepolia.infura.io/v3/${INFURA_API_KEY}`,
      accounts: [SEPOLIA_PRIVATE_KEY],
      chainId: 11155111,
      // Enhanced gas settings for mainnet deployment
      gas: 10000000,
      gasPrice: 20000000000, // 20 gwei
      timeout: 1800000
    }
  },
  etherscan: {
    // Add your Etherscan API key for contract verification
    apiKey: process.env.ETHERSCAN_API_KEY || ""
  },
  paths: {
    sources: "./contracts",
    tests: "./test",
    cache: "./cache",
    artifacts: "./artifacts"
  },
  // ENHANCED: Mocha settings for complex tests
  mocha: {
    timeout: 1800000 // 30 minutes for complex tests
  }
};