// we should know how to cast

use std::f64::consts::PI;

fn add_one(x: f64) -> f64 {
    x + 1.
}

fn sine(x: f64) -> f64 {
    x.sin()
}

fn derivative(f: fn(f64) -> f64, x: f64, step_size: f64) -> f64 {
    (f(x + step_size) - f(x)) / step_size
}

fn integrate(f: fn(f64) -> f64, a: f64, b: f64, step_size: f64) -> f64 {
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

fn main() {

    const STEP_SIZE: f64 = 0.000001;

    /*
    Derivative of f(x) = x + 1 at x = 0
    with step size = 0.1
    result: 1.
     */

    let result_1: f64 = derivative(add_one, 0., STEP_SIZE);
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
    
    /*
    Derivative of f(x) = x + 1 at x = 0.
    With step size 0.1, 0.01, ... , 0.0000000001.
    Although the expected value should be 1, we will see deviations demonstrated below.
    Here we will see the impact of rounding error vs truncation error.
     */
    let mut step_sizes_4: Vec<f64> = Vec::new();
    let mut results_4: Vec<f64> = Vec::new();
    let mut errors_4: Vec<f64> = Vec::new();
    for step_size_exp in (-20..-1).rev() {
        let step_size: f64 = 10.0_f64.powi(step_size_exp);
        let result_4: f64 = derivative(add_one, 0., step_size);
        let error = (1.0 - result_4).abs();
        results_4.push(result_4);
        step_sizes_4.push(step_size);
        errors_4.push(error);
    }
    // println!("step sizes: {:?}", step_sizes_4);
    // println!("results: {:?}", results_4);
    // println!("errors: {:?}", errors_4);

    println!("{:<40} {:<40} {:<40}", "Step Size", "Result", "Error");
    println!("{:-<40} {:-<40} {:-<40}", "", "", "");
    for i in 0..step_sizes_4.len() {
        println!("{:<40e} {:<40.20} {:<40.20}", step_sizes_4[i], results_4[i], errors_4[i]);
    }

        /*
    Derivative of f(x) = sin(x) at x = 0.
    With step size 0.1, 0.01, ... , 0.0000000001.
    Although the expected value should be 1, we will see deviations demonstrated below.
    Here we will see the impact of rounding error vs truncation error.
     */
    let mut step_sizes_5: Vec<f64> = Vec::new();
    let mut results_5: Vec<f64> = Vec::new();
    let mut errors_5: Vec<f64> = Vec::new();
    for step_size_exp in (-20..-1).rev() {
        let step_size: f64 = 10.0_f64.powi(step_size_exp);
        let result_5: f64 = derivative(sine, 0., step_size);
        let error = (1.0 - result_5).abs();
        results_5.push(result_5);
        step_sizes_5.push(step_size);
        errors_5.push(error);
    }

    println!("{:<40} {:<40} {:<40}", "Step Size", "Result", "Error");
    println!("{:-<40} {:-<40} {:-<40}", "", "", "");
    for i in 0..step_sizes_4.len() {
        println!("{:<40e} {:<40.20} {:<40.20}", step_sizes_5[i], results_5[i], errors_5[i]);
    }

    // TODO: a good exercise would be to explain why error converges to 1 for x -> x + 1 as h -> 0, whereas
    // for sine function, error is gradually going to zero. However, will this continue to be the case?

}
