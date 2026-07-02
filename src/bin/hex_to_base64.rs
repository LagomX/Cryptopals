use cryptopals::{bytes_to_base64, hex_to_bytes};
use std::io;

/// Reads a hexadecimal string from standard input and prints its Base64 form.
fn main() {
    let mut input = String::new();
    println!("Please enter hex: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let hex = input.trim();
    let bytes = hex_to_bytes(hex);
    let base64 = bytes_to_base64(&bytes);
    println!("{}", base64);
}
