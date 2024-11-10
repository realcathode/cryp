

This is a personal Rust library created to help me solve the Cryptopals Crypto Challenges. The library includes functions to solve various challenges related to cryptography, including converting between different formats, implementing cryptographic algorithms, and solving attacks on real-world crypto systems. The goal is to deepen my understanding of cryptographic concepts and how they can be attacked or exploited.


The library contains functions designed to solve specific Cryptopals challenges. Here's a simple example demonstrating how to use the hex_to_base64 function for the first challenge in the first set.

```Rust
use cryp;

fn main() {
    let s = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

    if let Ok(a) = cryp::hex_to_base64(&s) {
        println!("HEX: {}\nBASE64: {}", s, a);
    }
}
```
