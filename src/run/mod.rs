use std::error::Error;

use crate::Config;
use crate::cipher;
use crate::write_result;

pub fn with_config(config: Config) -> Result<(), Box<dyn Error>> {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let result = cipher::encrypt(&config.text, config.shift, alphabet)?;

    println!("Encrypting '{}'...", &config.text);
    println!("Encrypted: '{}' (shift: {})", result, &config.shift);

    if config.should_export {
        write_result(result, &config.shift)?
    }

    Ok(())
}