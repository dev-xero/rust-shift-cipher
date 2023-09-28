mod letter;
mod digit;

pub fn encrypt(text: &String, shift: i8, alphabet: &str) -> Result<String, &'static str> {
    let mut encrypted_text = String::with_capacity(text.len());

    for ch in text.chars() {
        if ch.is_numeric() {
            let encrypted_digit = match digit::encrypt(ch, shift) {
                Ok(res) => res,
                Err(err) => return Err(err)
            };

            encrypted_text.push(encrypted_digit);
        } else {
            let encrypted_letter = match letter::encrypt(ch, shift, alphabet) {
                Ok(res) => res,
                Err(err) => return Err(err)
            };

            encrypted_text.push(encrypted_letter);
        }
    }

    Ok(encrypted_text)
}

pub fn decrypt(text: &String, shift: i8, alphabet: &str) -> Result<String, &'static str> {
    let mut decrypted_text = String::with_capacity(text.len());

    for ch in text.chars() {
        if ch.is_numeric() {
            let decrypted_digit = match digit::decrypt(ch, shift) {
                Ok(res) => res,
                Err(err) => return Err(err)
            };

            decrypted_text.push(decrypted_digit);
        } else {
            let decrypted_letter = match letter::decrypt(ch, shift, alphabet) {
                Ok(res) => res,
                Err(err) => return Err(err)
            };

            decrypted_text.push(decrypted_letter);
        }
    }

    Ok(decrypted_text)
}