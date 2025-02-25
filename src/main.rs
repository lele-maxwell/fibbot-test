use std::{env::{self, args}, vec};

fn main() {
    println!("Hello, world!");

    let eneble_fib = env::args().nth(1).expect("input is required");
    let max_threshold = env::args().nth(2).expect("input  is required");

    println!( "\n enable_fib: {}  \n max_threshold: {} ", eneble_fib, max_threshold);
}
