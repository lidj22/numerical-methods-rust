use std::time::Instant;

use week_2::{dynamical_systems::DynamicalSystem3D, linear_algebra::{create_from_profile, SquareMatrix3}};

fn main() {

    let identity_matrix = SquareMatrix3 {
        values: vec![vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.]]
    };
    let mat_1 = SquareMatrix3 {
        values: vec![
            vec![2., 3., 1.],
            vec![1., -3., -2.],
            vec![4., -1., 0.],
        ]
    };

    assert!(identity_matrix.is_valid());
    assert!(mat_1.is_valid());

    let transpose = mat_1.transpose();
    println!("Transpose is:");
    transpose.print();

    let mult_result_1 = identity_matrix.mult(&mat_1);
    println!("Result of multiplying by identity: ");
    mult_result_1.print();

    let det_result_1 = mat_1.det();
    println!("Determinant of matrix is: {:?}", det_result_1);

    println!("Do some evolutions");
    // basic rotation by pi/2 radians

    // do some timing.
    fn rotation_matrix_opt(initial_state: SquareMatrix3) -> SquareMatrix3 {
        let rotation = SquareMatrix3 {
            values: vec![
                vec![1., 0., 0.],
                vec![0., 0., -1.],
                vec![0., 1., 0.]
            ]
        };
        rotation.mult(&initial_state)
    }

    let system = DynamicalSystem3D {
        evolution: rotation_matrix_opt,
        initial_state: create_from_profile(week_2::linear_algebra::CommonSquareMatrixProfile::Identity)
    };

    let start = Instant::now();
    let final_state = system.compute(1000002);
    let duration = start.elapsed();

    // takes approximately 2.8 seconds.
    // amusingly slower than the python numpy implementation but guess that's what
    // happens when you use an optimized lib with c bindings...
    // wait... it's slower than vanilla python...?
    println!("Finished computation, total time is {} seconds.", duration.as_secs_f64());
    final_state.print();

}
