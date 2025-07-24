use warp::Filter;
use serde::{Deserialize, Serialize};
use reqwest;
use anyhow::Result;

// Import your existing circuit modules
mod circuits {
    pub mod simple_spirals;
}

use circuits::simple_spirals::{
    calculate_configuration_mapping, 
    generate_spiral_particles, 
    generate_spiral_triangles
};

#[derive(Serialize, Deserialize)]
struct SpiralRequest {
    seed: u64,
    canvas_size: u64,
}

#[derive(Serialize)]
struct SpiralResponse {
    seed: u64,
    variant: u64,
    spiral_type: u64,
    num_arms: u64,
    particles: Vec<ParticleData>,
    triangles: Vec<TriangleData>,
    config: ConfigData,
}

#[derive(Serialize)]
struct ParticleData {
    x: u64,
    y: u64,
    arm_index: u64,
    particle_index: u64,
}

#[derive(Serialize)]
struct TriangleData {
    vertices: [(u64, u64); 3],
    arm_index: u64,
    triangle_index: u64,
}

#[derive(Serialize)]
struct ConfigData {
    spiral_type: u64,
    num_arms: u64,
    spiral_quotient: u64,
    arms_quotient: u64,
    arms_remainder: u64,
}

// Gas estimation structures - DYNAMIC VERSION
#[derive(Serialize, Deserialize)]
struct GasRequest {
    seed: Option<u64>,        // Optional seed to estimate gas for specific spiral
    canvas_size: Option<u64>, // Optional canvas size
}

#[derive(Serialize)]
struct GasResponse {
    gas_price_gwei: f64,
    estimated_gas_units: u64,
    total_gas_cost_eth: f64,
    total_gas_cost_usd: f64,
    total_gas_cost_btc: f64,
    eth_price_usd: f64,
    btc_price_usd: f64,
    breakdown: GasBreakdown,
    timestamp: u64,
}

#[derive(Serialize)]
struct GasBreakdown {
    base_mint_gas: u64,
    zk_proof_gas: u64,
    metadata_gas: u64,
    total_gas: u64,
}

// External API response structures
#[derive(Deserialize)]
struct EtherscanGasResponse {
    result: EtherscanGasResult,
}

#[derive(Deserialize)]
struct EtherscanGasResult {
    #[serde(rename = "SafeGasPrice")]
    safe_gas_price: String,
    #[serde(rename = "ProposeGasPrice")]
    propose_gas_price: String,
    #[serde(rename = "FastGasPrice")]
    fast_gas_price: String,
}

#[derive(Deserialize)]
struct CoinGeckoResponse {
    ethereum: CurrencyPrice,
    bitcoin: CurrencyPrice,
}

#[derive(Deserialize)]
struct CurrencyPrice {
    usd: f64,
}

// FIXED spiral point calculation - full canvas distribution
fn calculate_spiral_point_fixed(
    arm_index: u64,
    particle_index: u64, 
    total_arms: u64,
    spiral_type: u64,
    canvas_size: u64,
) -> (u64, u64, u64) {
    const TRIG_TABLE_SIZE: u64 = 32;
    const SCALE_FACTOR: i64 = 10000;
    
    // Same trig tables as circuit
    const SIN_TABLE: [i64; 32] = [
        0, 1951, 3827, 5556, 7071, 8315, 9239, 9808,
        10000, 9808, 9239, 8315, 7071, 5556, 3827, 1951,
        0, -1951, -3827, -5556, -7071, -8315, -9239, -9808,
        -10000, -9808, -9239, -8315, -7071, -5556, -3827, -1951,
    ];
    
    const COS_TABLE: [i64; 32] = [
        10000, 9808, 9239, 8315, 7071, 5556, 3827, 1951,
        0, -1951, -3827, -5556, -7071, -8315, -9239, -9808,
        -10000, -9808, -9239, -8315, -7071, -5556, -3827, -1951,
        0, 1951, 3827, 5556, 7071, 8315, 9239, 9808,
    ];
    
    let particles_per_arm = 15u64;
    let max_radius = (canvas_size * 40) / 100; // Use 40% of canvas for radius
    
    let base_angle_index = (arm_index * TRIG_TABLE_SIZE) / total_arms;
    let t = (particle_index * 1000) / particles_per_arm;
    let progression = match spiral_type {
        0 => (t * 16) / 1000, // Tight
        1 => (t * 4) / 1000,  // Loose  
        _ => (t * 8) / 1000,  // Classic
    };
    
    let angle_index = (base_angle_index + progression) % TRIG_TABLE_SIZE;
    let radius = (t * max_radius) / 1000;
    
    let sin_val = SIN_TABLE[angle_index as usize];
    let cos_val = COS_TABLE[angle_index as usize];
    
    let center = canvas_size / 2;
    
    // FIXED: Use full sin/cos range, not abs() - allows full 360° distribution
    let x = ((center as i64) + ((radius as i64 * cos_val) / SCALE_FACTOR)).max(0) as u64;
    let y = ((center as i64) + ((radius as i64 * sin_val) / SCALE_FACTOR)).max(0) as u64;
    
    (x, y, angle_index)
}

// FIXED triangle generation - individual triangles at each position
fn generate_improved_triangles(positions: &[(u64, u64)], num_arms: u64) -> Vec<TriangleData> {
    let particles_per_arm = 15u64;
    let mut triangles = Vec::new();
    
    for arm_index in 0..num_arms {
        for particle_index in 0..particles_per_arm {
            let base_particle_idx = (arm_index * particles_per_arm + particle_index) as usize;
            
            if base_particle_idx < positions.len() {
                let center = positions[base_particle_idx];
                let size = 8 + (particle_index * 2); // Triangles get bigger as spiral progresses
                
                // Create triangle vertices around the center point  
                let angle = (arm_index as f64 * std::f64::consts::PI * 2.0) / num_arms as f64 + (particle_index as f64 * 0.3);
                
                let vertex1 = (
                    ((center.0 as f64) + (angle.cos() * size as f64)) as u64,
                    ((center.1 as f64) + (angle.sin() * size as f64)) as u64
                );
                
                let vertex2 = (
                    ((center.0 as f64) + ((angle + (std::f64::consts::PI * 2.0) / 3.0).cos() * size as f64)) as u64,
                    ((center.1 as f64) + ((angle + (std::f64::consts::PI * 2.0) / 3.0).sin() * size as f64)) as u64
                );
                
                let vertex3 = (
                    ((center.0 as f64) + ((angle + (std::f64::consts::PI * 4.0) / 3.0).cos() * size as f64)) as u64,
                    ((center.1 as f64) + ((angle + (std::f64::consts::PI * 4.0) / 3.0).sin() * size as f64)) as u64
                );
                
                triangles.push(TriangleData {
                    vertices: [vertex1, vertex2, vertex3],
                    arm_index,
                    triangle_index: particle_index,
                });
            }
        }
    }
    
    triangles
}

async fn generate_spiral_data(seed: u64, canvas_size: u64) -> Result<SpiralResponse, warp::Rejection> {
    // Use your actual ZK circuit logic for configuration
    let variant = seed % 41;
    let (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder) = 
        calculate_configuration_mapping(variant);
    
    // Generate particles using FIXED calculation
    let particles_per_arm = 15u64;
    let mut positions = Vec::new();
    let mut metadata = Vec::new();
    
    for arm_index in 0..num_arms {
        for particle_index in 0..particles_per_arm {
            let (x, y, angle_index) = calculate_spiral_point_fixed(
                arm_index,
                particle_index,
                num_arms,
                spiral_type,
                canvas_size,
            );
            
            positions.push((x, y));
            metadata.push((arm_index, particle_index, angle_index));
        }
    }
    
    // Generate triangles using FIXED method
    let triangles = generate_improved_triangles(&positions, num_arms);
    
    // Convert to response format
    let particles: Vec<ParticleData> = positions.iter().zip(metadata.iter())
        .map(|((x, y), (arm_index, particle_index, _angle_index))| ParticleData {
            x: *x,
            y: *y,
            arm_index: *arm_index,
            particle_index: *particle_index,
        })
        .collect();
    
    let config = ConfigData {
        spiral_type,
        num_arms,
        spiral_quotient,
        arms_quotient,
        arms_remainder,
    };
    
    Ok(SpiralResponse {
        seed,
        variant,
        spiral_type,
        num_arms,
        particles,
        triangles,
        config,
    })
}

// Gas estimation functions with YOUR API KEY
async fn fetch_eth_gas_price() -> Result<f64> {
    let url = "https://api.etherscan.io/api?module=gastracker&action=gasoracle&apikey=MF1UH981PQBWJXHNWNQW6AAX3A3ERVGYGH";
    
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await?;
    
    if response.status().is_success() {
        let gas_data: EtherscanGasResponse = response.json().await?;
        // Use "standard" gas price (ProposeGasPrice)
        let gas_price = gas_data.result.propose_gas_price.parse::<f64>()?;
        Ok(gas_price)
    } else {
        // Fallback to estimated gas price if API fails
        Ok(20.0) // 20 gwei fallback
    }
}

async fn fetch_crypto_prices() -> Result<(f64, f64)> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum,bitcoin&vs_currencies=usd";
    
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await?;
    
    if response.status().is_success() {
        let price_data: CoinGeckoResponse = response.json().await?;
        Ok((price_data.ethereum.usd, price_data.bitcoin.usd))
    } else {
        // Fallback prices if API fails
        Ok((3000.0, 45000.0)) // ETH: $3000, BTC: $45000
    }
}

fn estimate_ethereum_gas(spiral_data: &SpiralResponse) -> GasBreakdown {
    // Base costs that are always the same
    let base_mint_gas = 50000u64;     // Basic ERC-721 mint
    let zk_proof_gas = 80000u64;      // ZK proof verification
    
    // DYNAMIC metadata cost based on actual data size
    let metadata_size = calculate_metadata_size(spiral_data);
    let metadata_gas = calculate_metadata_gas(metadata_size);
    
    let total_gas = base_mint_gas + zk_proof_gas + metadata_gas;
    
    GasBreakdown {
        base_mint_gas,
        zk_proof_gas,
        metadata_gas,
        total_gas,
    }
}

fn calculate_metadata_size(spiral_data: &SpiralResponse) -> u64 {
    // Calculate the actual size of metadata that will be stored on-chain
    let mut size = 0u64;
    
    // Base metadata (name, description, etc.) - ~500 bytes
    size += 500;
    
    // Spiral configuration data - ~200 bytes
    size += 200;
    
    // Triangle coordinates - each triangle is ~50 bytes when stored efficiently
    size += (spiral_data.triangles.len() as u64) * 50;
    
    // Particle positions - each particle ~20 bytes
    size += (spiral_data.particles.len() as u64) * 20;
    
    // ZK proof data - ~300 bytes
    size += 300;
    
    // SVG/visual representation data - varies by complexity
    // More arms = more complex = more data
    let visual_complexity = spiral_data.num_arms * 100; // ~100 bytes per arm
    size += visual_complexity;
    
    size
}

fn calculate_metadata_gas(metadata_size_bytes: u64) -> u64 {
    // Ethereum storage costs:
    // - Setting a storage slot (32 bytes) from 0 costs 20,000 gas
    // - Modifying existing storage costs 5,000 gas
    // - Each byte in calldata costs ~16 gas
    
    let storage_slots = (metadata_size_bytes + 31) / 32; // Round up to 32-byte slots
    let base_storage_cost = storage_slots * 20000; // 20k gas per new storage slot
    
    // Additional costs for complex data structures and encoding
    let encoding_overhead = metadata_size_bytes * 50; // ~50 gas per byte for complex encoding
    
    // Transaction calldata cost
    let calldata_cost = metadata_size_bytes * 16; // 16 gas per byte
    
    base_storage_cost + encoding_overhead + calldata_cost
}

async fn calculate_gas_costs_for_spiral(spiral_data: &SpiralResponse) -> Result<GasResponse, warp::Rejection> {
    // Fetch current gas price and crypto prices
    let gas_price_gwei = match fetch_eth_gas_price().await {
        Ok(price) => price,
        Err(_) => 20.0, // Fallback
    };
    
    let (eth_price_usd, btc_price_usd) = match fetch_crypto_prices().await {
        Ok(prices) => prices,
        Err(_) => (3000.0, 45000.0), // Fallback
    };
    
    // Get DYNAMIC gas estimates based on actual spiral data
    let breakdown = estimate_ethereum_gas(spiral_data);
    
    // Calculate costs
    let gas_price_eth = gas_price_gwei / 1_000_000_000.0; // Convert gwei to ETH
    let total_gas_cost_eth = (breakdown.total_gas as f64) * gas_price_eth;
    let total_gas_cost_usd = total_gas_cost_eth * eth_price_usd;
    let total_gas_cost_btc = total_gas_cost_usd / btc_price_usd;
    
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    Ok(GasResponse {
        gas_price_gwei,
        estimated_gas_units: breakdown.total_gas,
        total_gas_cost_eth,
        total_gas_cost_usd,
        total_gas_cost_btc,
        eth_price_usd,
        btc_price_usd,
        breakdown,
        timestamp,
    })
}

async fn handle_gas_estimation(request: GasRequest) -> Result<impl warp::Reply, warp::Rejection> {
    // Use provided seed or default to representative sample
    let seed = request.seed.unwrap_or(12345u64);
    let canvas_size = request.canvas_size.unwrap_or(500u64);
    
    // Generate spiral data for the specific seed to get accurate gas estimate
    let spiral_data = generate_spiral_data(seed, canvas_size).await?;
    let response = calculate_gas_costs_for_spiral(&spiral_data).await?;
    Ok(warp::reply::json(&response))
}

async fn handle_generate_spiral(request: SpiralRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let response = generate_spiral_data(request.seed, request.canvas_size).await?;
    Ok(warp::reply::json(&response))
}

#[tokio::main]
async fn main() {
    // API route for generating spiral data
    let api_generate = warp::path("api")
        .and(warp::path("generate"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_generate_spiral);
    
    // API route for gas estimation
    let api_gas = warp::path("api")
        .and(warp::path("gas"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_gas_estimation);
    
    // Serve static files (HTML, CSS, JS)
    let static_files = warp::fs::dir("static");
    
    // Health check endpoint
    let health = warp::path("health")
        .map(|| "zkBg Server Running!");
    
    let routes = api_generate
        .or(api_gas)
        .or(static_files)
        .or(health)
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_headers(vec!["content-type", "authorization"])
                .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        );
    
    println!("🌀 zkBg Server starting on http://localhost:3030");
    println!("📊 Visualizer: http://localhost:3030/spiral_visualizer.html");
    println!("🔧 API: POST http://localhost:3030/api/generate");
    println!("⛽ Gas API: POST http://localhost:3030/api/gas (Ethereum + USD/BTC + YOUR API KEY)");
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}