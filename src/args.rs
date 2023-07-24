use clap::Parser;

const LONG_ABOUT: &str = "A CLI tool that reads text and \
    return it back in BioReader format \
    for faster reading.";

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "BioReader CLI Tool",  // Shows at --h
    long_about = LONG_ABOUT,  // Shows at --help 
)]
pub struct BioReadArgs {
    /// The first arg
    #[clap(short = 'f', long, value_name = "INT", required = false)]
    pub first_arg: Option<String>,

    /// The second arg
    #[clap(short = 's', long, value_name = "STRING", required = false)]
    pub second_arg: Option<String>,

    /// Capture text from stdin
    #[clap(value_name = "TEXT")]
    pub text: Option<String>,
}
