#![cfg_attr(feature = "cargo-clippy", feature(tool_lints))]

// Eqution:
// y = (x + 5)^2
//   = x^2 + 2 * 5 x + 5 ^ 2
//   = x^2 + 10x + 25

// (d/dx)y = 2x + 10 = 0
//                 x = -5

// local minima (expected answer): -5

fn main() {
    // Let Initial Input to 3
    let mut x = 3.0;
    // learning rate is to alter the rate of change
    let learning_rate = 0.01;
    let mut counter = 0;
    let mut step = 1.0;
    let precision = 0.000_000_1;
    let loss_fn = |x: f32| -> f32 { 2.0 * x + 10.0 }; // gradient
    loop {
        if step > precision && counter < 100000 {
            let new_x = x - loss_fn(x) * learning_rate;
            counter = counter + 1;
            step = (new_x - x).abs();
            println!("Iteration {} : x is value -> {}", counter, new_x);
            x = new_x;
        } else {
            break
        }
    }

    println!("Good bye, world!");
}
