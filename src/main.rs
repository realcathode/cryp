use cryp;

fn main() {
    let s = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

    if let Ok(a) = cryp::hex_to_base64(&s) {
        println!("HEX: {}\nBASE64: {}", s, a);
    }
}