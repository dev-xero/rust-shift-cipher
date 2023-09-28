use std::error::Error;

use crate::Config;
use crate::cipher;
use crate::write_result;

fn run_encryption(config: &Config,  alphabet: &str) -> Result<(), Box<dyn Error>> {
    let result = cipher::encrypt(&config.text, config.shift, alphabet)?;

    println!("Encrypting '{}'...", &config.text);
    println!("Encrypted: '{}' (shift: {})", result, &config.shift);

    if config.should_export {
        write_result(result, &config.shift)?
    }

    Ok(())
}

fn run_decryption(config: &Config, alphabet: &str) -> Result<(), Box<dyn Error>> {
    
    let result = cipher::decrypt(&config.text, config.shift, alphabet)?;

    println!("Decrypting '{}'...", &config.text);
    println!("Decrypted: '{}' (shift: {})", result, &config.shift);

    Ok(())
}

pub fn with_config(config: Config) -> Result<(), Box<dyn Error>> {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    if !config.should_decrypt {
        run_encryption(&config, alphabet)
    } else {
        run_decryption(&config, alphabet)
    }
}