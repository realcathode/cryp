use cryp::{character_frequency};

fn main() {
    let data = "Hello, world!".as_bytes();
    let frequencies = character_frequency(data);

    for (ch, count) in &frequencies {
        println!("'{}': {}", ch, count);
    }
}