use std::error::Error;

use crate::Config;
use crate::cipher;

pub fn with_config(config: Config) -> Result<(), Box<dyn Error>> {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let result = cipher::encrypt(&config.text, config.shift, alphabet)?;

    println!("Encrypting '{}'...", &config.text);
    println!("Encrypted: '{}' (shift: {})", result, &config.shift);

    Ok(())
}