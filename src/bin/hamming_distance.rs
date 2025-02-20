use cryp::{hamming_distance_char, hamming_distance_bit};

fn main() {
    let s = hamming_distance_bit(b"We started dancing", b"We stopped dancing");
    println!("{}", s);
    let s = hamming_distance_char("We started dancing", "We stopped dancing");
    println!("{}", s);
}