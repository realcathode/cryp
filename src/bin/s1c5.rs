use cryp::{bytes_to_hexstr, xor_repeating};

fn main() {
    let s = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let result = xor_repeating(s.as_bytes(), "ICE".as_bytes());
    println!("{}", bytes_to_hexstr(&result));
}