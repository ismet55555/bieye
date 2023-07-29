//////////////////////////////////////////////////////////////////////////////////
// This file is used to test code snippets that are not used in the main program
//////////////////////////////////////////////////////////////////////////////////


//////////////////////////////////////////////////////////////////////////////////
// Attempt to solve hanging stdin read
//////////////////////////////////////////////////////////////////////////////////
use os_pipe::pipe;
use std::io::prelude::*;

// Check if there are any command-line arguments
let has_args = std::env::args().count() > 1;
println!("HAS ARGS: {}", has_args);

// Create a non-blocking pipe for reading from stdin
let (reader, _) = pipe()?;

// Read from stdin if available
let mut input_from_stdin = String::new();
let mut stdin = BufReader::new(reader);
stdin.read_to_string(&mut input_from_stdin)?;

println!("PIPED STDIN: {}", input_from_stdin);

// If there's nothing read from stdin, return an error
if input_from_stdin.is_empty() {
    return Err(eyre!("No text provided via command-line argument or stdin"));
}



//////////////////////////////////////////////////////////////////////////////////
/// Remove spaces
//////////////////////////////////////////////////////////////////////////////////
        let words: Vec<&str> = re_words
            .find_iter(self.text_input.as_str())
            .map(|mat| mat.as_str())
            .filter(|word| !word.trim().is_empty())  // REMOVING SPACES
            .collect();



//////////////////////////////////////////////////////////////////////////////////
/// Loading stdin
//////////////////////////////////////////////////////////////////////////////////
use std::{io, io::prelude::*};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn main() -> Result<()> {
    for line in io::stdin().lock().lines() {
        println!("length = {}", line?.len());
    }
    Ok(())
}
