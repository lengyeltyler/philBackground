const hre = require("hardhat");

async function main() {
  const network = hre.network.name;
  console.log(`🌌 Deploying Enhanced zkBgNFT Galaxy contract to ${network}...`);

  // Get the enhanced contract factory
  const zkBgNFT = await hre.ethers.getContractFactory("zkBgNFT");
  
  console.log("📋 Contract factory loaded successfully");
  console.log("🔧 Contract features: Galaxy micro-triangles, batch minting, enhanced metadata");
  
  // Deploy the enhanced contract
  console.log("🚀 Deploying contract...");
  const zkBg = await zkBgNFT.deploy();
  
  // Wait for deployment
  await zkBg.waitForDeployment();
  
  const address = await zkBg.getAddress();
  console.log("✅ Enhanced zkBgNFT deployed to:", address);
  
  // Verify enhanced contract functionality
  console.log("🔍 Verifying enhanced contract functions...");
  try {
    // Test that new functions exist
    const maxTrianglesPerBatch = await zkBg.MAX_TRIANGLES_PER_BATCH();
    const galaxyParticlesPerArm = await zkBg.GALAXY_PARTICLES_PER_ARM();
    const maxGalaxyTriangles = await zkBg.MAX_GALAXY_TRIANGLES();
    
    console.log("✅ Enhanced contract validation:");
    console.log(`   • MAX_TRIANGLES_PER_BATCH: ${maxTrianglesPerBatch}`);
    console.log(`   • GALAXY_PARTICLES_PER_ARM: ${galaxyParticlesPerArm}`);
    console.log(`   • MAX_GALAXY_TRIANGLES: ${maxGalaxyTriangles}`);
    
    // Verify new functions exist
    console.log("🔍 Checking enhanced functions...");
    const contractInterface = zkBg.interface;
    const hasInitializeGalaxy = contractInterface.hasFunction('initializeGalaxy');
    const hasAddTriangleBatch = contractInterface.hasFunction('addTriangleBatch');
    const hasGetGalaxyMetadata = contractInterface.hasFunction('getGalaxyMetadata');
    const hasGetTriangleBatch = contractInterface.hasFunction('getTriangleBatch');
    
    console.log("✅ Enhanced functions available:");
    console.log(`   • initializeGalaxy: ${hasInitializeGalaxy}`);
    console.log(`   • addTriangleBatch: ${hasAddTriangleBatch}`);
    console.log(`   • getGalaxyMetadata: ${hasGetGalaxyMetadata}`);
    console.log(`   • getTriangleBatch: ${hasGetTriangleBatch}`);
    
    if (!hasInitializeGalaxy || !hasAddTriangleBatch) {
      throw new Error("Enhanced contract functions not found - deployment may have failed");
    }
    
  } catch (error) {
    console.log("❌ Enhanced contract validation failed:", error.message);
    console.log("⚠️  The contract may not have the galaxy enhancements");
  }
  
  if (network === "hardhat" || network === "localhost") {
    console.log("🔧 Local deployment - no verification needed");
    console.log("💰 Unlimited test ETH available!");
    console.log("⚡ Instant transactions!");
    console.log("🌌 Galaxy complexity supported!");
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
  
  // Save deployment info with enhanced metadata
  const fs = require("fs");
  const deploymentInfo = {
    network: network,
    address: address,
    deployedAt: new Date().toISOString(),
    deployer: (await hre.ethers.getSigners())[0].address,
    // Enhanced deployment metadata
    contractType: "Enhanced Galaxy zkBgNFT",
    features: {
      galaxyMicroTriangles: true,
      batchMinting: true,
      enhancedMetadata: true,
      maxTrianglesPerBatch: 100,
      galaxyParticlesPerArm: 69
    },
    gasLimits: {
      deployment: (await zkBg.deploymentTransaction()).gasLimit?.toString() || "unknown",
      recommended: "15000000" // 15M gas for galaxy minting
    }
  };
  
  // Save to blockchain directory
  fs.writeFileSync(
    "./deployments.json",
    JSON.stringify(deploymentInfo, null, 2)
  );
  
  // Also save to static directory for frontend access
  try {
    fs.writeFileSync(
      "../static/deployments.json", 
      JSON.stringify(deploymentInfo, null, 2)
    );
    console.log("📄 Deployment info saved to deployments.json and static/deployments.json");
  } catch (error) {
    console.log("📄 Deployment info saved to deployments.json");
    console.log("⚠️  Could not copy to static/ directory:", error.message);
  }
  
  if (network === "hardhat" || network === "localhost") {
    console.log("\n🎯 Next steps:");
    console.log("1. Keep this terminal running (Hardhat node)");
    console.log("2. Contract address automatically updated in frontend");
    console.log("3. Start your Rust server: cargo run");
    console.log("4. Open http://localhost:3030/spiral_visualizer_hardhat.html");
    console.log("5. Connect MetaMask to localhost:8545 (Chain ID: 1337)");
    console.log("6. Generate galaxy spirals with micro-triangles!");
    console.log("\n🌌 Galaxy Features:");
    console.log("   • 69 particles per arm (vs 15 previously)");
    console.log("   • Micro-triangles for particle effect");
    console.log("   • Batch minting for gas optimization");
    console.log("   • Enhanced metadata and validation");
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("❌ Deployment failed:", error);
    process.exit(1);
  });