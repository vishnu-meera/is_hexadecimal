use std::env;
use std::process;


fn is_value_hex(value: u32) -> bool {
    (value >= 97 && value<=102) || (value >=65 && value <=70) || (value >=48 && value <=57)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Error: No argument provided");
        process::exit(1);
    }

    if args.len() > 2 {
        eprintln!("Error: Too many arguments");
        process::exit(1);
    }

    let input = &args[1];

    let is_hex: bool = match input.parse::<u32>() {
        Ok(value) => is_value_hex(value),
        Err(_)=>{
            if let Some(first_char)= input.chars().next() {
                is_value_hex(first_char as u32)
            } else {
                false
            }
        }
    };

    if is_hex {
        println!("given argument is a hexadecimal");
    } else {
        println!("given argument is not a hexadecimal");
    
    }
}
