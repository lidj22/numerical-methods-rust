use crate::linear_algebra::{LinearAlgebraError, Matrix};

pub trait Norm<T> {
    fn norm(&self) -> Result<T, LinearAlgebraError>;
}

pub trait Trace<T> {
    fn trace(&self) -> T;
}

pub trait Negative<T, const ROWS: usize, const COLS: usize> {
    fn negative(&self) -> Matrix<T, ROWS, COLS>;
}

pub trait Transpose<T, const ROWS: usize, const COLS: usize> {
    fn transpose(&self) -> Matrix<T, COLS, ROWS>;
}

pub trait ApproxEquals<const ROWS: usize, const COLS: usize, T> {
    const DEFAULT_THRESHOLD: T; // this is an associated item.
    fn approx_equals(&self, rhs: &Matrix<T, ROWS, COLS>, threshold: T) -> Result<bool, LinearAlgebraError>;
    fn approx_equals_default(&self, rhs: &Matrix<T, ROWS, COLS>) -> Result<bool, LinearAlgebraError> {
        self.approx_equals(rhs, Self::DEFAULT_THRESHOLD)
    }
}

pub trait Add<T, const ROWS: usize, const COLS: usize> {
    fn add(&self, rhs: &Matrix<T, ROWS, COLS>) -> Matrix<T, ROWS, COLS>;
}

// Element-wise application.
pub trait Apply<T, const ROWS: usize, const COLS: usize> {
    // Q: why not fn(T) -> T?
    fn apply<F>(&self, function: F) -> Matrix<T, ROWS, COLS> where F: Fn(T) -> T;
}

pub trait ScalarMult<T, const ROWS: usize, const COLS: usize> {
    fn scalar_mult(&self, scalar: T) -> Matrix<T, ROWS, COLS>;
}

// description: first matrix is AxB, second matrix is BxC.
pub trait MatrixMult<T, const B: usize, const C: usize> {
    type Output; // associated placeholder type
    fn mult(&self, rhs: &Matrix<T, B, C>) -> Self::Output;
}

// square matrix-specific operations
pub trait SquareMatrixOps<T, const N: usize> {

    fn determinant(&self) -> T;
    fn pow(&self, exp: u32) -> Self;

}
