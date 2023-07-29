use clap::Parser;

const DESCRIPTION: &str = concat!(
    env!("CARGO_PKG_NAME"),
    " v",
    env!("CARGO_PKG_VERSION"),
    "\n\n",
    "This CLI tool reads text and returns it back in bionic reading format\n",
    "for blazingly fast loading and even faster reading!",
);

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = DESCRIPTION,       // Shows at -h
    long_about = DESCRIPTION,  // Shows at --help 
)]
pub struct CliArgs {
    /// Capture text from stdin
    #[clap(value_name = "TEXT")]
    pub text: Option<String>,

    /// Color highlighted text
    #[clap(short = 'c', long, required = false)]
    pub color: bool,

    /// Dim text not highlighted
    #[clap(short = 'd', long, required = false)]
    pub dim: bool,

    // ///Color of highlighted text (default: yellow)
    // FIXME: This is currently not working, parse in Bieye struct
    // #[clap(
    //     short = 'y',
    //     long,
    //     value_name = "STRING",
    //     required = false,
    // )]
    // pub color_name: Option<String>,
}
