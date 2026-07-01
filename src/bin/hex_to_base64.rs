use std::io;
use cryptopals::{hex_digit, hex_to_bytes, bytes_to_base64};

fn main() {
    let mut input = String::new();
    println!("Please enter hex: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let hex = input.trim();
    let bytes = hex_to_bytes(&hex);
    let base64 = bytes_to_base64(&bytes);
    println!("{}", base64);
}