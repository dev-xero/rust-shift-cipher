use std::char;

pub fn encrypt(digit: char, shift: i8) -> Result<char, &'static str> {
    let digit = digit.to_digit(10)
        .ok_or("Failed to parse digit")?;

    let shifted_digit = (digit + (shift as u32)) % 10;
    let shifted_digit_char = char::from_digit(shifted_digit as u32, 10)
        .ok_or("Failed to parse digit")?;

    Ok(shifted_digit_char)
}

pub fn decrypt(digit: char, shift: i8) -> Result<char, &'static str> {
    let digit = digit.to_digit(10)
        .ok_or("Failed to parse digit")?;

    let shifted_digit = (digit + 10 - (shift as u32)) % 10;
    let shifted_digit_char = char::from_digit(shifted_digit as u32, 10)
        .ok_or("Failed to parse digit")?;

    Ok(shifted_digit_char)
}