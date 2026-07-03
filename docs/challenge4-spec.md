# Cryptopals Set 1 · Challenge 4 Specification

> Goal: Detect which hexadecimal line contains a message encrypted with
> single-byte XOR, then recover its key and English plaintext.

---

## 1. The Problem

The input file `docs/4.txt` contains many independent hexadecimal strings.
Exactly one line is an English plaintext encrypted by XORing every byte with
the same single-byte key. The remaining lines are random-looking decoys.

Find:

- the encrypted line;
- the single-byte XOR key;
- the recovered English plaintext.

Hexadecimal is only the text representation of each byte sequence. It is not
the encryption itself, and the file is not one continuous article.

---

## 2. Core Idea

Challenge 3 searched two dimensions:

```text
one ciphertext × every possible key
```

Challenge 4 adds another dimension:

```text
every line × every possible key
```

For each line:

1. Decode the hexadecimal text into raw bytes.
2. Try every key from `0` through `255`.
3. XOR every byte with the current key.
4. Score the resulting bytes as English.
5. Compare the score with the best score seen across the entire file.
6. When a higher score is found, save the line, key, score, and ciphertext.

The search can be written mathematically as:

```text
argmax EnglishScore(line XOR key)
line,key
```

The complete data flow is:

```text
docs/4.txt
    |
    v
read one hexadecimal line
    |
    v
hex_to_bytes
    |
    v
try keys 0..=255 -> XOR each byte -> score candidate
    |
    v
compare with the global best candidate
    |
    v
best ciphertext + best key -> plaintext bytes -> String
```

---

## 3. Why the Highest Score Is the Candidate

The challenge supplies an important assumption: one line decrypts to English.
When the correct key is tried on that line, the result should contain common
letters, spaces, and punctuation. Incorrect keys and random decoy lines will
usually produce unusual symbols or non-printable bytes.

The score is a statistical heuristic, not a mathematical proof. Random bytes
could occasionally resemble English, and a weak scoring function can choose a
wrong candidate. The challenge data is constructed so that the real plaintext
scores clearly enough with a reasonable English-frequency model.

A key of `0` may be included in the search. XOR with zero leaves the bytes
unchanged, so an unencrypted English line would naturally appear as the
`key = 0` candidate.

---

## 4. State to Track

The program needs global state that survives across both loops:

| Value | Purpose |
|-------|---------|
| `best_score` | Highest English score found so far |
| `real_key` | Key that produced the highest score |
| `target_line_number` | Location of the winning ciphertext |
| `result` | Bytes of the winning ciphertext |

Initialize `best_score` to `i32::MIN`. Some candidates may have negative
scores, so initializing it to zero could prevent any candidate from winning.

When a new best candidate is found, replace all related values together. If
only the score or key is updated, the final key and ciphertext may come from
different candidates.

---

## 5. Program Structure

### 5.1 Open and read the input file

The binary is normally run from the project root, so the relative path is:

```text
docs/4.txt
```

`File::open` returns a `Result<File, std::io::Error>`. A convenient `main`
signature is:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>>
```

This allows `?` to propagate file and line-reading errors.

Wrap the opened file in `BufReader`, then use `lines().enumerate()` to process
one line at a time. Each item returned by `lines()` is also a `Result`, so it
must be handled before passing the string to `hex_to_bytes`.

### 5.2 Search every line and key

Use two nested loops:

```text
for each line
    decode hex into bytes

    for each key from 0 through 255
        reset score to zero

        for each byte in this line
            decrypt byte with XOR
            add its English score

        update the global best candidate if necessary
```

The score must be reset inside the key loop. Each key represents an independent
candidate.

Borrow the byte vector while testing keys. Moving the vector into the first
iteration would make it unavailable for the remaining 255 keys.

### 5.3 Preserve the winning ciphertext

The local `bytes` vector belongs to the current line and is replaced on the
next iteration. When its candidate becomes the global best, save an owned copy
of it.

Do not use `push(bytes)` on a `Vec<u8>`:

- `push` adds one `u8`;
- `bytes` is an entire `Vec<u8>`;
- the desired operation is to replace the previous winning vector.

### 5.4 Build and print the plaintext

After both search loops finish:

1. Create a separate plaintext byte vector.
2. Iterate over the saved ciphertext bytes.
3. XOR each byte with `real_key`.
4. Push each decrypted byte into the plaintext vector.
5. Convert the plaintext bytes with `String::from_utf8_lossy`.
6. Print the line number, key, score, and plaintext.

Keep ciphertext and plaintext in separate vectors. Appending decrypted bytes
to the ciphertext vector mixes two different kinds of data and causes borrowing
problems.

---

## 6. Rust Syntax Reference

| Task | Syntax |
|------|--------|
| Open a file | `File::open("docs/4.txt")?` |
| Buffered file reader | `BufReader::new(file)` |
| Read numbered lines | `reader.lines().enumerate()` |
| Extract a successful line | `let hex = line_result?;` |
| Borrow a `String` as `&str` | `&hex` |
| Try every byte key | `for key in 0u8..=255` |
| Borrow vector elements | `for &byte in &bytes` |
| XOR one byte with a key | `byte ^ key` |
| Copy an owned vector | `bytes.clone()` |
| Smallest `i32` | `i32::MIN` |
| Lossy UTF-8 conversion | `String::from_utf8_lossy(&plaintext)` |
| Human-readable line number | `line_number + 1` |

`enumerate()` starts counting at zero. Add one only when displaying a line
number to a person; keep the internal zero-based value if it is used as an
index.

---

## 7. Complexity

Let:

- `L` be the number of lines;
- `N` be the average number of bytes per line;
- `256` be the number of possible keys.

The runtime is:

```text
O(L × 256 × N)
```

Because the key space contains only 256 values, exhaustive search is practical
for this challenge.

The program only needs to retain the current line and the global winning line,
so it does not need to load the entire file into memory.

---

## 8. Verification Checklist

Before considering the challenge complete, confirm that:

- `docs/4.txt` is opened successfully from the project root.
- Every line is handled as an independent hexadecimal ciphertext.
- Hex text is decoded before XOR is applied.
- Every key from `0` through `255` is tested for every line.
- The score is reset for each key.
- Ciphertext bytes are borrowed rather than consumed during key search.
- The best score, key, line number, and ciphertext are updated together.
- The saved ciphertext is replaced, not appended to the previous winner.
- The final plaintext is built in a separate byte vector.
- The output is a readable English sentence.

---

## 9. How to Run It

With the binary under `src/bin/c4.rs`, run:

```bash
cargo run --bin c4
```

Check that the project compiles without running it:

```bash
cargo check --bin c4
```

Run the full test suite with:

```bash
cargo test
```

---

## 10. If You Get Stuck

- If `BufReader` says `Result<File, Error>` does not implement `Read`, handle
  the result returned by `File::open` before constructing the reader.
- If `?` cannot be used in `main`, give `main` a compatible `Result` return
  type and return `Ok(())` at the end.
- If `hex_to_bytes` expects `&str`, pass a reference to the line's `String`.
- If the byte vector is moved during the first key attempt, iterate over a
  reference to it.
- If `push` expects `u8` but receives `Vec<u8>`, distinguish appending one byte
  from replacing the complete winning vector.
- If Rust rejects `Vec<u8> ^ u8`, apply XOR to each byte individually.
- If the program compiles but prints nothing, add the final plaintext
  conversion and output after both search loops.
- If the output is close but the line number seems off by one, remember that
  `enumerate()` starts at zero.
