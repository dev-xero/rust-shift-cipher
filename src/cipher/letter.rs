pub fn encrypt(letter: char, shift: i8, alphabet: &str) -> Result<char, &'static str> {
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

pub fn decrypt(letter: char, shift: i8, alphabet: &str) -> Result<char, &'static str> {
    if !letter.is_ascii_alphabetic() {
        return Err("Invalid letter encountered, only English letters can be used.");
    }

    let is_upper_case = letter.is_ascii_uppercase();

    let idx = alphabet
        .chars()
        .position(|ch| ch == letter.to_ascii_lowercase())
        .ok_or("Letter not found in the alphabet.")?;

    let shifted_idx = (idx + 26 - shift as usize) % 26;

    let shifted_letter = alphabet
        .chars()
        .nth(shifted_idx)
        .unwrap();

    if is_upper_case { Ok(shifted_letter.to_ascii_uppercase()) } else { Ok(shifted_letter) }
}