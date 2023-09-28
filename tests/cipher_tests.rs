use rust_shift_cipher::cipher;

#[test]
fn test_encrypt() {
    let encrypted_text = "xiaaewzl";
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let test = cipher::encrypt(&String::from("password"), 8, alphabet)
        .unwrap();

    assert_eq!(encrypted_text, test);
}

#[test]
fn test_encrypt_with_digit() {
    let encrypted_text = "xiaaewzl6";
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let test = cipher::encrypt(&String::from("password8"), 8, alphabet)
        .unwrap();

    assert_eq!(encrypted_text, test);
}

#[test]
fn test_encrypt_digits_only() {
    let encrypted_text = "111111111";
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let test = cipher::encrypt(&String::from("999999999"), 2, alphabet)
        .unwrap();

    assert_eq!(encrypted_text, test);
}

#[test]
#[should_panic]
fn test_encrypt_with_invalid_string() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    cipher::encrypt(&String::from("こんにちは"), 8, alphabet)
        .unwrap();
}

#[test]
fn test_decrypt() {
    let decrypted_text = "password";
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let test = cipher::decrypt(&String::from("xiaaewzl"), 8, alphabet)
        .unwrap();

    assert_eq!(decrypted_text, test);
}

#[test]
fn test_decrypt_with_digit() {
    let decrypted_text = "password8";
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let test = cipher::decrypt(&String::from("xiaaewzl6"), 8, alphabet)
        .unwrap();

    assert_eq!(decrypted_text, test);
}

#[test]
fn test_decrypt_digits_only() {
    let decrypted_text = "888888888";
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let test = cipher::decrypt(&String::from("000000000"), 2, alphabet)
        .unwrap();

    assert_eq!(decrypted_text, test);
}

#[test]
fn test_decrypt_single_text() {
    let decrypted_text = "a";
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let test = cipher::decrypt(&String::from("a"), 26, alphabet)
        .unwrap();

    assert_eq!(decrypted_text, test);
}

#[test]
#[should_panic]
fn test_decrypt_with_invalid_string() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    cipher::encrypt(&String::from("こんにちは"), 8, alphabet)
        .unwrap();
}