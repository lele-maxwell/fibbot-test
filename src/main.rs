/*use std::{env::{self, args}, vec};

fn main() {
    println!("Hello, world!");

    let eneble_fib = env::args().nth(1).expect("input is required");
    let max_threshold = env::args().nth(2).expect("input  is required");

    println!( "\n enable_fib: {}  \n max_threshold: {} ", eneble_fib, max_threshold);
}
*/
mod pull_request;

use pull_request::{fetch_all_pull_requests, verify_pull_request};
use reqwest::Error;
use std::env;
use tokio::main;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <enable_fib> <max_threshold>", args[0]);
        return Ok(());
    }

    let enable_fib = &args[1];
    let max_threshold = &args[2];

    println!("\n enable_fib: {}", enable_fib);
    println!("\n max_threshold: {}", max_threshold);

    if enable_fib == "true" {
        // Your Fibonacci logic here
        println!("\n Fibonacci program is enabled with max threshold: {}", max_threshold);
        // Example Fibonacci function call
        println!("Fibonacci sequence up to {}: {:?}", max_threshold, generate_fibonacci(max_threshold.parse().unwrap_or(0)));
    } else {
        println!("\n Fibonacci program is disabled");
    }

    let owner = "your-username"; // Replace with your GitHub username
    let repo = "your-repo";      // Replace with your repository name

    match fetch_all_pull_requests(owner, repo).await {
        Ok(pull_requests) => {
            if pull_requests.is_empty() {
                println!("No pull requests found.");
            } else {
                for pr in pull_requests {
                    if verify_pull_request(&pr) {
                        println!("Title: {}", pr.title);
                        println!("Body: {}", pr.body);
                        println!("URL: {}", pr.html_url);
                    } else {
                        println!("Pull request verification failed for PR #{}", pr.html_url);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch pull requests: {}", e);
        }
    }

    Ok(())
}

fn generate_fibonacci(n: u64) -> Vec<u64> {
    let mut fib = vec![0, 1];
    while fib.last().unwrap() < &n {
        let next = fib[fib.len() - 1] + fib[fib.len() - 2];
        if next > n {
            break;
        }
        fib.push(next);
    }
    fib
}
