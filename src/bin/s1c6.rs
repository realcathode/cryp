use std::fs;
use cryp::{ xor_repeating, xor_guess_key,
            xor_guess_key_len, 
            base64_decode,
        };

fn main() {
    let encoded_data = fs::read_to_string("../../input/6.txt").expect("Failed to read file");
    let data = base64_decode(&encoded_data.replace('\n', "")).expect("Invalid base64 input");

    let key_size = xor_guess_key_len(&data, 2, 40);
    println!("Probable key size: {}", key_size);

    let guessed_key = xor_guess_key(&data, key_size);
    let guessed_key_str = String::from_utf8(guessed_key.clone()).unwrap();
    println!("The guessed key is: {}", guessed_key_str);

    let decoded = xor_repeating(&data, &guessed_key);
    println!("{}", String::from_utf8_lossy(&decoded));
}
