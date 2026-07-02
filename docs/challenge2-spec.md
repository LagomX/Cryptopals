# Cryptopals Set 1 · Challenge 2 Specification

> Goal: Use this document to implement the challenge yourself. Try not to copy
> a finished solution; return to the hints only when you get stuck.

---

## 1. The Problem

Take two equal-length buffers and produce their XOR combination.

First input, written as hexadecimal text:

```text
1c0111001f010100061a024b53535009181c
```

Second input, also written as hexadecimal text:

```text
686974207468652062756c6c277320657965
```

Expected hexadecimal output:

```text
746865206b696420646f6e277420706c6179
```

The two inputs must contain the same number of bytes.

---

## 2. What XOR Does

XOR means “exclusive OR.” It compares two bits and returns `1` when they are
different, or `0` when they are the same.

| A | B | A XOR B |
|---|---|---------|
| 0 | 0 | 0 |
| 0 | 1 | 1 |
| 1 | 0 | 1 |
| 1 | 1 | 0 |

For example:

```text
  1010
^ 1100
------
  0110
```

Rust uses the `^` operator for XOR.

Some useful XOR properties are:

```text
A ^ 0 = A
A ^ A = 0
A ^ B = B ^ A
(A ^ B) ^ B = A
```

The last property is one reason XOR appears so often in cryptography: applying
the same key a second time reverses the operation.

---

## 3. Work With Raw Bytes

Do not XOR the visible hexadecimal characters themselves. Hexadecimal is only
a text representation of bytes.

The complete data flow is:

```text
hex text A ──decode──► bytes A ──┐
                                ├── byte-by-byte XOR ──► result bytes ──encode──► hex text
hex text B ──decode──► bytes B ──┘
```

Challenge 1 already provides `hex_to_bytes`, so you can reuse it for both
inputs. You will also need to convert the result bytes back into hexadecimal
text.

### First-byte example

The first byte of each input is `1c` and `68`:

```text
0x1c = 00011100
0x68 = 01101000
       --------
XOR  = 01110100 = 0x74
```

Therefore, the first output byte is `74`.

---

## 4. Functions to Implement

### 4.1 `fn fixed_xor(left: &[u8], right: &[u8]) -> Vec<u8>`

**Responsibility:** XOR two equal-length byte slices and return the resulting
bytes.

**Inputs:**

- `left`: the first byte slice.
- `right`: the second byte slice.

**Output:** A new `Vec<u8>` containing one XOR result for every pair of input
bytes.

**Steps:**

1. Confirm that `left` and `right` have the same length.
2. Create an empty result vector.
3. Visit both slices one byte at a time.
4. XOR the two current bytes with `^`.
5. Push the result into the vector.
6. Return the vector.

**Small verification case:**

```text
left:    [0x1c, 0x01]
right:   [0x68, 0x69]
result:  [0x74, 0x68]
```

Decide what your function should do when the lengths differ. For this
challenge, an assertion or panic with a clear message is sufficient.

### 4.2 `fn bytes_to_hex(bytes: &[u8]) -> String`

**Responsibility:** Encode raw bytes as lowercase hexadecimal text.

Each byte produces exactly two hex characters:

- The upper four bits select the first character.
- The lower four bits select the second character.

You can use this lowercase lookup table:

```text
0123456789abcdef
```

For each byte:

1. Obtain its upper four bits by shifting right by four.
2. Obtain its lower four bits with a `0b1111` mask.
3. Use both values as indexes into the lookup table.
4. Append both selected characters to the output string.

**Verification point:**

```text
[0x74, 0x68, 0x65] -> "746865"
```

### 4.3 `fn main()`

**Responsibility:** Connect the conversion and XOR steps.

Suggested steps:

1. Store or read the two hexadecimal input strings.
2. Decode both with `hex_to_bytes`.
3. Pass the two byte slices to `fixed_xor`.
4. Encode the result with `bytes_to_hex`.
5. Print the hexadecimal result.
6. Compare it with the expected output.

---

## 5. Rust Syntax Reference

| Task | Syntax |
|------|--------|
| XOR two values | `a ^ b` |
| Compare lengths | `left.len() == right.len()` |
| Assert a condition | `assert!(condition)` |
| Assert equality | `assert_eq!(left, right)` |
| Pair two iterators | `left.iter().zip(right.iter())` |
| Add to a vector | `result.push(value)` |
| Shift right four bits | `byte >> 4` |
| Keep the lower four bits | `byte & 0b1111` |
| Convert to an index | `value as usize` |
| Convert a byte to a character | `value as char` |

You can solve the iteration either with indexes or with `zip`. Both approaches
are valid; choose the one you understand best.

---

## 6. Verification Checklist

Before considering the challenge complete, check that:

- Both hexadecimal inputs are decoded before XOR is applied.
- XOR operates on raw bytes, not hexadecimal characters.
- Unequal input lengths are rejected instead of silently truncated.
- Every output byte becomes exactly two lowercase hexadecimal characters.
- The official inputs produce:

```text
746865206b696420646f6e277420706c6179
```

As an optional debugging step, decode the expected output as ASCII. It reads:

```text
the kid don't play
```

---

## 7. How to Run It

If you create a binary named `fixed_xor.rs` under `src/bin/`, run it with:

```bash
cargo run --bin fixed_xor
```

Run the test suite with:

```bash
cargo test
```

---

## 8. If You Get Stuck

- Print the decoded byte vectors with `{:?}` and confirm that each begins with
  `0x1c` and `0x68` respectively.
- Test only the first byte and verify that `0x1c ^ 0x68 == 0x74`.
- If the output has the correct length but wrong characters, inspect
  `bytes_to_hex`.
- If the output is twice as long or otherwise unusual, make sure you are not
  XORing the ASCII bytes of the hex text.
- Build and verify one function at a time using the small examples above.
