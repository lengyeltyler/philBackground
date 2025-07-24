# zkBg NFT Blockchain Setup

This directory contains the smart contracts and deployment scripts for minting zkBg spiral NFTs on Ethereum.

## Prerequisites

1. Node.js v16+ and npm
2. MetaMask wallet with Sepolia testnet ETH
3. Your zkBg Rust server running (`cargo run`)

## Setup Steps

### 1. Install Dependencies

```bash
cd blockchain
npm install
```

### 2. Configure Environment

Copy the example environment file:
```bash
cp .env.example .env
```

Edit `.env` and add your private key:
```
SEPOLIA_PRIVATE_KEY=your_wallet_private_key_here
```

⚠️ **NEVER commit your private key!** The `.env` file should be in `.gitignore`.

### 3. Get Sepolia Test ETH

You'll need test ETH to deploy contracts and mint NFTs:
- [Sepolia Faucet](https://sepoliafaucet.com/)
- [Alchemy Sepolia Faucet](https://sepoliafaucet.com/)
- [Infura Sepolia Faucet](https://www.infura.io/faucet/sepolia)

### 4. Compile Contracts

```bash
npm run compile
```

### 5. Deploy to Sepolia

```bash
npm run deploy:sepolia
```

This will:
- Deploy the zkBgNFT contract
- Save the contract address to `deployments.json`
- Attempt to verify the contract on Etherscan

### 6. Update Frontend

After deployment, update the contract address in `spiral_visualizer_blockchain.html`:

```javascript
const CONTRACT_ADDRESS = "0x...your_deployed_contract_address...";
```

## Usage

1. Start your Rust server: `cargo run`
2. Open `http://localhost:3030/spiral_visualizer_blockchain.html`
3. Connect MetaMask (make sure you're on Sepolia)
4. Generate a spiral pattern
5. Click "Estimate Gas" to see real blockchain costs
6. Click "Mint NFT" to mint your spiral on-chain

## Contract Features

- **ERC-721 NFT**: Standard NFT implementation
- **On-Chain Storage**: Stores ZK proof hash and spiral configuration
- **Triangle Data**: Stores up to 50 triangles per NFT
- **Gas Optimized**: Limited data storage for reasonable gas costs

## Gas Costs (Sepolia Estimates)

- Base NFT Mint: ~50,000 gas
- ZK Proof Storage: ~80,000 gas
- Triangle Data: ~100,000-500,000 gas (depends on complexity)
- **Total**: ~250,000-650,000 gas

At 20 gwei gas price:
- Min: ~0.005 ETH (~$15 at $3000/ETH)
- Max: ~0.013 ETH (~$39 at $3000/ETH)

## Testing

Run the test suite:
```bash
npm test
```

## Mainnet Deployment

When ready for mainnet:

1. Add mainnet configuration to `hardhat.config.js`
2. Ensure sufficient mainnet ETH
3. Deploy with: `npx hardhat run scripts/deploy.js --network mainnet`
4. Consider using a multisig wallet for ownership

## Troubleshooting

### "Cannot connect to Rust server"
- Make sure your zkBg Rust server is running: `cargo run`

### "Insufficient funds"
- Get more Sepolia ETH from a faucet
- Check you're on the right network

### "Gas estimation failed"
- Ensure contract is deployed
- Check contract address is updated in frontend
- Verify you have enough ETH for gas

## Security Notes

- Never share your private key
- Always test on testnet first
- Consider audit before mainnet deployment
- Use hardware wallet for mainnet operations