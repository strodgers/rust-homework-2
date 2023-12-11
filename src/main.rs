use std::env::args;
use std::error::Error;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() -> Result<(), Box<dyn Error>> {
    let fname = PathBuf::from(args().nth(1).ok_or("Expected filename")?);
    let buffread = BufReader::new(File::open(&fname)?);
   
    let brainfuck_chars = "<>+-[].,";

    for line_result in buffread.lines() {
        let line = line_result?;

        line.chars()
            .filter(|c| brainfuck_chars.contains(*c))
            .for_each(|c| print!("{}", c));
    }

    Ok(())
}
