# Cryptopals Set 1 · Challenge 3 Specification

> Goal: Use this document to implement the challenge yourself. Try not to copy
> a finished solution; return to the hints only when you get stuck.

---

## 1. The Problem

The following hexadecimal string was produced by XORing every plaintext byte
with the same single-byte key:

```text
1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
```

Find the key and recover the English plaintext.

The key is exactly one byte, so it can have any value from `0` through `255`.
The challenge suggests scoring each possible plaintext by how closely it
resembles ordinary English.

---

## 2. Core Idea

XOR is reversible when the same key is applied twice:

```text
ciphertext = plaintext ^ key
plaintext  = ciphertext ^ key
```

This works because:

```text
(A ^ B) ^ B = A
```

The same key byte is applied independently to every byte in the message:

```text
plaintext[0] ^ key = ciphertext[0]
plaintext[1] ^ key = ciphertext[1]
plaintext[2] ^ key = ciphertext[2]
...
```

There are only 256 possible single-byte keys. Instead of deriving the key
mathematically, try every possibility and select the result that looks most
like English.

The complete data flow is:

```text
hex ciphertext
      |
      v
hex_to_bytes
      |
      v
raw ciphertext bytes
      |
      v
try keys 0..=255 -> XOR -> score each candidate
      |
      v
best key -> XOR ciphertext again -> plaintext bytes -> String
```

---

## 3. Scoring English Text

A computer does not automatically know whether a byte sequence is meaningful
English. Give each candidate a numeric score based on its characters.

The scoring rules used by this implementation are:

| Character category | Score per byte |
|--------------------|---------------:|
| Space | 13 |
| `e`, `t`, `a`, `o`, `i`, `n` | 8 |
| `s`, `h`, `r`, `d`, `l`, `u` | 4 |
| Other English letters | 2 |
| `.`, `,`, `'`, `!`, `?` | 1 |
| Other printable ASCII | 0 |
| Non-printable bytes | -20 |

Uppercase and lowercase letters should receive the same score. Convert each
byte to lowercase before matching it against the table.

This is a heuristic rather than a proof. It works because normal English tends
to contain spaces and common letters, while incorrect keys usually produce
control characters, unusual symbols, or non-ASCII bytes.

---

## 4. Functions to Implement

### 4.1 `fn score_byte(byte: u8) -> i32`

**Responsibility:** Assign an English-likeness score to one decrypted byte.

**Steps:**

1. Convert the byte to lowercase with `to_ascii_lowercase()`.
2. Use `match` to identify its character category.
3. Return a positive score for common English characters.
4. Return `0` for printable but unremarkable ASCII characters.
5. Return a strong negative score for non-printable bytes.

Useful byte patterns include:

```rust
b' '                 // one ASCII space
b'e' | b't' | b'a'  // several alternatives
b'a'..=b'z'          // inclusive lowercase-letter range
32..=126             // printable ASCII range
```

The return type is `i32` because scores may be negative.

### 4.2 `fn decode_xor(bytes: &[u8]) -> String`

**Responsibility:** Test all single-byte keys and return the highest-scoring
plaintext.

**First pass: find the best key.**

1. Store a `real_key`, initially `0`.
2. Store `best_score`, initially `i32::MIN` so that even a negative candidate
   can replace it.
3. Iterate over all byte values with `0u8..=255`.
4. For each key:
   - Reset the current candidate's score to `0`.
   - XOR every ciphertext byte with the key.
   - Pass each decrypted byte to `score_byte`.
   - Add the returned value to the current score.
5. If the current score exceeds `best_score`, save both the score and key.

It is important to reset `score` inside the outer loop. Each key must receive
an independent score.

**Second pass: build the plaintext.**

1. Create an empty `Vec<u8>` for the result.
2. Iterate over the ciphertext again.
3. XOR every byte with `real_key`.
4. Push each decrypted byte into the result vector.
5. Convert the completed vector into a `String` with
   `String::from_utf8_lossy(&result).into_owned()`.
6. Return the string.

The byte vector should be completed before it is converted into a string.

### 4.3 `fn main()`

**Responsibility:** Read a hexadecimal ciphertext and connect the existing
conversion function to the single-byte XOR decoder.

**Steps:**

1. Create an empty `String` for user input.
2. Read one line from standard input.
3. Remove the trailing newline with `trim()`.
4. Decode the hexadecimal text with `hex_to_bytes` from Challenge 1.
5. Pass the resulting byte slice to `decode_xor`.
6. Print the recovered sentence.

---

## 5. Rust Syntax Reference

| Task | Syntax |
|------|--------|
| XOR two bytes | `byte ^ key` |
| Inclusive byte range | `0u8..=255` |
| Smallest `i32` value | `i32::MIN` |
| Add to a running score | `score += value;` |
| Iterate over referenced bytes by value | `for &byte in bytes` |
| Convert ASCII byte to lowercase | `byte.to_ascii_lowercase()` |
| Match several alternatives | `b'e' \| b't' \| b'a' => ...` |
| Match an inclusive byte range | `b'a'..=b'z' => ...` |
| Create a byte vector | `Vec::<u8>::new()` |
| Append a byte | `result.push(byte);` |
| Convert possibly invalid UTF-8 lossily | `String::from_utf8_lossy(bytes)` |
| Create an owned `String` | `.into_owned()` |

---

## 6. Verification Checklist

Before considering the challenge complete, check that:

- The hexadecimal input is decoded before XOR is applied.
- XOR operates on raw bytes, not hexadecimal characters.
- Every key from `0` through `255` is tested.
- The candidate score is reset for each new key.
- Uppercase and lowercase English letters are scored equally.
- Non-printable bytes receive a penalty.
- The best key is saved whenever a higher score is found.
- The complete plaintext byte vector is converted into a string only after
  decryption finishes.
- The official ciphertext produces a readable English sentence.

---

## 7. How to Run It

With the binary under `src/bin/single_byte_xor_cipher.rs`, run:

```bash
cargo run --bin single_byte_xor_cipher
```

Paste the official hexadecimal ciphertext when prompted.

Run the full test suite with:

```bash
cargo test
```

---

## 8. If You Get Stuck

- Print each candidate key and score to confirm that all 256 keys are tested.
- If only the final character seems to affect the score, check that you used
  `+=` rather than `=+`.
- If no key replaces the initial key, initialize `best_score` to `i32::MIN`
  instead of `0`.
- If letters score correctly but spaces do not, remember that an ASCII space
  byte literal is `b' '`.
- If the function reports that it expected `String` but found `()`, ensure the
  converted string is the final expression and has no trailing semicolon.
- If the output is Base64 or hexadecimal, stop encoding it. The decrypted raw
  bytes should be interpreted as text with `String::from_utf8_lossy`.
- Verify the scoring and key-search stages separately before debugging the
  final string conversion.
