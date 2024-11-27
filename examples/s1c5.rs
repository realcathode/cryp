use cryp::{bytes_to_hexstr, repeating_xor};

fn main() {
    let s = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let result = repeating_xor(s.as_bytes(), "ICE".as_bytes());
    println!("{}", bytes_to_hexstr(&result));
}