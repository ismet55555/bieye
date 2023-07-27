use colored::Colorize;
use log::debug;
use regex::Regex;

// BioReader regex patterns
const REGEX_PUNCTUATION: &str = r"^[\W_]+$";
const REGEX_NUMBER: &str = r"^\d+$";
const REGEX_CONTRACTION: &str = r"^\w+'\w+$";
const REGEX_LESS_THAN_THREE: &str = r"^\w{1,2}$";
const REGEX_THREE_LETTER_WORD: &str = r"^\w{3}$";

/// This struct handles all BioReader related functions
#[derive(Debug)]
pub struct BioReader {
    pub text_input: String,
    pub text_output: String,
    pub bold: bool,
    pub is_colored: bool,
    pub color: String,
    pub is_dimmed: bool,
}

impl Default for BioReader {
    fn default() -> Self {
        Self {
            text_input: String::new(),
            text_output: String::new(),
            bold: true,
            is_colored: false,
            color: String::from(""),
            is_dimmed: false,
        }
    }
}

impl BioReader {
    /// Print the original text
    // pub fn print_original(&self) {
    //     println!("{}", self.text_input);
    // }

    /// Print the processed text
    pub fn print_processed(&self) {
        println!("{}", self.text_output);
    }

    /// Apply bold text to the text
    /// TODO: Optomize this function! -
    pub fn process_text(&mut self) -> &Self {
        // Compile regex patterns
        let regex_whitespace: Regex = Regex::new(r"^[\s]+$").unwrap();
        let regex_punctuation: Regex = Regex::new(REGEX_PUNCTUATION).unwrap();
        let regex_number: Regex = Regex::new(REGEX_NUMBER).unwrap();
        let regex_contraction: Regex = Regex::new(REGEX_CONTRACTION).unwrap();
        let regex_less_than_three: Regex = Regex::new(REGEX_LESS_THAN_THREE).unwrap();
        let regex_three_letter_word: Regex = Regex::new(REGEX_THREE_LETTER_WORD).unwrap();

        // Regex pattern to divide text into words
        let re_words = Regex::new(r"\w+|[^\w]|[\s]").unwrap();

        // Divided input text into words
        let words: Vec<&str> = re_words
            .find_iter(self.text_input.as_str())
            .map(|mat| mat.as_str())
            .collect();

        let mut words_processed: Vec<String> = Vec::new();

        debug!("Processing {} words ...", words.len());
        for word in words {
            let word_processed = match word {

                word if regex_whitespace.is_match(word) => {
                    debug!("[Whitespace]");
                    word.normal()
                }

                word if regex_punctuation.is_match(word) => {
                    debug!("{} -> {} [Punctuation]", word, word);
                    word.normal()
                }

                word if regex_number.is_match(word) => {
                    debug!("{} -> {} [Number]", word, word.bold());
                    word.bold()
                }

                word if regex_contraction.is_match(word) => {
                    debug!("{} -> {} [Contraction]", word, word.bold());
                    word.bold()
                }

                word if regex_less_than_three.is_match(word) => {
                    debug!("{} -> {} [Less than 3 letters]", word, word.bold());
                    word.bold()
                }

                word if regex_three_letter_word.is_match(word) => {
                    let first_half = &word[..2];
                    let second_half = &word[2..];
                    debug!(
                        "{} -> {} [3-Letter word]",
                        word,
                        format!("{}{}", first_half.bold(), second_half)
                    );
                    format!("{}{}", first_half.bold(), second_half).normal() // TODO: Better way to
                                                                             // concatenate into a ColoredString?
                }

                _ => {
                    let split_point = word.len() / 2;
                    let first_half = &word[..split_point];
                    let second_half = &word[split_point..];
                    debug!(
                        "{} -> {} [3-Letter word]",
                        word,
                        format!("{}{}", first_half.bold(), second_half)
                    );
                    format!("{}{}", first_half.bold(), second_half).normal()
                }
            };

            words_processed.push(word_processed.to_string());
        }
        debug!("Successfully processed all words!");
        self.text_output = words_processed.join("");
        self
    }
}
