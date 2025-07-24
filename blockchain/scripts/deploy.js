const hre = require("hardhat");

async function main() {
  const network = hre.network.name;
  console.log(`🚀 Deploying zkBgNFT contract to ${network}...`);

  // Get the contract factory
  const zkBgNFT = await hre.ethers.getContractFactory("zkBgNFT");
  
  // Deploy the contract
  const zkBg = await zkBgNFT.deploy();
  
  // Wait for deployment
  await zkBg.waitForDeployment();
  
  const address = await zkBg.getAddress();
  console.log("✅ zkBgNFT deployed to:", address);
  
  if (network === "hardhat" || network === "localhost") {
    console.log("🔧 Local deployment - no verification needed");
    console.log("💰 Unlimited test ETH available!");
    console.log("⚡ Instant transactions!");
  } else {
    // Wait for a few block confirmations on testnets
    console.log("⏳ Waiting for block confirmations...");
    await zkBg.deploymentTransaction().wait(5);
    
    // Verify on Etherscan for testnets
    console.log("🔍 Verifying contract on Etherscan...");
    try {
      await hre.run("verify:verify", {
        address: address,
        constructorArguments: [],
      });
      console.log("✅ Contract verified on Etherscan");
    } catch (error) {
      console.log("❌ Verification failed:", error.message);
    }
  }
  
  // Save deployment info
  const fs = require("fs");
  const deploymentInfo = {
    network: network,
    address: address,
    deployedAt: new Date().toISOString(),
    deployer: (await hre.ethers.getSigners())[0].address
  };
  
  fs.writeFileSync(
    "./deployments.json",
    JSON.stringify(deploymentInfo, null, 2)
  );
  
  console.log("📄 Deployment info saved to deployments.json");
  
  if (network === "hardhat" || network === "localhost") {
    console.log("\n🎯 Next steps:");
    console.log("1. Keep this terminal running (Hardhat node)");
    console.log("2. Update the CONTRACT_ADDRESS in spiral_visualizer_blockchain.html");
    console.log("3. Start your Rust server: cargo run");
    console.log("4. Open http://localhost:3030/spiral_visualizer_blockchain.html");
    console.log("5. Connect MetaMask to localhost:8545 (Chain ID: 1337)");
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });