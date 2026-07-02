use cryptopals::{bytes_to_hex, fixed_xor, hex_to_bytes};
use std::io;

/// Reads two hex strings, XORs their decoded bytes, and prints the hex result.
fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();

    println!("Please enter first hex number:");
    io::stdin()
        .read_line(&mut input_a)
        .expect("Failed to read line");
    let hex_a = input_a.trim();

    println!("Please enter second hex number:");
    io::stdin()
        .read_line(&mut input_b)
        .expect("Failed to read line");
    let hex_b = input_b.trim();

    let bytes_a = hex_to_bytes(hex_a);
    let bytes_b = hex_to_bytes(hex_b);

    let xor_result = fixed_xor(&bytes_a, &bytes_b);
    let hex_result = bytes_to_hex(&xor_result);
    println!("{}", hex_result);
}
