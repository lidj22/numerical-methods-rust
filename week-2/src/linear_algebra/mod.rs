
/**
 * This is a simple not-so-good implementation of 3-dimensional square matrix.
 */
pub struct SquareMatrix3 {

    // does not guarantee dimensions, runtime errors ahoy
    pub values: Vec<Vec<f64>>,

}

pub enum CommonSquareMatrixProfile {
    Zero,
    Identity,
    One,
}

pub fn create_from_profile(profile: CommonSquareMatrixProfile) -> SquareMatrix3 {
    match profile {
        CommonSquareMatrixProfile::Identity => {
            SquareMatrix3 {
                values: vec![
                    vec![1., 0., 0.],
                    vec![0., 1., 0.],
                    vec![0., 0., 1.]
                ]
            }
        },
        CommonSquareMatrixProfile::One => {
            SquareMatrix3 {
                values: vec![
                    vec![1., 1., 1.],
                    vec![1., 1., 1.],
                    vec![1., 1., 1.],
                ]
            }
        },
        CommonSquareMatrixProfile::Zero => {
            SquareMatrix3 {
                values: vec![
                    vec![0., 0., 0.],
                    vec![0., 0., 0.],
                    vec![0., 0., 0.],
                ]
            }
        },
    }
}

impl SquareMatrix3 {

    /**
     * TODO: it would be better if the matrix knew on init whether it was valid.
     */
    pub fn is_valid(&self) -> bool {
        if self.values.len() != 3 {
            return false;
        }
        for row in self.values.iter() {
            if row.len() != 3 {
                return false;
            }
        }
        return true;
    }

    pub fn get_num_rows(&self) -> usize {
        self.values.len()
    }

    pub fn get_num_cols(&self) -> usize {
        self.values[0].len()
    }

    pub fn print(&self) {
        for i in 0..3 {
            println!("{:?}", self.values[i]);
        }
    }

    pub fn negative(&self) -> SquareMatrix3 {
        let mut values = create_from_profile(CommonSquareMatrixProfile::Zero).values.clone();
        for i in 0..3 {
            for j in 0..3 {
                values[i][j] = -self.values[i][j];
            }
        }
        SquareMatrix3 { values: values }
    }

    /**
     * Frobenius norm (real valued)
     * it's easy to compute that's all.
     */
    pub fn frobenius_norm(&self) -> f64 {
        self.transpose().mult(self).trace().sqrt()
    }

    /**
     * max norm
     */
    pub fn max_norm(&self) -> f64 {
        let mut max_val = self.values[0][0].abs();
        for i in 0..3 {
            for j in 0..3 {
                max_val = self.values[i][j].abs().max(max_val);
            }
        }
        max_val
    }

    /**
     * Loose equals for floating point.
     */
    pub fn equals(&self, other: &SquareMatrix3) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if (self.values[i][j] - other.values[i][j]).abs() > 0.00001_f64 {
                    return false;
                }
            }
        }
        return true;
    }

    // transpose
    pub fn transpose(&self) -> SquareMatrix3 {
        let mut values = create_from_profile(CommonSquareMatrixProfile::Zero).values.clone();
        for i in 0..3 {
            for j in 0..3 {
                values[i][j] = self.values[j][i];
            }
        }
        SquareMatrix3 { values: values }
    }

    /**
     * add two valid 3x3 square matrices.
     */
    pub fn add(&self, other: &SquareMatrix3) -> SquareMatrix3 {
        let mut values = vec![
            vec![0., 0., 0.],
            vec![0., 0., 0.],
            vec![0., 0., 0.],
        ];
        for i in 0..3 {
            for j in 0..3 {
                values[i][j] = self.values[i][j] + other.values[i][j];
            }
        }
        let result = SquareMatrix3 {
            values: values
        };
        result
    }

    pub fn subtract(&self, other: &SquareMatrix3) -> SquareMatrix3 {
        self.add(&other.negative())
    }

    /**
     * Multiplication of 3x3 square matrices.
     */
    pub fn mult(&self, other: &SquareMatrix3) -> SquareMatrix3 {
        let mut values = create_from_profile(CommonSquareMatrixProfile::Zero).values.clone();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    values[i][j] = values[i][j] + self.values[i][k] * other.values[k][j]
                }
            }
        }
        let result = SquareMatrix3 {
            values: values
        };
        result
    }

    /**
     * Trace of a 3x3 square matrix.
     */
    pub fn trace(&self) -> f64 {
        let mut sum = 0.;
        for i in 0..3 {
            sum += self.values[i][i];
        }
        sum
    }

    /**
     * Determinant of a 3x3 matrix.
     * might need to consider a recursive solution here
     * we'll just use a basic one here.
     * then need to figure out a way to exclude the row/column
     * to extract the submatrix
     */
    pub fn det(&self) -> f64 {
        let mut total: f64 = 0.;
        for i in 0..3_usize {
            let sign: f64;
            if i.checked_rem(2_usize) == Some(0) {
                sign = 1.;
            } else {
                sign = -1.;
            }
            let outer = sign * self.values[0][i];
            let mut submat: Vec<Vec<f64>> = vec![
                Vec::new(),
                Vec::new()
            ];

            // pretty sure this is more complicated than if we just generalized..
            for j in 1..3_usize {
                for k in 0..3_usize {
                    if i == k {
                        continue;
                    }
                    submat[j-1].push(self.values[j][k]);
                }
            }
            let inner = submat[0][0] * submat[1][1] - submat[1][0] * submat[0][1];
            total += outer * inner;
        }
        total
    }

}
