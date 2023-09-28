use std::{env, process};

use rust_shift_cipher::Config;
use rust_shift_cipher::run;

fn main() {
    let config = Config::build(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    
    if let Err(err) = run::with_config(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
