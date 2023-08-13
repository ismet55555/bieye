use color_eyre::eyre::Result;
use log::debug;
use std::io::{self, stdin, IsTerminal, Read};

pub mod bieye;
mod cli_args;

use bieye::Bieye;
use clap::CommandFactory;
use clap::Parser;
use cli_args::CliArgs;

#[cfg(not(target_family = "wasm"))]
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
        debug!("Reading input from command line argument ...");
        input_text = text;
    } else {
        // Passed via stdin pipe
        debug!("Reading input from stdin ...");
        let is_stdin_available = !stdin().is_terminal();
        if is_stdin_available {
            io::stdin().read_to_string(&mut input_text)?;
        } else {
            debug!("No input received via stdin.");
            let mut cmd = CliArgs::command();
            cmd.print_help()?;
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
