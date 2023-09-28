use std::char;

fn encrypt_letter(letter: char, shift: i8, alphabet: &str) -> Result<char, &'static str> {
    if !letter.is_ascii_alphabetic() {
        return Err("Invalid letter encountered, only English letters can be used.");
    }

    let is_upper_case = letter.is_ascii_uppercase();

    let idx = alphabet
        .chars()
        .position(|ch| ch == letter.to_ascii_lowercase())
        .ok_or("Letter not found in the alphabet.")?;

    let shifted_idx = (idx + shift as usize) % 26;

    let shifted_letter = alphabet
        .chars()
        .nth(shifted_idx)
        .unwrap();

    if is_upper_case { Ok(shifted_letter.to_ascii_uppercase()) } else { Ok(shifted_letter) }
}

fn encrypt_digit(digit: char, shift: i8) -> Result<char, &'static str> {
    let digit = digit.to_digit(10)
        .ok_or("Failed to parse digit")?;

    let shifted_digit = (digit + (shift as u32)) % 10;
    let shifted_digit_char = char::from_digit(shifted_digit as u32, 10)
        .ok_or("Failed to parse digit")?;

    Ok(shifted_digit_char)
}

pub fn encrypt(text: String, shift: i8, alphabet: &str) -> Result<String, &'static str> {
    let mut encrypted_text = String::with_capacity(text.len());

    for ch in text.chars() {
        if ch.is_numeric() {
            let encrypted_digit = match encrypt_digit(ch, shift) {
                Ok(res) => res,
                Err(err) => return Err(err)
            };

            encrypted_text.push(encrypted_digit);
        } else {
            let encrypted_letter = match encrypt_letter(ch, shift, alphabet) {
                Ok(res) => res,
                Err(err) => return Err(err)
            };

            encrypted_text.push(encrypted_letter);
        }
    }

    Ok(encrypted_text)
}