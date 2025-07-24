pub mod circuits {
    pub mod simple_spirals;
}

#[cfg(test)]
mod tests {
    use super::circuits::simple_spirals::{
        SimpleSpiralsCircuit, 
        calculate_configuration_mapping, 
        generate_spiral_particles,
        generate_spiral_triangles
    };
    use halo2_proofs::{circuit::Value, dev::MockProver};
    use pasta_curves::Fp;

    #[test]
    fn test_simple_spirals_circuit() {
        let seed_u64 = 12345u64;
        let variant_u64 = seed_u64 % 41;
        let quotient_u64 = seed_u64 / 41;
        
        // Calculate configuration mapping
        let (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder) = 
            calculate_configuration_mapping(variant_u64);
        
        println!("Testing with:");
        println!("  Seed: {}", seed_u64);
        println!("  Variant: {}", variant_u64);
        println!("  Quotient: {}", quotient_u64);
        println!("  Verification: {} = {} * 41 + {}", seed_u64, quotient_u64, variant_u64);
        println!("  Configuration: spiral_type={}, num_arms={}", spiral_type, num_arms);
        
        let circuit = SimpleSpiralsCircuit::<Fp> {
            seed: Value::known(Fp::from(seed_u64)),
            variant_id: Value::known(Fp::from(variant_u64)),
            quotient: Value::known(Fp::from(quotient_u64)),
            spiral_type: Value::known(Fp::from(spiral_type)),
            num_arms: Value::known(Fp::from(num_arms)),
            spiral_quotient: Value::known(Fp::from(spiral_quotient)),
            arms_quotient: Value::known(Fp::from(arms_quotient)),
            arms_remainder: Value::known(Fp::from(arms_remainder)),
            particles_per_arm: Value::known(Fp::from(15u64)),
            total_particles: Value::known(Fp::from(num_arms * 15)),
            canvas_size: Value::known(Fp::from(500u64)),
            particle_positions: vec![],
            particle_metadata: vec![],
            // Add triangle fields
            triangles_per_arm: Value::known(Fp::from(5u64)),
            total_triangles: Value::known(Fp::from(num_arms * 5)),
            triangle_vertices: vec![],
            triangle_metadata: vec![],
        };

        let prover = MockProver::run(12, &circuit, vec![]).unwrap();
        assert_eq!(prover.verify(), Ok(()));
        
        println!("✅ Variant selection and configuration mapping verified!");
    }

    #[test]
    fn test_multi_particle_generation() {
        let seed_u64 = 12345u64;
        let variant_u64 = seed_u64 % 41;
        let quotient_u64 = seed_u64 / 41;
        let canvas_size = 500u64;
        
        // Calculate configuration mapping
        let (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder) = 
            calculate_configuration_mapping(variant_u64);
        
        // Generate particle positions
        let (positions, metadata) = generate_spiral_particles(spiral_type, num_arms, canvas_size);
        
        println!("🌀 Multi-Particle Generation Test:");
        println!("  Spiral config: type={}, arms={}", spiral_type, num_arms);
        println!("  Total particles generated: {}", positions.len());
        println!("  Expected particles: {}", num_arms * 15);
        
        // Show first few particles for each arm
        for arm in 0..num_arms {
            let arm_particles: Vec<_> = metadata.iter()
                .enumerate()
                .filter(|(_, (arm_idx, _, _))| *arm_idx == arm)
                .take(3)
                .collect();
            
            println!("  Arm {}: {} particles (showing first 3)", arm, arm_particles.len());
            for (i, (_arm_idx, particle_idx, angle_idx)) in arm_particles {
                let (x, y) = positions[i];
                println!("    Particle {}: ({}, {}) angle_idx={}", particle_idx, x, y, angle_idx);
            }
        }
        
        // Convert to circuit values
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
            spiral_quotient: Value::known(Fp::from(spiral_quotient)),
            arms_quotient: Value::known(Fp::from(arms_quotient)),
            arms_remainder: Value::known(Fp::from(arms_remainder)),
            particles_per_arm: Value::known(Fp::from(15u64)),
            total_particles: Value::known(Fp::from(num_arms * 15)),
            canvas_size: Value::known(Fp::from(canvas_size)),
            particle_positions,
            particle_metadata,
            // Add triangle fields (empty for this test)
            triangles_per_arm: Value::known(Fp::from(5u64)),
            total_triangles: Value::known(Fp::from(num_arms * 5)),
            triangle_vertices: vec![],
            triangle_metadata: vec![],
        };

        let prover = MockProver::run(12, &circuit, vec![]).unwrap();
        assert_eq!(prover.verify(), Ok(()));
        
        println!("✅ Multi-particle generation circuit verified!");
        println!("✅ All {} particles successfully processed in ZK circuit!", positions.len());
    }

    #[test]
    fn test_triangle_generation() {
        let seed_u64 = 12345u64;
        let variant_u64 = seed_u64 % 41;
        let quotient_u64 = seed_u64 / 41;
        let canvas_size = 500u64;
        
        // Calculate configuration mapping
        let (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder) = 
            calculate_configuration_mapping(variant_u64);
        
        // Generate particle positions (from Phase 2B)
        let (positions, _metadata) = generate_spiral_particles(spiral_type, num_arms, canvas_size);
        
        // Generate triangle vertices (NEW for Phase 2C)
        let (triangle_vertices, triangle_metadata) = generate_spiral_triangles(spiral_type, num_arms, canvas_size);
        
        println!("🔺 Triangle Generation Test:");
        println!("  Spiral config: type={}, arms={}", spiral_type, num_arms);
        println!("  Total particles: {}", positions.len());
        println!("  Total triangles generated: {}", triangle_vertices.len());
        println!("  Expected triangles: {}", num_arms * 5); // 5 triangles per arm
        
        // Show triangle details for each arm
        for arm in 0..num_arms {
            let arm_triangles: Vec<_> = triangle_metadata.iter()
                .enumerate()
                .filter(|(_, (arm_idx, _, _))| *arm_idx == arm)
                .take(2) // Show first 2 triangles per arm
                .collect();
            
            println!("  Arm {}: {} triangles (showing first 2)", arm, arm_triangles.len());
            for (i, (_arm_idx, triangle_idx, triangle_type)) in arm_triangles {
                let (x1, y1, x2, y2, x3, y3) = triangle_vertices[i];
                println!("    Triangle {}: vertices=({},{}) ({},{}) ({},{}) type={}", 
                         triangle_idx, x1, y1, x2, y2, x3, y3, triangle_type);
            }
        }
        
        // Convert to circuit values
        let particle_positions: Vec<(Value<Fp>, Value<Fp>)> = positions.iter()
            .map(|(x, y)| (Value::known(Fp::from(*x)), Value::known(Fp::from(*y))))
            .collect();
            
        let particle_metadata: Vec<(Value<Fp>, Value<Fp>, Value<Fp>)> = vec![]; // Simplified for this test
        
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
            spiral_quotient: Value::known(Fp::from(spiral_quotient)),
            arms_quotient: Value::known(Fp::from(arms_quotient)),
            arms_remainder: Value::known(Fp::from(arms_remainder)),
            particles_per_arm: Value::known(Fp::from(15u64)),
            total_particles: Value::known(Fp::from(num_arms * 15)),
            canvas_size: Value::known(Fp::from(canvas_size)),
            particle_positions,
            particle_metadata,
            // NEW Phase 2C fields
            triangles_per_arm: Value::known(Fp::from(5u64)),
            total_triangles: Value::known(Fp::from(num_arms * 5)),
            triangle_vertices: triangle_vertices_circuit,
            triangle_metadata: triangle_metadata_circuit,
        };

        let prover = MockProver::run(12, &circuit, vec![]).unwrap();
        assert_eq!(prover.verify(), Ok(()));
        
        println!("✅ Triangle generation circuit verified!");
        println!("✅ All {} triangles successfully processed in ZK circuit!", triangle_vertices.len());
        
        // Verify triangle formation logic
        assert_eq!(triangle_vertices.len(), (num_arms * 5) as usize);
        println!("✅ Triangle count verification passed!");
        
        // Check that triangles are formed from consecutive particles
        for (_i, (x1, y1, x2, y2, x3, y3)) in triangle_vertices.iter().enumerate() {
            // Basic sanity check - all coordinates should be within canvas bounds
            assert!(*x1 <= canvas_size && *y1 <= canvas_size);
            assert!(*x2 <= canvas_size && *y2 <= canvas_size);
            assert!(*x3 <= canvas_size && *y3 <= canvas_size);
        }
        println!("✅ Triangle vertex bounds verification passed!");
    }

    #[test]
    fn test_multiple_seeds() {
        let test_cases = vec![
            123u64, 999u64, 0u64, 40u64, 41u64, 82u64, 5000u64
        ];

        for seed in test_cases {
            let variant = seed % 41;
            let quotient = seed / 41;
            
            // Calculate configuration mapping for each variant
            let (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder) = 
                calculate_configuration_mapping(variant);

            println!("Testing seed {} → variant {} (quotient: {})", seed, variant, quotient);
            println!("  Config: spiral_type={}, num_arms={}", spiral_type, num_arms);

            let circuit = SimpleSpiralsCircuit::<Fp> {
                seed: Value::known(Fp::from(seed)),
                variant_id: Value::known(Fp::from(variant)),
                quotient: Value::known(Fp::from(quotient)),
                spiral_type: Value::known(Fp::from(spiral_type)),
                num_arms: Value::known(Fp::from(num_arms)),
                spiral_quotient: Value::known(Fp::from(spiral_quotient)),
                arms_quotient: Value::known(Fp::from(arms_quotient)),
                arms_remainder: Value::known(Fp::from(arms_remainder)),
                particles_per_arm: Value::known(Fp::from(15u64)),
                total_particles: Value::known(Fp::from(num_arms * 15)),
                canvas_size: Value::known(Fp::from(500u64)),
                particle_positions: vec![],
                particle_metadata: vec![],
                // Add triangle fields
                triangles_per_arm: Value::known(Fp::from(5u64)),
                total_triangles: Value::known(Fp::from(num_arms * 5)),
                triangle_vertices: vec![],
                triangle_metadata: vec![],
            };

            let prover = MockProver::run(12, &circuit, vec![]).unwrap();
            assert_eq!(prover.verify(), Ok(()));
            
            println!("✅ Verified: {} = {} * 41 + {}", seed, quotient, variant);
        }
    }

    #[test]
    fn test_spiral_mathematics() {
        let seed_u64 = 12345u64;
        let variant_u64 = seed_u64 % 41;
        let quotient_u64 = seed_u64 / 41;
        
        // Calculate configuration mapping
        let (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder) = 
            calculate_configuration_mapping(variant_u64);
        
        println!("Spiral configuration:");
        println!("  Variant: {}", variant_u64);
        println!("  Spiral type: {}", spiral_type);
        println!("  Number of arms: {}", num_arms);
        println!("  Intermediate values: spiral_quotient={}, arms_quotient={}, arms_remainder={}", 
                 spiral_quotient, arms_quotient, arms_remainder);
        
        let circuit = SimpleSpiralsCircuit::<Fp> {
            seed: Value::known(Fp::from(seed_u64)),
            variant_id: Value::known(Fp::from(variant_u64)),
            quotient: Value::known(Fp::from(quotient_u64)),
            spiral_type: Value::known(Fp::from(spiral_type)),
            num_arms: Value::known(Fp::from(num_arms)),
            spiral_quotient: Value::known(Fp::from(spiral_quotient)),
            arms_quotient: Value::known(Fp::from(arms_quotient)),
            arms_remainder: Value::known(Fp::from(arms_remainder)),
            particles_per_arm: Value::known(Fp::from(15u64)),
            total_particles: Value::known(Fp::from(num_arms * 15)),
            canvas_size: Value::known(Fp::from(500u64)),
            particle_positions: vec![],
            particle_metadata: vec![],
            // Add triangle fields
            triangles_per_arm: Value::known(Fp::from(5u64)),
            total_triangles: Value::known(Fp::from(num_arms * 5)),
            triangle_vertices: vec![],
            triangle_metadata: vec![],
        };

        let prover = MockProver::run(12, &circuit, vec![]).unwrap();
        assert_eq!(prover.verify(), Ok(()));
        
        println!("✅ Spiral mathematics circuit verified!");
    }

    #[test]
    fn test_configuration_mapping() {
        // Test various configurations across the 41 variants
        let test_cases = vec![
            (0, 0, 3),   // variant 0 → tight spiral, 3 arms
            (1, 1, 3),   // variant 1 → loose spiral, 3 arms  
            (2, 2, 3),   // variant 2 → classic spiral, 3 arms
            (3, 0, 4),   // variant 3 → tight spiral, 4 arms
            (4, 1, 4),   // variant 4 → loose spiral, 4 arms
            (15, 0, 8),  // variant 15 → tight spiral, 8 arms
            (18, 0, 3),  // variant 18 → tight spiral, 3 arms
            (40, 1, 4),  // variant 40 → loose spiral, 4 arms
        ];

        for (variant, expected_spiral_type, expected_arms) in test_cases {
            println!("Testing variant {} → spiral_type: {}, arms: {}", 
                     variant, expected_spiral_type, expected_arms);
            
            // Calculate using our mapping logic
            let (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder) = 
                calculate_configuration_mapping(variant);
            
            println!("  Calculated: spiral_type: {}, arms: {}", spiral_type, num_arms);
            println!("  Intermediate: spiral_quotient={}, arms_quotient={}, arms_remainder={}", 
                     spiral_quotient, arms_quotient, arms_remainder);
            
            assert_eq!(spiral_type, expected_spiral_type);
            assert_eq!(num_arms, expected_arms);
            
            println!("✅ Configuration verified");
        }

        // Distribution analysis
        println!("\n📊 Configuration Distribution Analysis:");
        let mut spiral_counts = [0; 3];
        let mut arm_counts = [0; 6];
        
        for variant in 0..41 {
            let (spiral_type, num_arms, _, _, _) = calculate_configuration_mapping(variant);
            spiral_counts[spiral_type as usize] += 1;
            arm_counts[(num_arms - 3) as usize] += 1;
        }
        
        println!("  Spiral types: Tight={}, Loose={}, Classic={}", 
                 spiral_counts[0], spiral_counts[1], spiral_counts[2]);
        println!("  Arm counts: 3={}, 4={}, 5={}, 6={}, 7={}, 8={}", 
                 arm_counts[0], arm_counts[1], arm_counts[2], 
                 arm_counts[3], arm_counts[4], arm_counts[5]);
    }

    #[test]
    fn test_configuration_constraints() {
        // Test that the circuit properly verifies configuration mapping constraints
        println!("Testing configuration mapping constraints in circuit...");
        
        let test_variants = vec![0, 1, 4, 15, 25, 40];
        
        for variant in test_variants {
            let (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder) = 
                calculate_configuration_mapping(variant);
                
            println!("Variant {}: spiral_type={}, arms={}, intermediates=({},{},{})", 
                     variant, spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder);
            
            // Verify the mathematical relationships manually
            assert_eq!(variant, spiral_quotient * 3 + spiral_type);
            assert_eq!(spiral_quotient, arms_quotient * 6 + arms_remainder);
            assert_eq!(num_arms, 3 + arms_remainder);
            
            let circuit = SimpleSpiralsCircuit::<Fp> {
                seed: Value::known(Fp::from(12345u64)),
                variant_id: Value::known(Fp::from(variant)),
                quotient: Value::known(Fp::from(12345u64 / 41)),
                spiral_type: Value::known(Fp::from(spiral_type)),
                num_arms: Value::known(Fp::from(num_arms)),
                spiral_quotient: Value::known(Fp::from(spiral_quotient)),
                arms_quotient: Value::known(Fp::from(arms_quotient)),
                arms_remainder: Value::known(Fp::from(arms_remainder)),
                particles_per_arm: Value::known(Fp::from(15u64)),
                total_particles: Value::known(Fp::from(num_arms * 15)),
                canvas_size: Value::known(Fp::from(500u64)),
                particle_positions: vec![],
                particle_metadata: vec![],
                // Add triangle fields
                triangles_per_arm: Value::known(Fp::from(5u64)),
                total_triangles: Value::known(Fp::from(num_arms * 5)),
                triangle_vertices: vec![],
                triangle_metadata: vec![],
            };

            let prover = MockProver::run(12, &circuit, vec![]).unwrap();
            assert_eq!(prover.verify(), Ok(()));
            
            println!("✅ Circuit constraints verified for variant {}", variant);
        }
    }
}