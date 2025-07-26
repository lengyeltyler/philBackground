pub mod circuits {
    pub mod simple_spirals;
}

#[cfg(test)]
mod tests {
    use super::circuits::simple_spirals::{
        SimpleSpiralsCircuit, 
        calculate_configuration_mapping, 
        generate_spiral_particles,
        generate_spiral_triangles,
        generate_background_type
    };
    use halo2_proofs::{circuit::Value, dev::MockProver};
    use pasta_curves::Fp;

    #[test]
    fn test_optimized_spirals_circuit() {
        let seed_u64 = 12345u64;
        let variant_u64 = seed_u64 % 41;
        let quotient_u64 = seed_u64 / 41;
        
        let (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder) = 
            calculate_configuration_mapping(variant_u64);
        
        let background_type = generate_background_type(seed_u64);
        
        println!("🌌 Testing OPTIMIZED Galaxy Spiral Circuit:");
        println!("  Seed: {}", seed_u64);
        println!("  Variant: {}", variant_u64);
        println!("  Config: spiral_type={}, num_arms={}", spiral_type, num_arms);
        println!("  Background: {}", background_type);
        
        let circuit = SimpleSpiralsCircuit::<Fp> {
            seed: Value::known(Fp::from(seed_u64)),
            variant_id: Value::known(Fp::from(variant_u64)),
            quotient: Value::known(Fp::from(quotient_u64)),
            spiral_type: Value::known(Fp::from(spiral_type)),
            num_arms: Value::known(Fp::from(num_arms)),
            background_type: Value::known(Fp::from(background_type)),
            spiral_quotient: Value::known(Fp::from(spiral_quotient)),
            arms_quotient: Value::known(Fp::from(arms_quotient)),
            arms_remainder: Value::known(Fp::from(arms_remainder)),
            particles_per_arm: Value::known(Fp::from(23u64)), // OPTIMIZED: 23 instead of 69
            total_particles: Value::known(Fp::from(num_arms * 23)),
            canvas_size: Value::known(Fp::from(420u64)), // Optimized canvas size
            particle_positions: vec![],
            particle_metadata: vec![],
            triangles_per_arm: Value::known(Fp::from(23u64)), // OPTIMIZED: 23 micro-triangles
            total_triangles: Value::known(Fp::from(num_arms * 23)),
            triangle_vertices: vec![],
            triangle_metadata: vec![],
        };

        let prover = MockProver::run(12, &circuit, vec![]).unwrap();
        assert_eq!(prover.verify(), Ok(()));
        
        println!("✅ OPTIMIZED galaxy configuration verified!");
        println!("   Expected particles: {} (reduced from {})", num_arms * 23, num_arms * 69);
        println!("   Expected triangles: {} (reduced from {})", num_arms * 23, num_arms * 69);
        println!("   Gas savings: ~10.4M gas (estimated)");
    }

    #[test]
    fn test_optimized_particle_generation() {
        let seed_u64 = 12345u64;
        let variant_u64 = seed_u64 % 41;
        let quotient_u64 = seed_u64 / 41;
        let canvas_size = 420u64; // Optimized canvas size
        
        let (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder) = 
            calculate_configuration_mapping(variant_u64);
        
        // Generate optimized particles (23 per arm)
        let (positions, metadata) = generate_spiral_particles(spiral_type, num_arms, canvas_size);
        
        println!("🌌 OPTIMIZED Particle Generation Test:");
        println!("  Spiral config: type={}, arms={}", spiral_type, num_arms);
        println!("  Total particles generated: {}", positions.len());
        println!("  Expected particles: {}", num_arms * 23);
        println!("  Reduction from previous: {} particles saved", num_arms * (69 - 23));
        assert_eq!(positions.len(), (num_arms * 23) as usize);
        
        // Verify optimized distribution
        for arm in 0..num_arms {
            let arm_particles: Vec<_> = metadata.iter()
                .enumerate()
                .filter(|(_, (arm_idx, _, _))| *arm_idx == arm)
                .take(5) // Show first 5 particles per arm
                .collect();
            
            println!("  Arm {}: {} particles (showing first 5)", arm, arm_particles.len());
            for (i, (_arm_idx, particle_idx, angle_idx)) in arm_particles {
                let (x, y) = positions[i];
                println!("    Particle {}: ({}, {}) angle_idx={}", particle_idx, x, y, angle_idx);
            }
        }
        
        // Convert to circuit values for testing
        let particle_positions: Vec<(Value<Fp>, Value<Fp>)> = positions.iter()
            .map(|(x, y)| (Value::known(Fp::from(*x)), Value::known(Fp::from(*y))))
            .collect();
            
        let particle_metadata: Vec<(Value<Fp>, Value<Fp>, Value<Fp>)> = metadata.iter()
            .map(|(arm_idx, particle_idx, angle_idx)| (
                Value::known(Fp::from(*arm_idx)),
                Value::known(Fp::from(*particle_idx)), 
                Value::known(Fp::from(*angle_idx))
            ))
            .collect();

        let circuit = SimpleSpiralsCircuit::<Fp> {
            seed: Value::known(Fp::from(seed_u64)),
            variant_id: Value::known(Fp::from(variant_u64)),
            quotient: Value::known(Fp::from(quotient_u64)),
            spiral_type: Value::known(Fp::from(spiral_type)),
            num_arms: Value::known(Fp::from(num_arms)),
            background_type: Value::known(Fp::from(generate_background_type(seed_u64))),
            spiral_quotient: Value::known(Fp::from(spiral_quotient)),
            arms_quotient: Value::known(Fp::from(arms_quotient)),
            arms_remainder: Value::known(Fp::from(arms_remainder)),
            particles_per_arm: Value::known(Fp::from(23u64)),
            total_particles: Value::known(Fp::from(num_arms * 23)),
            canvas_size: Value::known(Fp::from(canvas_size)),
            particle_positions,
            particle_metadata,
            triangles_per_arm: Value::known(Fp::from(23u64)),
            total_triangles: Value::known(Fp::from(num_arms * 23)),
            triangle_vertices: vec![],
            triangle_metadata: vec![],
        };

        let prover = MockProver::run(12, &circuit, vec![]).unwrap();
        assert_eq!(prover.verify(), Ok(()));
        
        println!("✅ OPTIMIZED particle generation verified!");
        println!("✅ All {} particles successfully processed!", positions.len());
    }

    #[test]
    fn test_optimized_triangle_generation() {
        let seed_u64 = 12345u64;
        let variant_u64 = seed_u64 % 41;
        let quotient_u64 = seed_u64 / 41;
        let canvas_size = 420u64; // Optimized canvas size
        
        let (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder) = 
            calculate_configuration_mapping(variant_u64);
        
        // Generate particles and optimized micro-triangles
        let (positions, _metadata) = generate_spiral_particles(spiral_type, num_arms, canvas_size);
        let (triangle_vertices, triangle_metadata) = generate_spiral_triangles(spiral_type, num_arms, canvas_size);
        
        println!("🔺 OPTIMIZED Micro-Triangle Generation Test:");
        println!("  Spiral config: type={}, arms={}", spiral_type, num_arms);
        println!("  Total particles: {}", positions.len());
        println!("  Total micro-triangles generated: {}", triangle_vertices.len());
        println!("  Expected triangles: {}", num_arms * 23);
        println!("  Reduction from previous: {} triangles saved", num_arms * (69 - 23));
        assert_eq!(triangle_vertices.len(), (num_arms * 23) as usize);
        
        // Verify optimized micro-triangle properties
        for arm in 0..num_arms {
            let arm_triangles: Vec<_> = triangle_metadata.iter()
                .enumerate()
                .filter(|(_, (arm_idx, _, _))| *arm_idx == arm)
                .take(3) // Show first 3 triangles per arm
                .collect();
            
            println!("  Arm {}: {} triangles (showing first 3)", arm, arm_triangles.len());
            for (i, (_arm_idx, triangle_idx, triangle_type)) in arm_triangles {
                let (x1, y1, x2, y2, x3, y3) = triangle_vertices[i];
                
                // Verify these are micro-triangles (small size)
                let width = x3.max(x1).max(x2) - x1.min(x2).min(x3);
                let height = y2.max(y1).max(y3) - y1.min(y2).min(y3);
                
                println!("    Triangle {}: center≈({},{}) size≈{}x{} type={}", 
                         triangle_idx, 
                         (x1 + x2 + x3) / 3, 
                         (y1 + y2 + y3) / 3,
                         width, height,
                         triangle_type);
                
                // Verify micro-triangle size constraints (should be small)
                assert!(width <= 12, "Triangle too wide: {}", width);
                assert!(height <= 12, "Triangle too tall: {}", height);
            }
        }
        
        // Convert to circuit values
        let particle_positions: Vec<(Value<Fp>, Value<Fp>)> = positions.iter()
            .map(|(x, y)| (Value::known(Fp::from(*x)), Value::known(Fp::from(*y))))
            .collect();
            
        let particle_metadata: Vec<(Value<Fp>, Value<Fp>, Value<Fp>)> = vec![];
        
        let triangle_vertices_circuit: Vec<(Value<Fp>, Value<Fp>, Value<Fp>, Value<Fp>, Value<Fp>, Value<Fp>)> = 
            triangle_vertices.iter()
            .map(|(x1, y1, x2, y2, x3, y3)| (
                Value::known(Fp::from(*x1)),
                Value::known(Fp::from(*y1)),
                Value::known(Fp::from(*x2)),
                Value::known(Fp::from(*y2)),
                Value::known(Fp::from(*x3)),
                Value::known(Fp::from(*y3))
            ))
            .collect();
            
        let triangle_metadata_circuit: Vec<(Value<Fp>, Value<Fp>, Value<Fp>)> = 
            triangle_metadata.iter()
            .map(|(arm_idx, triangle_idx, triangle_type)| (
                Value::known(Fp::from(*arm_idx)),
                Value::known(Fp::from(*triangle_idx)), 
                Value::known(Fp::from(*triangle_type))
            ))
            .collect();

        let circuit = SimpleSpiralsCircuit::<Fp> {
            seed: Value::known(Fp::from(seed_u64)),
            variant_id: Value::known(Fp::from(variant_u64)),
            quotient: Value::known(Fp::from(quotient_u64)),
            spiral_type: Value::known(Fp::from(spiral_type)),
            num_arms: Value::known(Fp::from(num_arms)),
            background_type: Value::known(Fp::from(generate_background_type(seed_u64))),
            spiral_quotient: Value::known(Fp::from(spiral_quotient)),
            arms_quotient: Value::known(Fp::from(arms_quotient)),
            arms_remainder: Value::known(Fp::from(arms_remainder)),
            particles_per_arm: Value::known(Fp::from(23u64)),
            total_particles: Value::known(Fp::from(num_arms * 23)),
            canvas_size: Value::known(Fp::from(canvas_size)),
            particle_positions,
            particle_metadata,
            triangles_per_arm: Value::known(Fp::from(23u64)),
            total_triangles: Value::known(Fp::from(num_arms * 23)),
            triangle_vertices: triangle_vertices_circuit,
            triangle_metadata: triangle_metadata_circuit,
        };

        let prover = MockProver::run(12, &circuit, vec![]).unwrap();
        assert_eq!(prover.verify(), Ok(()));
        
        println!("✅ OPTIMIZED micro-triangle generation verified!");
        println!("✅ All {} micro-triangles successfully processed!", triangle_vertices.len());
        println!("✅ Gas-optimized galaxy particle effect ready!");
    }

    #[test]
    fn test_background_type_generation() {
        println!("🎨 Testing Background Type Generation:");
        
        let test_seeds = vec![12345u64, 67890u64, 111u64, 999u64, 5555u64];
        let background_names = ["Deep Space", "Nebula", "Starfield", "Cosmic", "Void"];
        
        for seed in test_seeds {
            let bg_type = generate_background_type(seed);
            println!("  Seed {}: Background {} ({})", seed, bg_type, background_names[bg_type as usize]);
            assert!(bg_type < 5, "Background type should be 0-4");
        }
        
        // Test deterministic behavior
        let seed = 12345u64;
        let bg1 = generate_background_type(seed);
        let bg2 = generate_background_type(seed);
        assert_eq!(bg1, bg2, "Background generation should be deterministic");
        
        println!("✅ Background type generation verified!");
    }

    #[test]
    fn test_optimized_visual_distribution() {
        // Test different spiral types for optimized variety
        let test_cases = vec![
            (0, "Tight Galaxy (Optimized)"),
            (1, "Loose Galaxy (Optimized)"), 
            (2, "Classic Galaxy (Optimized)"),
        ];

        for (spiral_type, name) in test_cases {
            println!("\n🌌 Testing {}", name);
            
            let num_arms = 6u64;
            let canvas_size = 420u64; // Optimized canvas
            
            let (positions, _metadata) = generate_spiral_particles(spiral_type, num_arms, canvas_size);
            let (triangles, _tri_metadata) = generate_spiral_triangles(spiral_type, num_arms, canvas_size);
            
            // Verify optimized properties
            println!("  Total particles: {}", positions.len());
            println!("  Total micro-triangles: {}", triangles.len());
            println!("  Particles per arm: {}", positions.len() / num_arms as usize);
            
            // Check distribution across optimized canvas
            let center_x = canvas_size / 2;
            let center_y = canvas_size / 2;
            
            let mut radial_counts = vec![0; 5]; // 5 radial zones
            for (x, y) in &positions {
                let dx = (*x as i64 - center_x as i64).abs() as u64;
                let dy = (*y as i64 - center_y as i64).abs() as u64;
                let distance_sq = dx * dx + dy * dy;
                let distance = (distance_sq as f64).sqrt() as u64;
                
                let zone = ((distance * 5) / (canvas_size / 2)).min(4) as usize;
                radial_counts[zone] += 1;
            }
            
            println!("  Radial distribution: {:?}", radial_counts);
            
            // Verify galaxy has particles distributed properly
            assert!(radial_counts[0] > 0, "Should have core particles");
            assert!(radial_counts[4] > 0, "Should have edge particles");
            
            // Check optimization: should have exactly 23 particles per arm
            assert_eq!(positions.len(), (num_arms * 23) as usize, "Should have exactly 23 particles per arm");
            assert_eq!(triangles.len(), (num_arms * 23) as usize, "Should have exactly 23 triangles per arm");
            
            println!("  ✅ {} distribution verified", name);
        }
    }

    #[test]
    fn test_multiple_seeds_optimized() {
        let test_cases = vec![
            123u64, 999u64, 0u64, 40u64, 41u64, 82u64, 5000u64
        ];

        for seed in test_cases {
            let variant = seed % 41;
            let quotient = seed / 41;
            
            let (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder) = 
                calculate_configuration_mapping(variant);
            
            let background_type = generate_background_type(seed);

            println!("Testing OPTIMIZED Seed {} → variant {} (quotient: {})", seed, variant, quotient);
            println!("  Config: spiral_type={}, num_arms={}, background={}", spiral_type, num_arms, background_type);

            let circuit = SimpleSpiralsCircuit::<Fp> {
                seed: Value::known(Fp::from(seed)),
                variant_id: Value::known(Fp::from(variant)),
                quotient: Value::known(Fp::from(quotient)),
                spiral_type: Value::known(Fp::from(spiral_type)),
                num_arms: Value::known(Fp::from(num_arms)),
                background_type: Value::known(Fp::from(background_type)),
                spiral_quotient: Value::known(Fp::from(spiral_quotient)),
                arms_quotient: Value::known(Fp::from(arms_quotient)),
                arms_remainder: Value::known(Fp::from(arms_remainder)),
                particles_per_arm: Value::known(Fp::from(23u64)), // OPTIMIZED
                total_particles: Value::known(Fp::from(num_arms * 23)),
                canvas_size: Value::known(Fp::from(420u64)), // Optimized canvas
                particle_positions: vec![],
                particle_metadata: vec![],
                triangles_per_arm: Value::known(Fp::from(23u64)), // OPTIMIZED
                total_triangles: Value::known(Fp::from(num_arms * 23)),
                triangle_vertices: vec![],
                triangle_metadata: vec![],
            };

            let prover = MockProver::run(12, &circuit, vec![]).unwrap();
            assert_eq!(prover.verify(), Ok(()));
            
            println!("✅ OPTIMIZED Galaxy Verified: {} = {} * 41 + {}", seed, quotient, variant);
        }
    }

    #[test]
    fn test_optimized_complexity_limits() {
        // Test circuit capacity with optimized complexity
        let seed_u64 = 12345u64;
        let variant_u64 = seed_u64 % 41;
        let quotient_u64 = seed_u64 / 41;
        
        let (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder) = 
            calculate_configuration_mapping(variant_u64);
        
        println!("🔬 Testing OPTIMIZED Complexity Limits:");
        println!("  Expected total triangles: {}", num_arms * 23);
        println!("  Circuit complexity: OPTIMIZED (reduced from 69 to 23)");
        
        // Generate optimized data
        let (positions, metadata) = generate_spiral_particles(spiral_type, num_arms, 420);
        let (triangles, tri_metadata) = generate_spiral_triangles(spiral_type, num_arms, 420);
        
        println!("  Generated {} positions", positions.len());
        println!("  Generated {} triangles", triangles.len());
        
        // Test with substantial but optimized data
        let limited_positions: Vec<(Value<Fp>, Value<Fp>)> = positions.iter()
            .take(30) // Reduced test size for optimized circuit
            .map(|(x, y)| (Value::known(Fp::from(*x)), Value::known(Fp::from(*y))))
            .collect();
            
        let limited_metadata: Vec<(Value<Fp>, Value<Fp>, Value<Fp>)> = metadata.iter()
            .take(30)
            .map(|(arm_idx, particle_idx, angle_idx)| (
                Value::known(Fp::from(*arm_idx)),
                Value::known(Fp::from(*particle_idx)), 
                Value::known(Fp::from(*angle_idx))
            ))
            .collect();
            
        let limited_triangles: Vec<(Value<Fp>, Value<Fp>, Value<Fp>, Value<Fp>, Value<Fp>, Value<Fp>)> = 
            triangles.iter()
            .take(30)
            .map(|(x1, y1, x2, y2, x3, y3)| (
                Value::known(Fp::from(*x1)),
                Value::known(Fp::from(*y1)),
                Value::known(Fp::from(*x2)),
                Value::known(Fp::from(*y2)),
                Value::known(Fp::from(*x3)),
                Value::known(Fp::from(*y3))
            ))
            .collect();

        let circuit = SimpleSpiralsCircuit::<Fp> {
            seed: Value::known(Fp::from(seed_u64)),
            variant_id: Value::known(Fp::from(variant_u64)),
            quotient: Value::known(Fp::from(quotient_u64)),
            spiral_type: Value::known(Fp::from(spiral_type)),
            num_arms: Value::known(Fp::from(num_arms)),
            background_type: Value::known(Fp::from(generate_background_type(seed_u64))),
            spiral_quotient: Value::known(Fp::from(spiral_quotient)),
            arms_quotient: Value::known(Fp::from(arms_quotient)),
            arms_remainder: Value::known(Fp::from(arms_remainder)),
            particles_per_arm: Value::known(Fp::from(23u64)),
            total_particles: Value::known(Fp::from(num_arms * 23)),
            canvas_size: Value::known(Fp::from(420u64)),
            particle_positions: limited_positions,
            particle_metadata: limited_metadata,
            triangles_per_arm: Value::known(Fp::from(23u64)),
            total_triangles: Value::known(Fp::from(num_arms * 23)),
            triangle_vertices: limited_triangles,
            triangle_metadata: vec![],
        };

        let prover = MockProver::run(12, &circuit, vec![]).unwrap();
        assert_eq!(prover.verify(), Ok(()));
        
        println!("✅ OPTIMIZED complexity test passed!");
        println!("✅ Circuit ready for optimized galaxy generation!");
        println!("✅ Estimated gas savings: ~10.4M gas vs previous version!");
    }
}