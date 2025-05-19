pub fn integrate(f: fn(f64) -> f64, a: f64, b: f64, step_size: f64) -> f64 {
    // this is either floor or ceil or index add/remove 1 I forgot.
    let n = (( b - a ) / step_size).floor();
    let mut i: f64 = 0.;
    let mut total: f64 = 0.;
    while i < n {
        total = total + step_size * f(a + i * step_size);
        i = i + 1.0;
    }
    total
}