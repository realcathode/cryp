use std::fs::File;
use std::io::{self, BufRead, BufReader};
use cryp::{base64_decode, base64_encode, encrypt_cbc};

fn main() -> io::Result<()> {
    let filename = "../../input/10.txt";
    let file = File::open(filename)?;
    
    let reader = BufReader::new(file);

    let mut buf = Vec::new();

    for line in reader.lines().flatten() {
        buf.extend_from_slice(line.as_bytes());
    }
    
    let plaintext = base64_decode(std::str::from_utf8(&buf).unwrap());
    
    let key = b"YELLOW SUBMARINE";
    let iv = [0x00;16];
    
    let encrypted = encrypt_cbc(key, &iv, &plaintext.unwrap());
    println!("Encrypted: {:?}", encrypted);
    
    let encrypted_base64 = base64_encode(&encrypted.unwrap());
    println!("Base64: {}", encrypted_base64);

    Ok(())
}
