pub mod linear_algebra;

#[cfg(test)]
mod test_linear_algebra {
    use crate::linear_algebra::{Add, ApproxEquals, Matrix, MatrixMult, Norm, ScalarMult, Trace, Transpose};

    fn get_test_matrix() -> Matrix<f64, 3, 3> {
        Matrix {
            values: [
                [1., 2., 3.],
                [4., 5., 6.],
                [7., 8., 9.]
            ]
        }
    }

    #[test]
    fn test_zero_matrix() {
        let zero_matrix = Matrix {
            values: [[0.; 3]; 3]
        };
        let zero_matrix_norm = zero_matrix.norm().unwrap();
        assert!(zero_matrix_norm < 0.0001, "Expected zero matrix to have small norm; got {:?}", zero_matrix_norm);

        let test_matrix = get_test_matrix();
        let right_sum = zero_matrix.add(&test_matrix);
        assert!(right_sum.approx_equals_default(&test_matrix).unwrap(), "Expected right sum to equal test matrix, got {:?}.", right_sum.values);

        let doubled = test_matrix.scalar_mult(2.);
        let expected_doubled_matrix = Matrix {
            values: [
                [2., 4., 6.],
                [8., 10., 12.],
                [14., 16., 18.]
            ]
        };
        assert!(expected_doubled_matrix.approx_equals_default(&doubled).unwrap(), "Expected doubled matrix, got {:?}.", doubled);

    }

    #[test]
    fn test_trace() {
        let test_matrix = get_test_matrix();
        assert_eq!(test_matrix.trace(), 15.)
    }

    #[test]
    fn test_transpose() {
        let test_matrix = get_test_matrix();
        let expected_transpose = Matrix {
            values: [
                [1., 4., 7.],
                [2., 5., 8.],
                [3., 6., 9.]
            ]
        };
        let actual_transpose = test_matrix.transpose();
        assert!(expected_transpose.approx_equals_default(&actual_transpose).unwrap(), "Expected transpose of matrix, got {:?}", actual_transpose.values);
    }

    #[test]
    fn test_identity_matrix() {

        let identity_matrix = Matrix {
            values: [
                [1., 0., 0.],
                [0., 1., 0.],
                [0., 0., 1.]
            ]
        };
        let test_matrix = get_test_matrix();
        let result_1 = test_matrix.mult(&identity_matrix);
        assert!(result_1.approx_equals_default(&test_matrix).unwrap(), "Expected test matrix, got {:?}", result_1.values);
        let result_2 = identity_matrix.mult(&test_matrix);
        assert!(result_2.approx_equals_default(&test_matrix).unwrap(), "Expected test matrix, got {:?}", result_2.values);
    }

}