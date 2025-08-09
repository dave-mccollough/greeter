use std::io;
use chrono::{Utc};

fn main(){
    // let args: Vec<String> = env::args().collect();

    let mut name = String::new();
    let now = Utc::now();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    if !name.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
        eprintln!("Error: Name must only contain letters or spaces");
        std::process::exit(1);
    }

    if name.to_lowercase() == "admin" {
        println!("Hello Administrator!")
    } else if !name.is_empty() {
        println!("Hello, {}.. the time is {}! Welcome to Rust", name.trim().to_uppercase(), now);
    } else {
        println!("Please enter your name!")
    }
}