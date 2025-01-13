use std::collections::HashSet;
use std::io::{self, BufReader, BufRead};
use std::fs::File;
use colored::*;

fn main() -> io::Result<()> {
    let filename = "../../input/8.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut count = 0;

    for line in reader.lines() {
        count = count + 1;
        let line = line?;

        let mut seen = HashSet::new();
        let mut duplicates: Vec<&str> = vec![];

        let chunks: Vec<&str> = line.as_bytes()
            .chunks(32)
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect();

        for chunk in &chunks {
            if !seen.insert(*chunk) {
                duplicates.push(*chunk);
            }
        }

        let unique_duplicates: HashSet<_> = duplicates.into_iter().collect();

        if !unique_duplicates.is_empty() {
            print!("Line: {}\n", count);
            
            for chunk in chunks {
                for duplicate in &unique_duplicates {
                    if chunk.contains(*duplicate) {
                        print!("{}", chunk.red());
                    }else {
                        print!("{}", chunk);
                    }
                }
            }
            println!();
        }
    }
    Ok(())
}
