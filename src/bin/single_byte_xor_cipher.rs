use cryptopals::{decode_xor, hex_to_bytes};
use std::io;

fn main() {
    let mut input = String::new();

    println!("Please enter your encoded string: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    let hex = input.trim();

    let bytes = hex_to_bytes(hex);
    let sentence = decode_xor(&bytes);
    println!("{}", sentence);
}
