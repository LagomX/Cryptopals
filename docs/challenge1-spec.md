# Cryptopals Set 1 · Challenge 1 Specification

> Goal: Use this document to write the program yourself from scratch. Do not
> copy the existing implementation; return to it only when you get stuck.

---

## 1. The Problem

Convert a piece of **hexadecimal text** into **Base64 text**.

Input—a hex string:

```text
49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
```

Expected output—a Base64 string:

```text
SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
```

## 2. Core Idea

The input is text, not a number. The conversion has two stages, with a byte
sequence in between:

```text
hex text ──parse──► byte sequence (Vec<u8>) ──encode──► Base64 text
```

- One hex character represents 4 bits—half a byte—so **two hex characters
  form one byte**.
- One Base64 character represents 6 bits, so **three bytes (24 bits) become
  four Base64 characters**.

---

## 3. Functions to Implement

### 3.1 `fn hex_digit(c: u8) -> u8`

**Responsibility:** Convert one ASCII hex character into its numeric value
from 0 through 15.

**Input:** A byte containing the ASCII value of a hex character. For example,
the character `'4'` is passed as `52`.

**Output:** A numeric value from 0 through 15.

**Steps:**

- If `c` is between `'0'` and `'9'`, return `c - value of '0'`.
- If `c` is between `'a'` and `'f'`, return `c - value of 'a' + 10`.
- Otherwise, report an error; using `panic!` is sufficient for this challenge.

**Hint:** Use `match`. Byte literals are written as `b'0'` and `b'a'`, while
an inclusive range is written as `b'0'..=b'9'`.

---

### 3.2 `fn hex_to_bytes(hex: &str) -> Vec<u8>`

**Responsibility:** Parse an entire hex string into a byte sequence.

**Steps:**

1. Create an empty mutable `Vec<u8>`.
2. Use `hex.as_bytes()` to access the underlying byte slice by index.
3. Iterate over the indexes in steps of two with
   `(0..length).step_by(2)`.
4. During each iteration:
   - `high = hex_digit(character at i)`
   - `low = hex_digit(character at i + 1)`
   - Combine them into one byte with `(high << 4) | low`.
   - Push that byte into the vector.
5. Return the vector.

**Verification point:** `hex_to_bytes("49276d")` should produce
`[73, 39, 109]`.

---

### 3.3 `fn bytes_to_base64(bytes: &[u8]) -> String`

**Responsibility:** Encode a byte sequence as Base64 text.

**Preparation:** Create a 64-character lookup table. Indexes 0 through 63 map
to characters in this order:

```text
A-Z (26 characters), a-z (26 characters), 0-9 (10 characters), +, /
```

**Hint:** Use a byte string literal such as `b"ABC...+/"`. Its type is
`&[u8; 64]` when the table contains exactly 64 characters.

**Steps:** Create an empty `String`, then process the input in groups of three
bytes with `bytes.chunks(3)`:

1. Read the group's bytes as `b0`, `b1`, and `b2`. If fewer than three bytes
   remain, treat the missing values as `0`.
   Check `chunk.len()` before indexing to avoid an out-of-bounds panic. Convert
   each value with `as u32` before shifting.
2. Combine the three bytes into 24 bits:
   `n = (b0 << 16) | (b1 << 8) | b2`.
3. Split those 24 bits into four 6-bit values:
   - `i0 = (n >> 18) & 0b111111`
   - `i1 = (n >> 12) & 0b111111`
   - `i2 = (n >> 6) & 0b111111`
   - `i3 = n & 0b111111`
4. Produce the output characters:
   - Always output `table[i0]` and `table[i1]`.
   - Output `table[i2]` when the chunk has at least two bytes; otherwise,
     output `'='`.
   - Output `table[i3]` when the chunk has three bytes; otherwise, output
     `'='`.
   - Convert indexes with `as usize` and table bytes with `as char`.
5. Return the completed string.

**Verification point:** `bytes_to_base64(&[73, 39, 109])` should produce
`"SSdt"`.

---

### 3.4 `fn main()`

**Steps:**

1. Store or read the input hex string.
2. Call `hex_to_bytes(...)`.
3. Call `bytes_to_base64(&bytes)`.
4. Print the Base64 result.
5. Optionally, compare it with the expected output and print whether it is
   correct.

---

## 4. Rust Syntax Reference

| Task | Syntax |
|------|--------|
| Declare a mutable variable | `let mut x = ...;` |
| Create an empty vector | `Vec::new()` |
| Add an element to a vector | `v.push(x);` |
| Create an empty string | `String::new()` |
| Add a character to a string | `s.push('A');` |
| Access a string as bytes | `s.as_bytes()` → `&[u8]` |
| Process groups of n items | `slice.chunks(n)` |
| Iterate with a step | `(0..n).step_by(2)` |
| Match different cases | `match x { pattern => value, _ => ... }` |
| Convert a value's type | `x as u32`, `x as usize`, `x as char` |
| Write a binary literal | `0b111111` |
| Write a byte literal | `b'0'` |
| Write a byte string | `b"ABC..."` |
| Print a debug representation | `println!("{:?}", x);` |

## 5. How to Run It

From the project directory, run:

```bash
cargo run --bin hex_to_base64
```

If the output matches the expected result from Section 1, the challenge is
complete.

## 6. If You Get Stuck

- Read Rust compiler errors carefully; they usually identify both the location
  and the nature of the problem.
- If a function feels too large, implement and verify only one function at a
  time using the small verification examples above.
- Check that you are converting hex text into raw bytes before attempting the
  Base64 encoding.
- When debugging the final chunk, test inputs containing one, two, and three
  bytes so you can verify the `=` padding behavior.
