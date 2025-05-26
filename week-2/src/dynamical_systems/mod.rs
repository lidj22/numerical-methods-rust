use crate::linear_algebra::SquareMatrix3;

pub struct DynamicalSystem1D {

    // a function that takes a state and moves it to the next state.
    pub evolution: fn(f64) -> f64,
    pub initial_state: f64,

}

impl DynamicalSystem1D {
    
    /**
     * Compute state after number of steps.
     */
    pub fn compute(&self, steps: usize) -> f64 {
        let mut state = self.initial_state;
        for _ in 0..steps {
            state = (self.evolution)(state);
        }
        state
    }

}

pub struct DynamicalSystem3D {

    pub evolution: fn(SquareMatrix3) -> SquareMatrix3,
    pub initial_state: SquareMatrix3

}

// TODO: is there a more efficient way to do this
// that not only *sounds* like a good idea, but *is* a better idea?
impl DynamicalSystem3D {
    
    pub fn compute(&self, steps: usize) -> SquareMatrix3 {
        let mut state = SquareMatrix3 { values: self.initial_state.values.clone() };
        for _ in 0..steps {
            state = (self.evolution)(state);
        }
        SquareMatrix3 { values: state.values.clone() }
    }

}
