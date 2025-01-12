This is a personal Rust library to assist in solving the Cryptopals Crypto Challenges. It includes utilities for format conversions, cryptographic algorithm implementations, and solving challenges related to cryptography. The project aims to deepen my understanding of cryptography while learning Rust.

Example usage:
```Rust
// Break repeating-key xor

use std::fs;
use cryp::{ xor_one_byte, xor_repeating, xor_guess_key,
            xor_guess_key_len, hamming_distance_bit, 
            base64_decode, score_text
        };

fn main() {
    let encoded_data = fs::read_to_string("6.txt").expect("Failed to read file");
    let data = base64_decode(&encoded_data.replace('\n', "")).expect("Invalid base64 input");

    let key_size = xor_guess_key_len(&data, 2, 40);
    println!("Probable key size: {}", key_size);

    let guessed_key = xor_guess_key(&data, key_size);
    let guessed_key_str = String::from_utf8(guessed_key.clone()).unwrap();
    println!("The guessed key is: {}", guessed_key_str);

    let decoded = xor_repeating(&data, &guessed_key);
    println!("{}", String::from_utf8_lossy(&decoded));
}
```
- [x] Convert hex to base64
- [x] Fixed XOR
- [x] Single-byte XOR cipher
- [x] Detect single-character XOR
- [x] Implement repeating-key XOR
- [x] Break repeating-key XOR
- [x] AES in ECB mode
- [x] Detect AES in ECB mode
