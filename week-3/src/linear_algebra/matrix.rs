#[derive(Clone, Copy, Debug)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    pub values: [[T; COLS]; ROWS]
}
