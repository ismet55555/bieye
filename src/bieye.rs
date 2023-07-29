use colored::{ColoredString, Colorize};
use log::debug;
use regex::Regex;

// Reader regex patterns
const REGEX_DIVIDE_WORDS: &str = r"\w+|[^\w]|[\s]";
const REGEX_WHITESPACE: &str = r"^[\s]+$";
const REGEX_PUNCTUATION: &str = r"^[\W_]+$";
const REGEX_NUMBER: &str = r"^\d+$";
const REGEX_CONTRACTION: &str = r"^\w+'\w+$";
const REGEX_LESS_THAN_THREE: &str = r"^\w{1,2}$";
const REGEX_THREE_LETTER_WORD: &str = r"^\w{3}$";

/// This struct handles all Bieye related functions
#[derive(Debug)]
pub struct Bieye {
    pub text_input: String,
    pub text_output: String,
    pub bold: bool,
    pub is_colored: bool,
    pub color: String,
    pub is_dimmed: bool,
}

impl Default for Bieye {
    fn default() -> Self {
        Self {
            text_input: String::new(),
            text_output: String::new(),
            bold: true,
            is_colored: true,
            color: String::from(""),
            is_dimmed: true,
        }
    }
}

impl Bieye {
    /// Print the processed text
    pub fn print_processed(&self) {
        println!("{}", self.text_output);
    }

    // Color text if self.is_colored is true
    fn color_text(&self, word: &str) -> ColoredString {
        let word_style = if self.is_colored {
            word.yellow()
        } else {
            word.normal()
        };
        word_style
    }

    // Dim text if self.is_dimmed is true
    fn dim_text(&self, word: &str) -> ColoredString {
        let word_style = if self.is_dimmed {
            word.dimmed()
        } else {
            word.normal()
        };
        word_style
    }

    /// Apply bold text to the text
    pub fn process_text(&mut self) -> &Self {
        // Compile regex patterns
        let regex_whitespace: Regex = Regex::new(REGEX_WHITESPACE).unwrap();
        let regex_punctuation: Regex = Regex::new(REGEX_PUNCTUATION).unwrap();
        let regex_number: Regex = Regex::new(REGEX_NUMBER).unwrap();
        let regex_contraction: Regex = Regex::new(REGEX_CONTRACTION).unwrap();
        let regex_less_than_three: Regex = Regex::new(REGEX_LESS_THAN_THREE).unwrap();
        let regex_three_letter_word: Regex = Regex::new(REGEX_THREE_LETTER_WORD).unwrap();

        // Regex pattern to divide text into words
        let re_words = Regex::new(REGEX_DIVIDE_WORDS).unwrap();

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
                    let word_style = self.dim_text(word);
                    debug!("{} -> {} [Punctuation]", &word_style, &word_style);
                    word_style
                }

                word if regex_number.is_match(word) => {
                    let word_style = self.color_text(word).bold();
                    debug!("{} -> {} [Number]", &word_style, &word_style);
                    word_style
                }

                word if regex_contraction.is_match(word) => {
                    let word_style = self.color_text(word).bold();
                    debug!("{} -> {} [Contraction]", &word, &word.bold());
                    word_style
                }

                word if regex_less_than_three.is_match(word) => {
                    let word_style = self.color_text(word).bold();
                    debug!("{} -> {} [Less than 3 letters]", &word, &word.bold());
                    word_style
                }

                word if regex_three_letter_word.is_match(word) => {
                    let half_1 = &word[..2];
                    let half_2 = &word[2..];
                    let half_1_style = self.color_text(half_1).bold();
                    let half_2_style = self.dim_text(half_2);
                    debug!(
                        "{} -> {} [3-Letter word]",
                        &word,
                        format!("{}{}", &half_1_style, &half_2_style)
                    );
                    format!("{}{}", half_1_style, half_2_style).normal()
                }

                _ => {
                    let split_point = word.len() / 2;
                    let half_1 = &word[..split_point];
                    let half_2 = &word[split_point..];
                    let half_1_style = self.color_text(half_1).bold();
                    let half_2_style = self.dim_text(half_2);
                    debug!(
                        "{} -> {} [3-Letter word]",
                        word,
                        format!("{}{}", &half_1_style, &half_2_style)
                    );
                    format!("{}{}", half_1_style, half_2_style).normal()
                }
            };

            words_processed.push(word_processed.to_string());
        }
        debug!("Successfully processed all words!");
        self.text_output = words_processed.join("");
        self
    }
}
