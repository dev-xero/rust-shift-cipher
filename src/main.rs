use std::{env, process::exit};

use rust_shift_cipher::Config;

fn main() {
    let config = Config::build(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            exit(1);
        });
    
    println!("{}", config.text);
    println!("{}", config.shift);
    println!("{}", config.should_export);
}
