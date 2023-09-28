use std::error::Error;

use crate::Config;
use crate::cipher;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let result = cipher::encrypt(
        &config.text, 
        config.shift, 
        alphabet
    );

    Ok(())
}