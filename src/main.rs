/*use std::{env::{self, args}, vec};

fn main() {
    println!("Hello, world!");

    let eneble_fib = env::args().nth(1).expect("input is required");
    let max_threshold = env::args().nth(2).expect("input  is required");

    println!( "\n enable_fib: {}  \n max_threshold: {} ", eneble_fib, max_threshold);
}
*/


use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <enable_fib> <max_threshold>", args[0]);
        return;
    }

    let enable_fib = &args[1];
    let max_threshold = &args[2];

    println!("enable_fib: {}", enable_fib);
    println!("max_threshold: {}", max_threshold);

    if enable_fib == "true" {
        // Your Fibonacci logic here
        println!("Fibonacci program is enabled with max threshold: {}", max_threshold);
    } else {
        println!("Fibonacci program is disabled");
    }
}
