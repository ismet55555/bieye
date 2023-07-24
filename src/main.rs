use color_eyre::eyre::Result;
use std::io::{self, Read};

mod args;
mod bioreader;

use args::BioReadArgs;
use bioreader::BioReader;
use clap::Parser;

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    // Get usert options flag input
    let _args = BioReadArgs::parse();
    println!("{:?}", _args);

    // Check if stdin_text is Some
    if let Some(text) = _args.text {
        println!("STDIN: {}", text);
        return Ok(());
    }

    ///////////////////////////////////////////////////////

    // Read text from stdin
    let mut input_from_stdin = String::new();
    io::stdin().read_to_string(&mut input_from_stdin)?;
    if input_from_stdin.is_empty() {
        println!("No input from stdin");
        return Ok(());
    }

    ///////////////////////////////////////////////////////

    let mut br = BioReader::default();
    br.text = input_from_stdin;
    br.apply_bold_text();

    br.print_original();
    println!("----------------------------------");
    br.print_processed();

    ///////////////////////////////////////////////////////

    Ok(())
}
