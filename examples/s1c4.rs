use cryp::{
    hexstr_to_bytes,
    xor_one_byte,
    is_valid_text
};
use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let filename = "/path/to/4.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?; 
        for i in 0..255 {
            let bytes_line = hexstr_to_bytes(&line).unwrap();
            let xored = xor_one_byte(&bytes_line, i);
            if is_valid_text(&xored) {
                println!("Found string: {}\nKey: {} = {}\nOriginal hex: {}", 
                String::from_utf8_lossy(&xored), i, i as char, line);
            }
        }
    }

    Ok(())
}