use num_traits::Float;

use crate::linear_algebra::errors::LinearAlgebraError;
use crate::linear_algebra::Add;
use crate::linear_algebra::Apply;
use crate::linear_algebra::ApproxEquals;
use crate::linear_algebra::Matrix;
use crate::linear_algebra::MatrixMult;
use crate::linear_algebra::Negative;
use crate::linear_algebra::Norm;
use crate::linear_algebra::ScalarMult;
use crate::linear_algebra::Trace;
use crate::linear_algebra::Transpose;

impl <T, const ROWS: usize, const COLS: usize> Norm<T> for Matrix<T, ROWS, COLS> where T: Float {
    fn norm(&self) -> Result<T, LinearAlgebraError> {
        if ROWS == 0 && COLS == 0 {
            return Err(LinearAlgebraError::EmptyMatrix);
        }
        let mut max_value = self.values[0][0].abs();
        for i in 0..ROWS {
            for j in 0..COLS {
                let curr_value = self.values[i][j].abs();
                if curr_value > max_value {
                    max_value = curr_value;
                }
            }
        }
        Ok(max_value)
    }
}

impl <T, const ROWS: usize, const COLS: usize> Trace<T> for Matrix<T, ROWS, COLS> where T: Float {
    fn trace(&self) -> T {
        let mut total = T::zero();
        for i in 0..ROWS.min(COLS) {
            total = total + self.values[i][i];
        }
        total
    }
}

impl <T, const ROWS: usize, const COLS: usize> Negative<T, ROWS, COLS> for Matrix<T, ROWS, COLS> where T: Float {
    fn negative(&self) -> Matrix<T, ROWS, COLS> {
        let mut values = self.values; // we are using Copy trait here.
        for i in 0..ROWS {
            for j in 0..COLS {
                values[i][j] = -self.values[i][j];
            }
        }
        Matrix { values: values }
    }
}

impl <T, const ROWS: usize, const COLS: usize> Transpose<T, ROWS, COLS> for Matrix<T, ROWS, COLS> where T: Float {
    fn transpose(&self) -> Matrix<T, COLS, ROWS> {
        let mut values = [[T::zero(); ROWS]; COLS];
        for i in 0..ROWS {
            for j in 0..COLS {
                values[j][i] = self.values[i][j];
            }
        }
        Matrix { values: values }
    }
}

impl <T, const ROWS: usize, const COLS: usize> Add<T, ROWS, COLS> for Matrix<T, ROWS, COLS> where T: Float {
    fn add(&self, rhs: &Matrix<T, ROWS, COLS>) -> Matrix<T, ROWS, COLS> {
        let mut values = self.values; // we are using Copy trait here.
        for i in 0..ROWS {
            for j in 0..COLS {
                values[i][j] = values[i][j] + rhs.values[i][j]; 
            }
        }
        Matrix { values: values }
    }
}

impl <const ROWS: usize, const COLS: usize> ApproxEquals<ROWS, COLS, f64> for Matrix<f64, ROWS, COLS> {
    const DEFAULT_THRESHOLD: f64 = 0.00001;
    fn approx_equals(&self, rhs: &Matrix<f64, ROWS, COLS>, threshold: f64) -> Result<bool, LinearAlgebraError> {
        match self.add(&rhs.negative()).norm() {
            Ok(norm) => Ok(norm < threshold),
            Err(LinearAlgebraError::EmptyMatrix) => Err(LinearAlgebraError::EmptyMatrix)
        }
    }
}

impl <T, const ROWS: usize, const COLS: usize> Apply<T, ROWS, COLS> for Matrix<T, ROWS, COLS> where T: Copy {
    fn apply<F>(&self, function: F) -> Matrix<T, ROWS, COLS> where F: Fn(T) -> T {
        let mut values = self.values; // we are using Copy trait here.
        for i in 0..ROWS {
            for j in 0..COLS {
                values[i][j] = function(self.values[i][j]);
            }
        }
        Matrix { values: values }
    }
}

impl <T, const ROWS: usize, const COLS: usize> ScalarMult<T, ROWS, COLS> for Matrix<T, ROWS, COLS> where T: Float {
    fn scalar_mult(&self, scalar: T) -> Matrix<T, ROWS, COLS> {
        let mut values = self.values; // we are using Copy trait here.
        for i in 0..ROWS {
            for j in 0..COLS {
                values[i][j] = values[i][j] * scalar;
            }
        }
        Matrix { values: values }
    }
}

impl <T, const A: usize, const B: usize, const C: usize> MatrixMult<T, B, C> for Matrix<T, A, B> where T: Float {
    type Output = Matrix<T, A, C>;
    fn mult(&self, rhs: &Matrix<T, B, C>) -> Matrix<T, A, C> {
        let mut values = [[T::zero(); C]; A];
        for i in 0..A {
            for j in 0..C {
                let mut total = T::zero();
                for k in 0..B {
                    total = total + self.values[i][k] * rhs.values[k][j];
                }
                values[i][j] = total;
            }
        }
        Matrix { values: values }
    }
}
