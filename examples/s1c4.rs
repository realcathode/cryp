use cryp::{
    hexstr_to_bytes,
    one_byte_xor,
    is_valid_text
};
use std::fs::File;
use std::io::{self, BufReader, Read, BufRead};

fn main() -> io::Result<()> {
    let filename = "/path/to/4.txt";
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?; 
        let mut score = 0.0;
        for i in 0..255 {
            let bytes_line = hexstr_to_bytes(&line).unwrap();
            let xored = one_byte_xor(&bytes_line, i);
            if is_valid_text(&xored) {
                println!("Found string: {}\nKey: {} = {}\nOriginal hex: {}", 
                String::from_utf8_lossy(&xored), i, i as char, line);
            }
        }
    }

    Ok(())
}