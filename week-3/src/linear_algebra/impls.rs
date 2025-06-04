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

impl <const ROWS: usize, const COLS: usize> Norm<f64> for Matrix<f64, ROWS, COLS> {
    fn norm(&self) -> Result<f64, LinearAlgebraError> {
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

impl <const ROWS: usize, const COLS: usize> Trace<f64> for Matrix<f64, ROWS, COLS> {
    fn trace(&self) -> f64 {
        let mut total = 0.;
        for i in 0..ROWS.min(COLS) {
            total += self.values[i][i];
        }
        total
    }
}

impl <const ROWS: usize, const COLS: usize> Negative<f64, ROWS, COLS> for Matrix<f64, ROWS, COLS> {
    fn negative(&self) -> Matrix<f64, ROWS, COLS> {
        let mut values = self.values; // we are using Copy trait here.
        for i in 0..ROWS {
            for j in 0..COLS {
                values[i][j] = -self.values[i][j];
            }
        }
        Matrix { values: values }
    }
}

impl <const ROWS: usize, const COLS: usize> Transpose<f64, ROWS, COLS> for Matrix<f64, ROWS, COLS> {
    fn transpose(&self) -> Matrix<f64, COLS, ROWS> {
        let mut values = [[0.; ROWS]; COLS];
        for i in 0..ROWS {
            for j in 0..COLS {
                values[j][i] = self.values[i][j];
            }
        }
        Matrix { values: values }
    }
}

impl <const ROWS: usize, const COLS: usize> Add<f64, ROWS, COLS> for Matrix<f64, ROWS, COLS> {
    fn add(&self, rhs: &Matrix<f64, ROWS, COLS>) -> Matrix<f64, ROWS, COLS> {
        let mut values = self.values; // we are using Copy trait here.
        for i in 0..ROWS {
            for j in 0..COLS {
                values[i][j] += rhs.values[i][j]; 
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

impl <const ROWS: usize, const COLS: usize> ScalarMult<f64, ROWS, COLS> for Matrix<f64, ROWS, COLS> {
    fn scalar_mult(&self, scalar: f64) -> Matrix<f64, ROWS, COLS> {
        let mut values = self.values; // we are using Copy trait here.
        for i in 0..ROWS {
            for j in 0..COLS {
                values[i][j] *= scalar;
            }
        }
        Matrix { values: values }
    }
}

impl <const A: usize, const B: usize, const C: usize> MatrixMult<f64, B, C> for Matrix<f64, A, B> {
    type Output = Matrix<f64, A, C>;
    fn mult(&self, rhs: &Matrix<f64, B, C>) -> Matrix<f64, A, C> {
        let mut values = [[0.; C]; A];
        for i in 0..A {
            for j in 0..C {
                let mut total = 0.;
                for k in 0..B {
                    total = total + self.values[i][k] * rhs.values[k][j];
                }
                values[i][j] = total;
            }
        }
        Matrix { values: values }
    }
}