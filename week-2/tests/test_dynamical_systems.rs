use week_2::{dynamical_systems::{DynamicalSystem1D, DynamicalSystem3D}, linear_algebra::{create_from_profile, SquareMatrix3}};

// evolution along -sine curve by some small stepsize.
// looking at the graph, if less than pi/2, eventually
// goes to zero.
fn sine_evolution(state: f64) -> f64 {
    let diff = -state.sin();
    state + diff * 0.0001
}

// trivial zero sink with negative eigenvalues
// I just realized this was supposed to act on vectors, not matrices. Oops
fn zero_sink(state: SquareMatrix3) -> SquareMatrix3 {
    let a = create_from_profile(week_2::linear_algebra::CommonSquareMatrixProfile::Identity).negative();
    state.add(&a.mult(&state))
}

#[test]
fn test_sine_system() {

    // initial state less than pi/2.
    let system = DynamicalSystem1D {
        evolution: sine_evolution,
        initial_state: 1.5
    };

    // assert state basically converges to zero.
    let final_state = system.compute(1000000);
    assert!(final_state.abs() < 0.0001);

}

#[test]
fn test_3d_system() {

    // define some evolution that converges to zero.
    let system = DynamicalSystem3D {
        evolution: zero_sink,
        initial_state: SquareMatrix3 { values: vec![
            vec![1., 10., 3.],
            vec![2., -1., 3.],
            vec![1., 3., 0.],
        ] }
    };

    let final_state = system.compute(100000);
    let norm = final_state.max_norm();
    assert!(norm < 0.0001, "Expected norm to be small, but got {}", norm);

}