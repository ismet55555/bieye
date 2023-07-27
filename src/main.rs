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
    // println!("{:?}", _args);

    let mut input_text = String::new();

    // Load user input text
    if let Some(text) = _args.text {
        // Passed via command line option flag
        println!("Passed STDIN: {}", text);
        input_text = text;
        // return Ok(());
    } else {
        // Passed via stdin pipe
        io::stdin().read_to_string(&mut input_text)?;
        if input_text.is_empty() {
            println!("No input from stdin");
            return Ok(());
        }
    }

    let mut br = BioReader::default();
    br.text_input = input_text;
    br.process_text().print_processed();

    Ok(())
}
