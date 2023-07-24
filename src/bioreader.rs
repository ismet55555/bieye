use crossterm::style::{Attribute, SetAttribute, ResetColor};
use crossterm::terminal;


/// This struct handles all BioReader related functions
#[derive(Debug)]
pub struct BioReader {
    pub text: String,
    pub processed_text: String,
}

impl Default for BioReader {
    fn default() -> Self {
        Self {
            text: String::new(),
            processed_text: String::new(),
        }
    }
}

impl BioReader {
    /// Print the original text
    pub fn print_original(&self) {
        println!("{}", self.text);
    }

    /// Print the processed text
    pub fn print_processed(&self) {
        terminal::Clear(terminal::ClearType::All);
        println!("{}", self.processed_text);
    }

    /// Apply bold text to the text
    pub fn apply_bold_text(&mut self) -> &Self {
        self.processed_text = format!(
            "{}{}{}",
            SetAttribute(Attribute::Bold),
            self.text,
            ResetColor
        );
        self
    }
}
