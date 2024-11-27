This is a personal Rust library to assist in solving the Cryptopals Crypto Challenges. It includes utilities for format conversions, cryptographic algorithm implementations, and solving challenges related to cryptography. The project aims to deepen my understanding of cryptography while learning Rust.

Example usage:
```Rust
use cryp::{hexstr_to_bytes, bytes_to_hexstr};

fn main() {
    let s = "We started dancing and love put us into a groove";
    println!("String literal: \"{s}\"\n");

    let hex = bytes_to_hexstr(s.as_bytes());
    println!("bytes to hex encoded str: {}", hex);

    let bytes = hexstr_to_bytes(&hex).unwrap();
    println!("vector of bytes: {:?}", bytes);
}
```

- [x] Convert hex to base64
- [x] Fixed XOR
- [x] Single-byte XOR cipher
- [x] Detect single-character XOR
- [ ] Implement repeating-key XOR
- [ ] Break repeating-key XOR
- [ ] AES in ECB mode
- [ ] Detect AES in ECB mode
