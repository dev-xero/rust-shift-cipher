# Rust Shift Cipher
A shift cipher (or caesar cipher) is a fundamental cryptographic algorithm based on the concept of shifting the letters in a word by a specified number of times for encryption.  

Due to its uncomplicated nature, it proves particularly useful for safeguarding simple passwords and tokens.

## Usage
```sh
.\main password 8 -e
```

Two arguments must be supplied to the program in order for it to work,
1. The text, you want to encrypt, and
2. The shift distance

The last argument, `-e` is optional, when set, the program will write the result of a successful encryption to a plain text file named: `decryption_key_[encrypted text]-[shift distance].txt`. Which contains the encrypted text and the negative shift distance to decrypt the text.

## Example
Assume we have some password we want to encrypt, as a result of our clever thinking, we come up with `password` (don't ever use "password" as a password, at least without encrypting it), applying the shift cipher algorithm to it using a shift distance of `8`, we get:
- Original: `p` `a` `s` `s` `w` `o` `r` `d`
- Encrypted (shift distance: 8): `x` `i` `a` `a` `e` `w` `z` `l`
