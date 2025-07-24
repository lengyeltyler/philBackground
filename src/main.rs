use warp::Filter;
use serde::{Deserialize, Serialize};
use reqwest;
use anyhow::Result;

// Import enhanced circuit modules
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
    // NEW: Galaxy-specific metadata
    galaxy_stats: GalaxyStats,
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
    triangle_type: u64, // NEW: 0=spiral particle, 1=star, 2=core, 3=dust
    size: u64,          // NEW: Triangle size for micro-triangle rendering
}

#[derive(Serialize)]
struct ConfigData {
    spiral_type: u64,
    num_arms: u64,
    spiral_quotient: u64,
    arms_quotient: u64,
    arms_remainder: u64,
}

// NEW: Galaxy-specific statistics
#[derive(Serialize)]
struct GalaxyStats {
    particles_per_arm: u64,
    total_particles: u64,
    micro_triangles: u64,
    galaxy_type: String,
    density_factor: f64,
}

// Gas estimation structures
#[derive(Serialize, Deserialize)]
struct GasRequest {
    seed: Option<u64>,
    canvas_size: Option<u64>,
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
    galaxy_complexity_gas: u64, // NEW: Extra gas for galaxy complexity
    total_gas: u64,
}

// External API response structures (unchanged)
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

// ENHANCED: Galaxy spiral point calculation with micro-triangle generation
fn calculate_galaxy_spiral_point_enhanced(
    arm_index: u64,
    particle_index: u64, 
    total_arms: u64,
    spiral_type: u64,
    canvas_size: u64,
) -> (u64, u64, u64) {
    const TRIG_TABLE_SIZE: u64 = 32;
    const SCALE_FACTOR: i64 = 10000;
    
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
    
    let particles_per_arm = 69u64; // GALAXY DENSITY
    let max_radius = (canvas_size * 40) / 100; // Galaxy spread
    
    let base_angle_index = (arm_index * TRIG_TABLE_SIZE) / total_arms;
    let t = (particle_index * 1000) / particles_per_arm;
    
    // ENHANCED GALAXY SPIRAL PATTERNS
    let progression = match spiral_type {
        0 => (t * 20) / 1000, // Tight galaxy - dense core, tight arms
        1 => (t * 3) / 1000,  // Loose galaxy - spread out, open arms  
        _ => (t * 10) / 1000, // Classic galaxy - balanced spiral
    };
    
    let angle_index = (base_angle_index + progression) % TRIG_TABLE_SIZE;
    let radius = (t * max_radius) / 1000;
    
    let sin_val = SIN_TABLE[angle_index as usize];
    let cos_val = COS_TABLE[angle_index as usize];
    
    let center = canvas_size / 2;
    
    // Enhanced positioning for galaxy effect
    let x = ((center as i64) + ((radius as i64 * cos_val) / SCALE_FACTOR)).max(0) as u64;
    let y = ((center as i64) + ((radius as i64 * sin_val) / SCALE_FACTOR)).max(0) as u64;
    
    (x, y, angle_index)
}

// ENHANCED: Generate micro-triangles for galaxy particle effect
fn generate_enhanced_galaxy_triangles(positions: &[(u64, u64)], num_arms: u64) -> Vec<TriangleData> {
    let particles_per_arm = 69u64;
    let mut triangles = Vec::new();
    
    for arm_index in 0..num_arms {
        for particle_index in 0..particles_per_arm {
            let base_particle_idx = (arm_index * particles_per_arm + particle_index) as usize;
            
            if base_particle_idx < positions.len() {
                let center = positions[base_particle_idx];
                
                // MICRO-TRIANGLE sizing for galaxy particle effect
                let base_size = 6u64;
                let size_reduction = (particle_index * 4) / particles_per_arm; // Smaller toward edge
                let triangle_size = (base_size - size_reduction).max(2); // Minimum 2 pixels
                
                // Create micro-triangle vertices around the center point
                let half_size = triangle_size / 2;
                let vertex1 = (center.0, center.1.saturating_sub(half_size));           // Top
                let vertex2 = (center.0.saturating_sub(half_size), center.1 + half_size); // Bottom left
                let vertex3 = (center.0 + half_size, center.1 + half_size);            // Bottom right
                
                triangles.push(TriangleData {
                    vertices: [vertex1, vertex2, vertex3],
                    arm_index,
                    triangle_index: particle_index,
                    triangle_type: 0, // Type 0 = spiral particle
                    size: triangle_size,
                });
            }
        }
    }
    
    triangles
}

async fn generate_spiral_data(seed: u64, canvas_size: u64) -> Result<SpiralResponse, warp::Rejection> {
    let canvas_size = 420u64;
    // Use enhanced ZK circuit logic for galaxy generation
    let variant = seed % 41;
    let (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder) = 
        calculate_configuration_mapping(variant);
    
    // Generate galaxy-density particles (69 per arm instead of 15)
    let particles_per_arm = 69u64;
    let mut positions = Vec::new();
    let mut metadata = Vec::new();
    
    for arm_index in 0..num_arms {
        for particle_index in 0..particles_per_arm {
            let (x, y, angle_index) = calculate_galaxy_spiral_point_enhanced(
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
    
    // Generate micro-triangles using enhanced method
    let triangles = generate_enhanced_galaxy_triangles(&positions, num_arms);
    
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
    
    // NEW: Galaxy statistics
    let galaxy_type = match spiral_type {
        0 => "Tight Galaxy".to_string(),
        1 => "Loose Galaxy".to_string(),
        _ => "Classic Galaxy".to_string(),
    };
    
    let galaxy_stats = GalaxyStats {
        particles_per_arm,
        total_particles: num_arms * particles_per_arm,
        micro_triangles: triangles.len() as u64,
        galaxy_type,
        density_factor: particles_per_arm as f64 / 15.0, // Relative to original
    };
    
    Ok(SpiralResponse {
        seed,
        variant,
        spiral_type,
        num_arms,
        particles,
        triangles,
        config,
        galaxy_stats,
    })
}

// Gas estimation functions with enhanced galaxy complexity
async fn fetch_eth_gas_price() -> Result<f64> {
    let url = "https://api.etherscan.io/api?module=gastracker&action=gasoracle&apikey=MF1UH981PQBWJXHNWNQW6AAX3A3ERVGYGH";
    
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await?;
    
    if response.status().is_success() {
        let gas_data: EtherscanGasResponse = response.json().await?;
        let gas_price = gas_data.result.propose_gas_price.parse::<f64>()?;
        Ok(gas_price)
    } else {
        Ok(20.0) // Fallback
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
        Ok((3000.0, 45000.0)) // Fallback
    }
}

fn estimate_ethereum_gas_enhanced(spiral_data: &SpiralResponse) -> GasBreakdown {
    // Base costs
    let base_mint_gas = 50000u64;
    let zk_proof_gas = 120000u64; // Increased for enhanced circuit complexity
    
    // ENHANCED: Galaxy complexity gas calculation
    let metadata_size = calculate_galaxy_metadata_size(spiral_data);
    let metadata_gas = calculate_metadata_gas(metadata_size);
    
    // NEW: Additional gas for galaxy complexity
    let galaxy_complexity_gas = calculate_galaxy_complexity_gas(spiral_data);
    
    let total_gas = base_mint_gas + zk_proof_gas + metadata_gas + galaxy_complexity_gas;
    
    GasBreakdown {
        base_mint_gas,
        zk_proof_gas,
        metadata_gas,
        galaxy_complexity_gas,
        total_gas,
    }
}

fn calculate_galaxy_metadata_size(spiral_data: &SpiralResponse) -> u64 {
    let mut size = 0u64;
    
    // Base metadata
    size += 500;
    
    // Enhanced spiral configuration data
    size += 300; // More complex configuration
    
    // Micro-triangle coordinates (more triangles, but smaller individual size)
    size += (spiral_data.triangles.len() as u64) * 30; // Micro-triangles are more compact
    
    // Galaxy particle positions
    size += (spiral_data.particles.len() as u64) * 15;
    
    // Enhanced ZK proof data
    size += 500; // Larger proof for enhanced circuit
    
    // Galaxy-specific metadata
    size += 200;
    
    // Enhanced SVG/visual representation
    let visual_complexity = spiral_data.num_arms * spiral_data.galaxy_stats.particles_per_arm * 2;
    size += visual_complexity;
    
    size
}

fn calculate_galaxy_complexity_gas(spiral_data: &SpiralResponse) -> u64 {
    // Additional gas cost for galaxy-level complexity
    let triangle_count = spiral_data.triangles.len() as u64;
    let particle_count = spiral_data.particles.len() as u64;
    
    // Galaxy complexity factors
    let triangle_complexity = triangle_count * 800; // Per micro-triangle storage
    let particle_complexity = particle_count * 400; // Per particle computation
    let spiral_complexity = spiral_data.num_arms * 10000; // Per spiral arm complexity
    
    triangle_complexity + particle_complexity + spiral_complexity
}

fn calculate_metadata_gas(metadata_size_bytes: u64) -> u64 {
    let storage_slots = (metadata_size_bytes + 31) / 32;
    let base_storage_cost = storage_slots * 20000;
    let encoding_overhead = metadata_size_bytes * 60; // Higher for galaxy complexity
    let calldata_cost = metadata_size_bytes * 16;
    
    base_storage_cost + encoding_overhead + calldata_cost
}

async fn calculate_gas_costs_for_spiral(spiral_data: &SpiralResponse) -> Result<GasResponse, warp::Rejection> {
    let gas_price_gwei = match fetch_eth_gas_price().await {
        Ok(price) => price,
        Err(_) => 20.0,
    };
    
    let (eth_price_usd, btc_price_usd) = match fetch_crypto_prices().await {
        Ok(prices) => prices,
        Err(_) => (3000.0, 45000.0),
    };
    
    // Enhanced gas estimates for galaxy complexity
    let breakdown = estimate_ethereum_gas_enhanced(spiral_data);
    
    let gas_price_eth = gas_price_gwei / 1_000_000_000.0;
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
    let seed = request.seed.unwrap_or(12345u64);
    let canvas_size = request.canvas_size.unwrap_or(500u64);
    
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
    // API route for generating enhanced galaxy spirals
    let api_generate = warp::path("api")
        .and(warp::path("generate"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_generate_spiral);
    
    // API route for enhanced gas estimation
    let api_gas = warp::path("api")
        .and(warp::path("gas"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_gas_estimation);
    
    // Serve static files
    let static_files = warp::fs::dir("static");
    
    // Health check endpoint
    let health = warp::path("health")
        .map(|| "🌌 zkBg Galaxy Server Running!");
    
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
    
    println!("🌌 zkBg Galaxy Server starting on http://localhost:3030");
    println!("🎨 Enhanced Visualizer: http://localhost:3030/spiral_visualizer_hardhat.html");
    println!("🔧 Galaxy API: POST http://localhost:3030/api/generate");
    println!("⛽ Enhanced Gas API: POST http://localhost:3030/api/gas");
    println!("\n🚀 Phase 1: Galaxy Micro-Triangles");
    println!("   • 69 particles per arm (4.6x density increase)");
    println!("   • Micro-triangles for particle effect");
    println!("   • Enhanced galaxy spiral patterns");
    println!("   • ZK-verified galaxy generation");
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}