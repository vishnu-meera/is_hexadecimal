use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <string>");
        process::exit(1);
    }

    let input: &String = &args[1];

    let is_hex: bool = input.chars().all(|c| c.is_ascii_hexdigit());

    if is_hex {
        println!("given argument is a hexadecimal");
    } else {
        println!("given argument is not a hexadecimal");
    
    }
}
