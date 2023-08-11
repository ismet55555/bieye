use color_eyre::eyre::Result;
use log::debug;
use std::io::{self, Read};

pub mod bieye;
mod cli_args;

use bieye::Bieye;
use clap::Parser;
use cli_args::CliArgs;

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    // Get usert options flag input
    let _args = CliArgs::parse();
    debug!("[CLI ARGS] {:?}", _args);

    let mut input_text = String::new();

    // Load user input text
    if let Some(text) = _args.text {
        // Passed via command line option flag
        input_text = text;
    } else {
        // Passed via stdin pipe
        io::stdin().read_to_string(&mut input_text)?;
        if input_text.is_empty() {
            println!("ERROR: No input received via stdin.");
            return Ok(());
        }
    }

    let mut be = Bieye {
        text_input: input_text,
        is_colored: _args.color,
        is_dimmed: _args.dim,
        ..Default::default()
    };

    // Process text
    be.process_text().print_processed();

    Ok(())
}
