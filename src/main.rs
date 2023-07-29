use color_eyre::eyre::Result;
use std::io::{self, Read};

mod cli_args;
mod bieye;

use cli_args::CliArgs;
use bieye::Bieye;
use clap::Parser;

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    // Get usert options flag input
    let _args = CliArgs::parse();
    // println!("{:?}", _args);

    let mut input_text = String::new();

    // Load user input text
    if let Some(text) = _args.text {
        // Passed via command line option flag
        input_text = text;
    } else {
        // Passed via stdin pipe
        io::stdin().read_to_string(&mut input_text)?;
        if input_text.is_empty() {
            println!("No input from stdin");
            return Ok(());
        }
    }

    let mut br = Bieye::default();
    br.text_input = input_text;
    br.is_colored = _args.color;
    br.is_dimmed = _args.dim;
    br.process_text().print_processed();

    Ok(())
}
