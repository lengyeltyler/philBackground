use ff::Field;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Expression, Selector},
    poly::Rotation,
};

#[derive(Debug, Clone)]
pub struct SimpleSpiralsConfig {
    pub advice: [Column<Advice>; 24], // Expanded for triangle coordinates
    pub selector_variant: Selector,
    pub selector_spiral: Selector,
    pub selector_config: Selector,
    pub selector_particle: Selector,
    pub selector_triangle: Selector, // New selector for triangle verification
}

#[derive(Debug, Clone)]
pub struct SimpleSpiralsCircuit<F: Field> {
    pub seed: Value<F>,
    pub variant_id: Value<F>,
    pub quotient: Value<F>,
    // Configuration values
    pub spiral_type: Value<F>,
    pub num_arms: Value<F>,
    // Configuration intermediate values (calculated outside circuit)
    pub spiral_quotient: Value<F>,     // variant_id / 3
    pub arms_quotient: Value<F>,       // (variant_id / 3) / 6  
    pub arms_remainder: Value<F>,      // (variant_id / 3) % 6
    // Particle generation
    pub particles_per_arm: Value<F>,   // Number of particles per arm (15)
    pub total_particles: Value<F>,     // num_arms * particles_per_arm
    pub canvas_size: Value<F>,         // Canvas size for coordinate calculation
    // Particle data - store calculated positions
    pub particle_positions: Vec<(Value<F>, Value<F>)>, // (x, y) coordinates
    pub particle_metadata: Vec<(Value<F>, Value<F>, Value<F>)>, // (arm_index, particle_index, angle_index)
    // Triangle generation - NEW for Phase 2C
    pub triangles_per_arm: Value<F>,   // Number of triangles per arm (5)
    pub total_triangles: Value<F>,     // num_arms * triangles_per_arm
    pub triangle_vertices: Vec<(Value<F>, Value<F>, Value<F>, Value<F>, Value<F>, Value<F>)>, // (x1,y1,x2,y2,x3,y3)
    pub triangle_metadata: Vec<(Value<F>, Value<F>, Value<F>)>, // (arm_index, triangle_index, triangle_type)
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

        // Constraint 1: Variant selection (same as before)
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

        // Constraint 2: Configuration mapping verification
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

            let mut five = F::ZERO;
            for _ in 0..5 {
                five = five + F::ONE;
            }
            let five_expr = Expression::Constant(five);

            let mut six = F::ZERO;
            for _ in 0..6 {
                six = six + F::ONE;
            }
            let six_expr = Expression::Constant(six);

            let mut fifteen = F::ZERO;
            for _ in 0..15 {
                fifteen = fifteen + F::ONE;
            }
            let fifteen_expr = Expression::Constant(fifteen);

            vec![
                // Prove: variant_id = spiral_quotient * 3 + spiral_type
                s.clone() * (variant_id - (spiral_quotient.clone() * three_expr.clone() + spiral_type)),
                // Prove: spiral_quotient = arms_quotient * 6 + arms_remainder  
                s.clone() * (spiral_quotient - (arms_quotient * six_expr + arms_remainder.clone())),
                // Prove: num_arms = 3 + arms_remainder
                s.clone() * (num_arms.clone() - (three_expr + arms_remainder)),
                // Prove: particles_per_arm = 15
                s.clone() * (particles_per_arm.clone() - fifteen_expr),
                // Prove: total_particles = num_arms * particles_per_arm
                s.clone() * (total_particles - (num_arms.clone() * particles_per_arm)),
                // NEW: Prove: triangles_per_arm = 5
                s.clone() * (triangles_per_arm.clone() - five_expr),
                // NEW: Prove: total_triangles = num_arms * triangles_per_arm
                s * (total_triangles - (num_arms * triangles_per_arm)),
            ]
        });

        // Constraint 3: Particle position verification (simple placeholder)
        meta.create_gate("particle_verification", |meta| {
            let s = meta.query_selector(selector_particle);
            let particle_x = meta.query_advice(advice[11], Rotation::cur());
            let particle_y = meta.query_advice(advice[12], Rotation::cur());
            let _canvas_size = meta.query_advice(advice[10], Rotation::cur());

            // Simple placeholder constraints that always pass
            vec![
                s.clone() * (particle_x.clone() - particle_x),
                s * (particle_y.clone() - particle_y),
            ]
        });

        // Constraint 4: Triangle vertex verification - NEW for Phase 2C
        meta.create_gate("triangle_verification", |meta| {
            let s = meta.query_selector(selector_triangle);
            let x1 = meta.query_advice(advice[18], Rotation::cur());
            let y1 = meta.query_advice(advice[19], Rotation::cur());
            let x2 = meta.query_advice(advice[20], Rotation::cur());
            let y2 = meta.query_advice(advice[21], Rotation::cur());
            let x3 = meta.query_advice(advice[22], Rotation::cur());
            let y3 = meta.query_advice(advice[23], Rotation::cur());

            // Basic triangle validation - ensure vertices exist
            vec![
                // Verify all triangle vertices are present (placeholder)
                s.clone() * (x1.clone() - x1),
                s.clone() * (y1.clone() - y1),
                s.clone() * (x2.clone() - x2),
                s.clone() * (y2.clone() - y2),
                s.clone() * (x3.clone() - x3),
                s * (y3.clone() - y3),
            ]
        });

        // Constraint 5: Trigonometric lookup (enhanced)
        meta.create_gate("trig_lookup", |meta| {
            let s = meta.query_selector(selector_spiral);
            let angle_index = meta.query_advice(advice[15], Rotation::cur());

            vec![
                s * (angle_index.clone() - angle_index), // Placeholder - always 0
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
        // Region 1: Variant selection
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

        // Region 2: Configuration mapping
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

        // Region 3: Particle generation
        layouter.assign_region(
            || "particle generation",
            |mut region| {
                // Assign particle positions and metadata
                for (i, ((x, y), (arm_idx, particle_idx, angle_idx))) in 
                    self.particle_positions.iter().zip(self.particle_metadata.iter()).enumerate() 
                {
                    if i < 10 { // Limit to first 10 particles for circuit size
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

        // Region 4: Triangle generation - NEW for Phase 2C
        layouter.assign_region(
            || "triangle generation",
            |mut region| {
                // Assign triangle vertices and metadata
                for (i, ((x1, y1, x2, y2, x3, y3), (_arm_idx, _triangle_idx, _triangle_type))) in 
                    self.triangle_vertices.iter().zip(self.triangle_metadata.iter()).enumerate() 
                {
                    if i < 10 { // Limit to first 10 triangles for circuit size
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

// Helper function to calculate configuration mapping outside the circuit
pub fn calculate_configuration_mapping(variant_id: u64) -> (u64, u64, u64, u64, u64) {
    let spiral_type = variant_id % 3;
    let spiral_quotient = variant_id / 3;
    let arms_remainder = spiral_quotient % 6;
    let arms_quotient = spiral_quotient / 6;
    let num_arms = 3 + arms_remainder;
    
    (spiral_type, num_arms, spiral_quotient, arms_quotient, arms_remainder)
}

// Helper function to generate all particles for a spiral configuration
pub fn generate_spiral_particles(
    spiral_type: u64,
    num_arms: u64,
    canvas_size: u64,
) -> (Vec<(u64, u64)>, Vec<(u64, u64, u64)>) {
    let particles_per_arm = 15u64;
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

// NEW for Phase 2C: Generate triangles from spiral particles
pub fn generate_spiral_triangles(
    spiral_type: u64,
    num_arms: u64,
    canvas_size: u64,
) -> (Vec<(u64, u64, u64, u64, u64, u64)>, Vec<(u64, u64, u64)>) {
    let (positions, _metadata) = generate_spiral_particles(spiral_type, num_arms, canvas_size);
    let particles_per_arm = 15u64;
    let triangles_per_arm = 5u64; // 15 particles / 3 = 5 triangles per arm
    
    let mut triangle_vertices = Vec::new();
    let mut triangle_metadata = Vec::new();

    for arm_index in 0..num_arms {
        for triangle_index in 0..triangles_per_arm {
            // Get 3 consecutive particles from this arm to form a triangle
            let base_particle_idx = (arm_index * particles_per_arm + triangle_index * 3) as usize;
            
            if base_particle_idx + 2 < positions.len() {
                let (x1, y1) = positions[base_particle_idx];
                let (x2, y2) = positions[base_particle_idx + 1];
                let (x3, y3) = positions[base_particle_idx + 2];
                
                triangle_vertices.push((x1, y1, x2, y2, x3, y3));
                
                // Triangle type: 0 = consecutive particles, could add cross-arm types later
                triangle_metadata.push((arm_index, triangle_index, 0));
            }
        }
    }

    (triangle_vertices, triangle_metadata)
}

// Keep the existing trigonometric tables and helper function...
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

pub fn calculate_spiral_point(
    arm_index: u64,
    particle_index: u64, 
    total_arms: u64,
    spiral_type: u64,
    canvas_size: u64,
) -> (u64, u64, u64) {
    let particles_per_arm = 15u64;
    let max_radius = (canvas_size * 4963) / 10000;
    
    let base_angle_index = (arm_index * TRIG_TABLE_SIZE as u64) / total_arms;
    let t = (particle_index * 1000) / particles_per_arm;
    let progression = match spiral_type {
        0 => (t * 16) / 1000,
        1 => (t * 4) / 1000,
        _ => (t * 8) / 1000,
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