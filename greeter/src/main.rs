use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();

    let name = if args.len() > 1{
        &args[1]
    } else {
        "Rustacaen"
    };

    println!("Hello {}! Welcome to Rust", name)
}