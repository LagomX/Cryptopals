use cryptopals::{hex_to_bytes, score_byte};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("docs/4.txt")?;
    let reader = BufReader::new(file);

    let mut best_score = i32::MIN;
    let mut target_line_number = 0;
    let mut real_key = b' ';
    let mut result: Vec<u8> = Vec::new();

    for (line_number, line_result) in reader.lines().enumerate() {
        let hex = line_result?;
        let bytes = hex_to_bytes(&hex);

        for key in 0u8..=255 {
            let mut score = 0;
            for byte in &bytes {
                let decrypted_byte = *byte ^ key;
                score += score_byte(decrypted_byte);
            }
            if score > best_score {
                best_score = score;
                real_key = key;
                target_line_number = line_number;
                result = bytes.clone();
            }
        }
    }

    let mut plaintext = Vec::new();
    for &byte in &result {
        let decrypted_byte = byte ^ real_key;
        plaintext.push(decrypted_byte);
    }

    let plaintext = String::from_utf8_lossy(&plaintext);
    println!("line: {}", target_line_number + 1);
    println!("key: {}", real_key);
    println!("score: {}", best_score);
    println!("plaintext: {}", plaintext);

    Ok(())
}
