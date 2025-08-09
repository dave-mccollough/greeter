use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();

    let name = if args.len() > 1{
        &args[1..].join(" ")
    } else {
        &"Rustacaen".to_string()
    };

    if name.to_lowercase() == "admin" {
        println!("Hello Administrator!")
    } else {
        println!("Hello, {}! Welcome to Rust", name);
    }
}