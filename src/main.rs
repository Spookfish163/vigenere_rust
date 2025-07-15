use std::io;

enum Mode {
    Encrypt,
    Decrypt,
}

fn main() {
    let mut menu_input = String::new();

    println!("-- Vigenere encryption utility --");
    println!("Menu:");
    println!("1 - encrypt text");
    println!("2 - decrypt text");

    io::stdin()
        .read_line(&mut menu_input)
        .expect("Failed to read line.");

    match menu_input.trim().parse() {
        Ok(1) => { encrypt(); },
        Ok(2) => { decrypt(); },
        _ => {println!("Invalid input.");
        },
    };
}

fn encrypt() {
    println!("Enter the text to encrypt:");
    let text = accept_input();
    println!("Enter the key to encrypt with:");
    let key = accept_input();
    
    let text_bytes = to_bytes(&text);
    let key_bytes = to_bytes(&key);

    let cipher_bytes = apply_vigenere(text_bytes, key_bytes, Mode::Encrypt);
    let cipher_text = to_ascii(cipher_bytes);
    println!("Here's your ciphertext:");
    println!("{}", cipher_text);
}

fn decrypt() {
    println!("Enter the text to decrypt:");
    let text = accept_input();
    println!("Enter the key to decrypt with:");
    let key = accept_input();
    
    let text_bytes = to_bytes(&text);
    let key_bytes = to_bytes(&key);

    let plain_bytes = apply_vigenere(text_bytes, key_bytes, Mode::Decrypt);
    let plain_text = to_ascii(plain_bytes);
    println!("Here's your plaintext:");
    println!("{}", plain_text);
}

fn accept_input() -> String {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");
    str::make_ascii_lowercase(&mut user_input);
    user_input
}

fn apply_vigenere(text_bytes:Vec<u8>, key_bytes:Vec<u8>, mode: Mode) -> Vec<u8> {
    let mut cipher_bytes = Vec::new();
    let text_len = text_bytes.len();
    let key_len = key_bytes.len();
    let mut j = 0;

    for i in 0..text_len {
        let a = text_bytes[i] - 96;
        let b = key_bytes[j] - 96;
        match mode {
            Mode::Encrypt => {
                let c = ((a + b) % 26) + 96;
                cipher_bytes.push(c);
            }
            Mode::Decrypt => {
                let c = ((a + 26 - b) % 26) + 96;
                cipher_bytes.push(c);
            }
        }

        j += 1;
        if j >= key_len {
            j = 0;
        }
    }
    cipher_bytes
}

fn to_bytes(ascii:&str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for i in ascii.bytes() {
        if i > 96 && i < 123 {
            bytes.push(i)
        }
    }
    bytes
}

fn to_ascii(bytes:Vec<u8>) -> String {
    let ascii = String::from_utf8(bytes).unwrap();
    ascii
}
