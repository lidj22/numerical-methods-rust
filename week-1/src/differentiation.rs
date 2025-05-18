pub fn derivative(f: fn(f64) -> f64, x: f64, step_size: f64) -> f64 {
    (f(x + step_size) - f(x)) / step_size
}