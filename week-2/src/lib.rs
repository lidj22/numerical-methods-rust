
pub mod linear_algebra;
pub mod dynamical_systems;

#[cfg(test)]
mod test_linear_algebra {
    use crate::linear_algebra::{create_from_profile, CommonSquareMatrixProfile, SquareMatrix3};

    #[test]
    fn test_identity_matrix() {
        let identity_matrix = create_from_profile(CommonSquareMatrixProfile::Identity);
        let matrix_1 = SquareMatrix3 {
            values: vec![
                vec![1., 2., 3.],
                vec![4., 5., 6.],
                vec![7., 8., 9.]
            ]
        };
        let right_mult = identity_matrix.mult(&matrix_1);
        let left_mult = matrix_1.mult(&identity_matrix);
        assert!(right_mult.values[0][0] == 1.);
        assert!(right_mult.equals(&matrix_1));
        assert!(left_mult.equals(&matrix_1));
    }

    #[test]
    fn test_determinant() {
        let matrix_1 = SquareMatrix3 {
            values: vec![
                vec![1., 2., 3.],
                vec![4., 5., 6.],
                vec![7., 8., 9.]
            ]
        };
        let det = matrix_1.det();
        assert!(det == 0.);
    }

    #[test]
    fn test_is_valid() {
        let matrix_1 = SquareMatrix3 {
            values: vec![
                vec![1., 2., 3.],
                vec![4., 5., 6.],
                vec![7., 8.]
            ]
        };
        assert!(!matrix_1.is_valid());
    }

    #[test]
    fn test_negative() {
        let matrix_2 = SquareMatrix3 {
            values: vec![
                vec![1., 0., 0.],
                vec![0., 1., -1.],
                vec![0., -1., 1.]
            ]
        };
        assert!(matrix_2.negative().equals(&SquareMatrix3 {
            values: vec![
                vec![-1., 0., 0.],
                vec![0., -1., 1.],
                vec![0., 1., -1.]
            ]
        }));
    }

    #[test]
    fn test_subtraction() {
        let matrix_1 = SquareMatrix3 {
            values: vec![
                vec![1., 0., 0.],
                vec![0., 1., 1.],
                vec![1., 0., 1.]
            ]
        };
        let matrix_2 = SquareMatrix3 {
            values: vec![
                vec![0., 1., 1.],
                vec![0., 1., 1.],
                vec![1., 1., 1.]
            ]
        };
        assert!(matrix_1.subtract(&matrix_2).equals(&SquareMatrix3 { values: vec![
            vec![1., -1., -1.],
            vec![0., 0., 0.],
            vec![0., -1., 0.]
        ] }));
    }

}
