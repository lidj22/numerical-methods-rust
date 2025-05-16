// we should know how to cast

use std::f32::consts::PI;

fn add_one(x: f32) -> f32 {
    x + 1.
}

fn sine(x: f32) -> f32 {
    x.sin()
}

fn derivative(f: fn(f32) -> f32, x: f32, step_size: f32) -> f32 {
    (f(x + step_size) - f(x)) / step_size
}

fn integrate(f: fn(f32) -> f32, a: f32, b: f32, step_size: f32) -> f32 {
    // this is either floor or ceil or index add/remove 1 I forgot.
    let n = (( b - a ) / step_size).floor();
    let mut i: f32 = 0.;
    let mut total: f32 = 0.;
    while i < n {
        total = total + step_size * f(a + i * step_size);
        i = i + 1.0;
    }
    total
}

fn main() {

    const STEP_SIZE: f32 = 0.000001;

    /*
    Derivative of f(x) = x + 1 at x = 0
    with step size = 0.1
    result: 1.
     */

    let result_1: f32 = derivative(add_one, 0., STEP_SIZE);
    println!("The derivative of f(x) = x + 1 at x = 0 with h = {} is {}.", STEP_SIZE, result_1);
    
    /*
    Derivative of f(x) = sin(x) at x = pi
    with step size 0.1
    result: -1
     */

    let result_2 = derivative(sine, PI, STEP_SIZE);
    println!("The derivative of f(x) = sin(x) at x = pi with h = {} is approximately {}.", STEP_SIZE, result_2);

    /*
    Integral of f(x) = 2x + 1 over [0, 1] with step size 0.1
    result: 2
     */

    // example of closure
    let result_3 = integrate(| y | 2. * y + 1., 0., 1., STEP_SIZE);
    println!("The integral of f(x) = 2x + 1 over [0, 1] with h = {} is approximately {}.", STEP_SIZE, result_3);

}
