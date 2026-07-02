/// Converts one ASCII hexadecimal digit to its numeric value.
///
/// # Panics
///
/// Panics if `c` is not `0-9`, `a-f`, or `A-F`.
pub fn hex_digit(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("Invalid hex digit: {}", c as char),
    }
}

/// Decodes a hexadecimal string into raw bytes.
///
/// # Panics
///
/// Panics if the input has an odd length or contains an invalid hex digit.
pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let chars = hex.as_bytes();
    for i in (0..chars.len()).step_by(2) {
        let high = hex_digit(chars[i]) as u8;
        let low = hex_digit(chars[i + 1]) as u8;
        let byte = (high << 4) | low;
        bytes.push(byte);
    }
    bytes
}

/// Encodes raw bytes as a standard padded Base64 string.
pub fn bytes_to_base64(bytes: &[u8]) -> String {
    let base64 = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };

        let n = (b0 << 16) | (b1 << 8) | b2;

        let i0 = (n >> 18) & 0b111111;
        let i1 = (n >> 12) & 0b111111;
        let i2 = (n >> 6) & 0b111111;
        let i3 = n & 0b111111;

        result.push(base64[i0 as usize] as char);
        result.push(base64[i1 as usize] as char);

        if chunk.len() > 1 {
            result.push(base64[i2 as usize] as char)
        } else {
            result.push('=')
        };
        if chunk.len() > 2 {
            result.push(base64[i3 as usize] as char)
        } else {
            result.push('=')
        };
    }

    result
}

/// XORs two equal-length byte slices and returns the resulting bytes.
///
/// # Panics
///
/// Panics if the two input slices have different lengths.
pub fn fixed_xor(left: &[u8], right: &[u8]) -> Vec<u8> {
    assert_eq!(left.len(), right.len(), "Input must have the same length");

    let mut result = Vec::new();

    for (a, b) in left.iter().zip(right.iter()) {
        let xor_result = *a ^ *b;
        result.push(xor_result);
    }

    result
}

/// Encodes raw bytes as a lowercase hexadecimal string.
///
/// Each byte is split into its high and low four-bit values, which are used
/// as indexes into the hexadecimal character table.
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let table = b"0123456789abcdef";
    let mut result = String::new();

    for byte in bytes {
        let b0 = byte >> 4;
        let b1 = byte & 0b1111;

        result.push(table[b0 as usize] as char);
        result.push(table[b1 as usize] as char);
    }

    result
}
