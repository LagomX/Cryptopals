# Cryptopals

使用 Rust 完成 【Cryptopals Crypto Challenges】(https://cryptopals.com)

## 运行环境

- Rust

## 运行方式

例如运行 Challenge1 hex_to_base64

```bash
cargo run --bin hex_to_base64
```

## 挑战进度

- [x] Challenge 1：Convert hex to base64

- [ ] Challenge 2：Fixed XOR

- [ ] Challenge 3：Single-byte XOR cipher

- [ ] Challenge 4：Detect single-character XOR

- [ ] Challenge 5：Implement repeating-key XOR

- [ ] Challenge 6：Break repeating-key XOR

- [ ] Challenge 7：AES in ECB mode

- [ ] Challenge 8：Detect AES in ECB mode

## 项目结构
```test
src/
├── lib.rs
└── bin/
    └── hex_to_base64.rs


- src/lib.rs：可复用的编码和加密函数
- src/bin/：每个可执行 Challenge 的入口