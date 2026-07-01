pub fn hex_digit(c :u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("Invalid hex digit: {}", c as char),
    }
}

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

pub fn bytes_to_base64(bytes: &[u8]) -> String {

    let base64 = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else {0};
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else {0};


        let n = ( b0 << 16 ) | ( b1 << 8 ) | b2;

        let i0 = ( n >> 18 ) &0b111111;
        let i1 = ( n >> 12 ) &0b111111;
        let i2 = ( n >> 6 ) &0b111111;
        let i3 = n &0b111111;

        result.push(base64[i0 as usize] as char);
        result.push(base64[i1 as usize] as char);

        if chunk.len() > 1 {
            result.push(base64[i2 as usize] as char)
        } else {
            result.push('=')};
        if chunk.len() > 2 {
            result.push(base64[i3 as usize] as char)
        } else {
            result.push('=')
        };
    }

    result
}