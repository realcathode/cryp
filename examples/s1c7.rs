use aes::Aes128;
use block_modes::{BlockMode, Ecb, block_padding::Pkcs7};

use std::str;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Aes128Ecb = Ecb<Aes128, Pkcs7>;

use cryp::base64_decode;
fn main() -> io::Result<()> {
    let mykey =String::from("YELLOW SUBMARINE");

    let filename = "./7.txt";
    let file = File::open(filename)?;
    
    let reader = BufReader::new(file);

    let mut buf = Vec::new();

    //trim new lines from the file
    for line in reader.lines() {
        let line = line?;
        buf.extend_from_slice(line.as_bytes());
    }

    // println!("b64 bytes: {:?}\n", buf);
    let string_slice = std::str::from_utf8(&buf).unwrap();

    let decoded_plaintext = base64_decode(string_slice);
    // println!("decoded bytes{:?}\n", decoded_plaintext);

    let cipher = Aes128Ecb::new_from_slices(mykey.as_bytes(), &[]).unwrap();

    let decrypted_ciphertext = cipher.decrypt_vec(&decoded_plaintext.unwrap()).unwrap();
    println!("{}", str::from_utf8(&decrypted_ciphertext).unwrap());
    Ok(())
}