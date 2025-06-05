pub mod errors;
pub mod matrix;
pub mod traits;
pub mod impls;

pub use errors::LinearAlgebraError;
pub use matrix::Matrix;
pub use traits::*;