/// Single-byte XOR cipher
/// https://cryptopals.com/sets/1/challenges/3

use cryp::{
    hexstr_to_bytes,
    xor_one_byte,
    score_text
};

fn main() {
    let s1c3input = hexstr_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    let mut score: f32;
    for i in 0..255 {
        score = score_text(&xor_one_byte(&s1c3input, i));
        if score > 80.0 {
            println!("score:{} | key:{}={} | string:{:?}", score, i, i as char, String::from_utf8_lossy(&xor_one_byte(&s1c3input, i)));
        }
    }    
}