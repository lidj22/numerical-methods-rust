use std::time::Instant;

// Fast 3x3 matrix with Copy semantics for performance
#[derive(Clone, Copy)]
struct FastMatrix3x3 {
    values: [[f64; 3]; 3]
}

impl FastMatrix3x3 {
    fn identity() -> Self {
        FastMatrix3x3 {
            values: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0]
            ]
        }
    }
    
    fn print(&self) {
        for i in 0..3 {
            println!("{:?}", self.values[i]);
        }
    }
    
    // Optimized matrix multiplication
    #[inline(always)]
    fn mult(&self, other: &Self) -> Self {
        let mut result = [[0.0; 3]; 3];
        
        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0.0;
                for k in 0..3 {
                    sum += self.values[i][k] * other.values[k][j];
                }
                result[i][j] = sum;
            }
        }
        
        FastMatrix3x3 { values: result }
    }
}

fn main() {
    // First approach: direct iteration
    {
        // Create a rotation matrix
        let rotation = FastMatrix3x3 {
            values: [
                [1.0, 0.0, 0.0],
                [0.0, 0.0, -1.0],
                [0.0, 1.0, 0.0]
            ]
        };
        
        // Initial state is identity
        let mut state = FastMatrix3x3::identity();
        
        // Time the computation
        let start = Instant::now();
        
        // Compute evolution with direct iteration
        for _ in 0..1000002 {
            state = rotation.mult(&state);
        }
        
        let duration = start.elapsed();
        
        println!("Direct iteration: total time is {} seconds.", duration.as_secs_f64());
        state.print();
    }
    
    // Second approach: binary exponentiation
    {
        // Create a rotation matrix
        let mut temp_rot = FastMatrix3x3 {
            values: [
                [1.0, 0.0, 0.0],
                [0.0, 0.0, -1.0],
                [0.0, 1.0, 0.0]
            ]
        };
        
        // Initial state is identity
        let mut state = FastMatrix3x3::identity();
        
        // Time the computation
        let start = Instant::now();
        
        // Binary exponentiation algorithm - O(log n) complexity
        let mut steps = 1000002;
        while steps > 0 {
            if steps % 2 == 1 {
                state = temp_rot.mult(&state);
            }
            temp_rot = temp_rot.mult(&temp_rot);
            steps /= 2;
        }
        
        let duration = start.elapsed();
        
        println!("Binary exponentiation: total time is {} seconds.", duration.as_secs_f64());
        state.print();
    }
} 