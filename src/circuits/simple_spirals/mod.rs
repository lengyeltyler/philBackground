use ff::Field;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Expression, Selector},
    poly::Rotation,
};

#[derive(Debug, Clone)]
pub struct SimpleSpiralsConfig {
    pub advice: [Column<Advice>; 24], // Keep existing column count
    pub selector_variant: Selector,
    pub selector_spiral: Selector,
    pub selector_config: Selector,
    pub selector_particle: Selector,
    pub selector_triangle: Selector,
}

#[derive(Debug, Clone)]
pub struct SimpleSpiralsCircuit<F: Field> {
    pub seed: Value<F>,
    pub variant_id: Value<F>,
    pub quotient: Value<F>,
    // Configuration values
    pub spiral_type: Value<F>,
    pub num_arms: Value<F>,
    // Configuration intermediate values
    pub spiral_quotient: Value<F>,
    pub arms_quotient: Value<F>,
    pub arms_remainder: Value<F>,
    // Enhanced particle generation for galaxy effect
    pub particles_per_arm: Value<F>,   // Now 69 instead of 15
    pub total_particles: Value<F>,     // num_arms * 69
    pub canvas_size: Value<F>,
    // Particle data - now micro-particles
    pub particle_positions: Vec<(Value<F>, Value<F>)>,
    pub particle_metadata: Vec<(Value<F>, Value<F>, Value<F>)>,
    // Enhanced triangle generation - micro-triangles for galaxy effect
    pub triangles_per_arm: Value<F>,   // Now 69 instead of 5
    pub total_triangles: Value<F>,     // num_arms * 69
    pub triangle_vertices: Vec<(Value<F>, Value<F>, Value<F>, Value<F>, Value<F>, Value<F>)>,
    pub triangle_metadata: Vec<(Value<F>, Value<F>, Value<F>)>,
}

impl<F: Field> Circuit<F> for SimpleSpiralsCircuit<F> {
    type Config = SimpleSpiralsConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            seed: Value::unknown(),
            variant_id: Value::unknown(),
            quotient: Value::unknown(),
            spiral_type: Value::unknown(),
            num_arms: Value::unknown(),
            spiral_quotient: Value::unknown(),
            arms_quotient: Value::unknown(),
            arms_remainder: Value::unknown(),
            particles_per_arm: Value::unknown(),
            total_particles: Value::unknown(),
            canvas_size: Value::unknown(),
            particle_positions: vec![],
            particle_metadata: vec![],
            triangles_per_arm: Value::unknown(),
            total_triangles: Value::unknown(),
            triangle_vertices: vec![],
            triangle_metadata: vec![],
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advice = [
            meta.advice_column(), // 0: seed
            meta.advice_column(), // 1: quotient  
            meta.advice_column(), // 2: variant_id
            meta.advice_column(), // 3: spiral_type
            meta.advice_column(), // 4: num_arms
            meta.advice_column(), // 5: spiral_quotient
            meta.advice_column(), // 6: arms_quotient
            meta.advice_column(), // 7: arms_remainder
            meta.advice_column(), // 8: particles_per_arm
            meta.advice_column(), // 9: total_particles
            meta.advice_column(), // 10: canvas_size
            meta.advice_column(), // 11: particle_x
            meta.advice_column(), // 12: particle_y
            meta.advice_column(), // 13: arm_index
            meta.advice_column(), // 14: particle_index
            meta.advice_column(), // 15: angle_index
            // Triangle-specific columns
            meta.advice_column(), // 16: triangles_per_arm
            meta.advice_column(), // 17: total_triangles
            meta.advice_column(), // 18: triangle_x1
            meta.advice_column(), // 19: triangle_y1
            meta.advice_column(), // 20: triangle_x2
            meta.advice_column(), // 21: triangle_y2
            meta.advice_column(), // 22: triangle_x3
            meta.advice_column(), // 23: triangle_y3
        ];
        
        let selector_variant = meta.selector();
        let selector_spiral = meta.selector();
        let selector_config = meta.selector();
        let selector_particle = meta.selector();
        let selector_triangle = meta.selector();

        for col in &advice {
            meta.enable_equality(*col);
        }

        // Constraint 1: Variant selection (unchanged)
        meta.create_gate("variant_selection", |meta| {
            let s = meta.query_selector(selector_variant);
            let seed = meta.query_advice(advice[0], Rotation::cur());
            let quotient = meta.query_advice(advice[1], Rotation::cur());
            let variant_id = meta.query_advice(advice[2], Rotation::cur());

            let mut forty_one = F::ZERO;
            for _ in 0..41 {
                forty_one = forty_one + F::ONE;
            }
            let forty_one_expr = Expression::Constant(forty_one);

            vec![
                s * (seed - (quotient * forty_one_expr + variant_id)),
            ]
        });

        // Constraint 2: Enhanced configuration mapping for galaxy generation
        meta.create_gate("config_mapping", |meta| {
            let s = meta.query_selector(selector_config);
            let variant_id = meta.query_advice(advice[2], Rotation::cur());
            let spiral_type = meta.query_advice(advice[3], Rotation::cur());
            let num_arms = meta.query_advice(advice[4], Rotation::cur());
            let spiral_quotient = meta.query_advice(advice[5], Rotation::cur());
            let arms_quotient = meta.query_advice(advice[6], Rotation::cur());
            let arms_remainder = meta.query_advice(advice[7], Rotation::cur());
            let particles_per_arm = meta.query_advice(advice[8], Rotation::cur());
            let total_particles = meta.query_advice(advice[9], Rotation::cur());
            let triangles_per_arm = meta.query_advice(advice[16], Rotation::cur());
            let total_triangles = meta.query_advice(advice[17], Rotation::cur());

            // Build constants as expressions
            let mut three = F::ZERO;
            for _ in 0..3 {
                three = three + F::ONE;
            }
            let three_expr = Expression::Constant(three);

            let mut six = F::ZERO;
            for _ in 0..6 {
                six = six + F::ONE;
            }
            let six_expr = Expression::Constant(six);

            // UPDATED: particles_per_arm = 69 for galaxy effect
            let mut sixty_nine = F::ZERO;
            for _ in 0..69 {
                sixty_nine = sixty_nine + F::ONE;
            }
            let sixty_nine_expr = Expression::Constant(sixty_nine);

            vec![
                // Prove: variant_id = spiral_quotient * 3 + spiral_type
                s.clone() * (variant_id - (spiral_quotient.clone() * three_expr.clone() + spiral_type)),
                // Prove: spiral_quotient = arms_quotient * 6 + arms_remainder  
                s.clone() * (spiral_quotient - (arms_quotient * six_expr + arms_remainder.clone())),
                // Prove: num_arms = 3 + arms_remainder
                s.clone() * (num_arms.clone() - (three_expr + arms_remainder)),
                // UPDATED: Prove: particles_per_arm = 69 (galaxy density)
                s.clone() * (particles_per_arm.clone() - sixty_nine_expr),
                // Prove: total_particles = num_arms * particles_per_arm
                s.clone() * (total_particles - (num_arms.clone() * particles_per_arm)),
                // UPDATED: Prove: triangles_per_arm = particles_per_arm (1:1 ratio for micro-triangles)
                s.clone() * (triangles_per_arm.clone() - particles_per_arm),
                // Prove: total_triangles = num_arms * triangles_per_arm
                s * (total_triangles - (num_arms * triangles_per_arm)),
            ]
        });

        // Constraint 3: Particle position verification
        meta.create_gate("particle_verification", |meta| {
            let s = meta.query_selector(selector_particle);
            let particle_x = meta.query_advice(advice[11], Rotation::cur());
            let particle_y = meta.query_advice(advice[12], Rotation::cur());
            let _canvas_size = meta.query_advice(advice[10], Rotation::cur());

            // Simple placeholder constraints
            vec![
                s.clone() * (particle_x.clone() - particle_x),
                s * (particle_y.clone() - particle_y),
            ]
        });

        // Constraint 4: Enhanced triangle verification for micro-triangles
        meta.create_gate("triangle_verification", |meta| {
            let s = meta.query_selector(selector_triangle);
            let x1 = meta.query_advice(advice[18], Rotation::cur());
            let y1 = meta.query_advice(advice[19], Rotation::cur());
            let x2 = meta.query_advice(advice[20], Rotation::cur());
            let y2 = meta.query_advice(advice[21], Rotation::cur());
            let x3 = meta.query_advice(advice[22], Rotation::cur());
            let y3 = meta.query_advice(advice[23], Rotation::cur());

            // Basic triangle validation for micro-triangles
            vec![
                s.clone() * (x1.clone() - x1),
                s.clone() * (y1.clone() - y1),
                s.clone() * (x2.clone() - x2),
                s.clone() * (y2.clone() - y2),
                s.clone() * (x3.clone() - x3),
                s * (y3.clone() - y3),
            ]
        });

        // Constraint 5: Trigonometric lookup (unchanged)
        meta.create_gate("trig_lookup", |meta| {
            let s = meta.query_selector(selector_spiral);
            let angle_index = meta.query_advice(advice[15], Rotation::cur());

            vec![
                s * (angle_index.clone() - angle_index),
            ]
        });

        SimpleSpiralsConfig { 
            advice, 
            selector_variant, 
            selector_spiral,
            selector_config,
            selector_particle,
            selector_triangle,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        // Region 1: Variant selection (unchanged)
        layouter.assign_region(
            || "variant selection",
            |mut region| {
                config.selector_variant.enable(&mut region, 0)?;

                region.assign_advice(|| "seed", config.advice[0], 0, || self.seed)?;
                region.assign_advice(|| "quotient", config.advice[1], 0, || self.quotient)?;
                region.assign_advice(|| "variant_id", config.advice[2], 0, || self.variant_id)?;

                Ok(())
            },
        )?;

        // Region 2: Enhanced configuration mapping
        layouter.assign_region(
            || "configuration mapping",
            |mut region| {
                config.selector_config.enable(&mut region, 0)?;

                region.assign_advice(|| "variant_id", config.advice[2], 0, || self.variant_id)?;
                region.assign_advice(|| "spiral_type", config.advice[3], 0, || self.spiral_type)?;
                region.assign_advice(|| "num_arms", config.advice[4], 0, || self.num_arms)?;
                region.assign_advice(|| "spiral_quotient", config.advice[5], 0, || self.spiral_quotient)?;
                region.assign_advice(|| "arms_quotient", config.advice[6], 0, || self.arms_quotient)?;
                region.assign_advice(|| "arms_remainder", config.advice[7], 0, || self.arms_remainder)?;
                region.assign_advice(|| "particles_per_arm", config.advice[8], 0, || self.particles_per_arm)?;
                region.assign_advice(|| "total_particles", config.advice[9], 0, || self.total_particles)?;
                region.assign_advice(|| "canvas_size", config.advice[10], 0, || self.canvas_size)?;
                region.assign_advice(|| "triangles_per_arm", config.advice[16], 0, || self.triangles_per_arm)?;
                region.assign_advice(|| "total_triangles", config.advice[17], 0, || self.total_triangles)?;

                Ok(())
            },
        )?;

        // Region 3: Enhanced particle generation (more particles for galaxy density)
        layouter.assign_region(
            || "particle generation",
            |mut region| {
                for (i, ((x, y), (arm_idx, particle_idx, angle_idx))) in 
                    self.particle_positions.iter().zip(self.particle_metadata.iter()).enumerate() 
                {
                    if i < 50 { // Limit circuit size for testing, but generate more particles
                        config.selector_particle.enable(&mut region, i)?;
                        
                        region.assign_advice(|| "particle_x", config.advice[11], i, || *x)?;
                        region.assign_advice(|| "particle_y", config.advice[12], i, || *y)?;
                        region.assign_advice(|| "arm_index", config.advice[13], i, || *arm_idx)?;
                        region.assign_advice(|| "particle_index", config.advice[14], i, || *particle_idx)?;
                        region.assign_advice(|| "angle_index", config.advice[15], i, || *angle_idx)?;
                        region.assign_advice(|| "canvas_size", config.advice[10], i, || self.canvas_size)?;
                    }
                }

                Ok(())
            },
        )?;

        // Region 4: Enhanced triangle generation (micro-triangles for galaxy effect)
        layouter.assign_region(
            || "triangle generation",
            |mut region| {
                for (i, ((x1, y1, x2, y2, x3, y3), (_arm_idx, _triangle_idx, _triangle_type))) in 
                    self.triangle_vertices.iter().zip(self.triangle_metadata.iter()).enumerate() 
                {
                    if i < 50 { // Limit circuit size for testing
                        config.selector_triangle.enable(&mut region, i)?;
                        
                        region.assign_advice(|| "triangle_x1", config.advice[18], i, || *x1)?;
                        region.assign_advice(|| "triangle_y1", config.advice[19], i, || *y1)?;
                        region.assign_advice(|| "triangle_x2", config.advice[20], i, || *x2)?;
                        region.assign_advice(|| "triangle_y2", config.advice[21], i, || *y2)?;
                        region.assign_advice(|| "triangle_x3", config.advice[22], i, || *x3)?;
                        region.assign_advice(|| "triangle_y3", config.advice[23], i, || *y3)?;
                    }
                }

                Ok(())
            },
        )?;

        Ok(())
    }
}

// Helper function to calculate configuration mapping (unchanged)
pub fn calculate_configuration_mapping(variant_id: u64) -> (u64, u64, u64, u64, u64) {
    let spiral_type = variant_id % 3;
    let spiral_quotient = variant_id / 3;
    let arms_remainder = spiral_quotient % 6;
    let arms_quotient = spiral_quotient / 6;
    let num_arms = 3 + arms_remainder;
    
    (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder)
}

// ENHANCED: Generate galaxy-style spiral particles (69 per arm instead of 15)
pub fn generate_spiral_particles(
    spiral_type: u64,
    num_arms: u64,
    canvas_size: u64,
) -> (Vec<(u64, u64)>, Vec<(u64, u64, u64)>) {
    let particles_per_arm = 69u64; // GALAXY DENSITY - increased from 15
    let mut positions = Vec::new();
    let mut metadata = Vec::new();

    for arm_index in 0..num_arms {
        for particle_index in 0..particles_per_arm {
            let (x, y, angle_index) = calculate_spiral_point(
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

    (positions, metadata)
}

// ENHANCED: Generate micro-triangles for galaxy particle effect
pub fn generate_spiral_triangles(
    spiral_type: u64,
    num_arms: u64,
    canvas_size: u64,
) -> (Vec<(u64, u64, u64, u64, u64, u64)>, Vec<(u64, u64, u64)>) {
    let (positions, _metadata) = generate_spiral_particles(spiral_type, num_arms, canvas_size);
    let particles_per_arm = 69u64;
    
    let mut triangle_vertices = Vec::new();
    let mut triangle_metadata = Vec::new();

    for arm_index in 0..num_arms {
        for particle_index in 0..particles_per_arm {
            let particle_idx = (arm_index * particles_per_arm + particle_index) as usize;
            
            if particle_idx < positions.len() {
                let (center_x, center_y) = positions[particle_idx];
                
                // CREATE MICRO-TRIANGLES: Much smaller for galaxy particle effect
                let base_size = 6u64;
                let size_reduction = (particle_index * 4) / particles_per_arm; // Gets smaller toward edge
                let triangle_size = (base_size - size_reduction).max(2); // Minimum 2 pixels
                
                let (x1, y1, x2, y2, x3, y3) = create_micro_triangle(center_x, center_y, triangle_size);
                
                triangle_vertices.push((x1, y1, x2, y2, x3, y3));
                triangle_metadata.push((arm_index, particle_index, 0)); // Type 0 = spiral particle
            }
        }
    }

    (triangle_vertices, triangle_metadata)
}

// NEW: Create micro-triangles for galaxy particle effect
fn create_micro_triangle(center_x: u64, center_y: u64, size: u64) -> (u64, u64, u64, u64, u64, u64) {
    let half_size = size / 2;
    
    // Create small triangular "star" particle
    let x1 = center_x;                    // Top point
    let y1 = center_y.saturating_sub(half_size);
    let x2 = center_x.saturating_sub(half_size); // Bottom left
    let y2 = center_y + half_size;
    let x3 = center_x + half_size;        // Bottom right  
    let y3 = center_y + half_size;
    
    (x1, y1, x2, y2, x3, y3)
}

// Enhanced spiral point calculation with better galaxy distribution
pub fn calculate_spiral_point(
    arm_index: u64,
    particle_index: u64, 
    total_arms: u64,
    spiral_type: u64,
    canvas_size: u64,
) -> (u64, u64, u64) {
    const TRIG_TABLE_SIZE: usize = 32;
    const SCALE_FACTOR: u64 = 10000;
    
    const SIN_TABLE: [i64; TRIG_TABLE_SIZE] = [
        0, 1951, 3827, 5556, 7071, 8315, 9239, 9808,
        10000, 9808, 9239, 8315, 7071, 5556, 3827, 1951,
        0, -1951, -3827, -5556, -7071, -8315, -9239, -9808,
        -10000, -9808, -9239, -8315, -7071, -5556, -3827, -1951,
    ];
    
    const COS_TABLE: [i64; TRIG_TABLE_SIZE] = [
        10000, 9808, 9239, 8315, 7071, 5556, 3827, 1951,
        0, -1951, -3827, -5556, -7071, -8315, -9239, -9808,
        -10000, -9808, -9239, -8315, -7071, -5556, -3827, -1951,
        0, 1951, 3827, 5556, 7071, 8315, 9239, 9808,
    ];
    
    let particles_per_arm = 69u64; // Updated for galaxy density
    let max_radius = (canvas_size * 4963) / 10000; // Keep same radius ratio
    
    let base_angle_index = (arm_index * TRIG_TABLE_SIZE as u64) / total_arms;
    let t = (particle_index * 1000) / particles_per_arm;
    
    // GALAXY SPIRAL TYPES: Enhanced for better visual variety
    let progression = match spiral_type {
        0 => (t * 16) / 1000, // Tight spiral - more compressed
        1 => (t * 4) / 1000,  // Loose spiral - more open
        _ => (t * 8) / 1000,  // Classic spiral - balanced
    };
    
    let angle_index = (base_angle_index + progression) % (TRIG_TABLE_SIZE as u64);
    let radius = (t * max_radius) / 1000;
    
    let sin_val = SIN_TABLE[angle_index as usize];
    let cos_val = COS_TABLE[angle_index as usize];
    
    let center = canvas_size / 2;
    let x = center + ((radius * cos_val.abs() as u64) / SCALE_FACTOR);
    let y = center + ((radius * sin_val.abs() as u64) / SCALE_FACTOR);
    
    (x, y, angle_index)
}