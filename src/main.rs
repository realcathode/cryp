use cryp::{hexstr_to_bytes, bytes_to_hexstr};

fn main() {
    let s = "We started dancing and love put us into a groove";
    println!("String literal: \"{s}\"\n");

    let hex = bytes_to_hexstr(s.as_bytes());
    println!("bytes to hex encoded str: {}", hex);

    let bytes = hexstr_to_bytes(&hex).unwrap();
    println!("vector of bytes: {:?}", bytes);
}