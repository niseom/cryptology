/**
 * Author: NISEOM
 * Date: 2023
 * Description: This Rust code implements a Caesar cipher encryption and decryption program that
 * shifts each character in a message by a given number of places down the alphabet. The program
 * takes a message, a key, and a mode as input, and outputs the encrypted or decrypted message
 * depending on the mode. The implementation uses a set of symbols that can be encrypted,
 * including uppercase and lowercase letters, digits, and some punctuation marks. This
 * implementation is designed to be simple and easy to use, but it may not handle all edge cases
 * or be suitable for secure cryptographic purposes.
 */

fn caesar_cipher(message: &str, key: usize, mode: &str) -> String {
    let symbols = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";
    let mut translated: String = String::new();
    
    for symbol in message.chars() {
        if symbols.contains(symbol) {
            let symbol_index = symbols.find(symbol).unwrap();
            let translated_index = match mode {
                "encrypt" => (symbol_index + key) % symbols.len(),
                "decrypt" => (symbol_index + symbols.len() - key % symbols.len()) % symbols.len(),
                _ => panic!("Invalid mode"),
            };
            let char_translated = symbols.chars().nth(translated_index).unwrap();
            translated.push(char_translated);
        } else {
            translated.push(symbol);
        }
    }
    return translated;
}