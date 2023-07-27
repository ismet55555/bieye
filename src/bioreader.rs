use colored::Colorize;
use crossterm::terminal;
use log::debug;
use regex::Regex;

// FIXME: Should I use HashMap for bioreader options?

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
    pub fn print_original(&self) {
        println!("{}", self.text_input);
    }

    /// Print the processed text
    pub fn print_processed(&self) {
        terminal::Clear(terminal::ClearType::All);
        println!("{}", self.text_output);
    }

    // /// Style text depending on options passed
    // /// **TODO**: Add color, dimmed, underline, etc
    // fn color_word(&self, word: String) -> String {
    //     let word_processed = word.bold();
    //     word_processed.to_string()
    // }

    /// Check if word matches a regex pattern
    fn check_word_match(&self, regex_pattern: &str, word: &str) -> bool {
        let re_word = Regex::new(regex_pattern).unwrap();
        re_word.is_match(word)
    }

    /// Apply bold text to the text
    pub fn process_text(&mut self) -> &Self {
        let re_words = Regex::new(r"\w+|[^\w]|[\s]").unwrap();

        // Divided input text into words
        let words: Vec<&str> = re_words
            .find_iter(self.text_input.as_str())
            .map(|mat| mat.as_str())
            .collect();

        let mut words_processed: Vec<String> = Vec::new();

        debug!("Processing {} words ...", words.len());
        for word in words {

            // Whitespace
            if self.check_word_match(r"^[\s]+$", word) {
                let word_processed = word.to_string();
                debug!("[Whitespace]");
                words_processed.push(word_processed);
                continue;
            }

            // Word is punctuation
            if self.check_word_match(r"^[\W_]+$", word) {
                let word_processed = word.to_string();
                debug!("{} -> {} [Punctuation]", word, word_processed);
                words_processed.push(word_processed);
                continue;
            }

            // Word is a number
            if self.check_word_match(r"^\d+$", word) {
                let word_processed = word.bold().to_string();
                debug!("{} -> {} [Number]", word, word_processed);
                words_processed.push(word_processed);
                continue;
            }

            // Word is a contraction
            // NOTE: Think about splitting this in first and second part
            if self.check_word_match(r"^\w+'\w+$", word) {
                let word_processed = word.bold().to_string();
                debug!("{} -> {} [Contraction]", word, word_processed);
                words_processed.push(word_processed);
                continue;
            }

            // Word with less than three letters
            if self.check_word_match(r"^\w{1,2}$", word) {
                let word_processed = word.bold().to_string();
                debug!("{} -> {} [Less than 3 letters]", word, word_processed);
                words_processed.push(word_processed);
                continue;
            }

            // Word is 3 letter word
            if self.check_word_match(r"^\w{3}$", word) {
                let first_half = &word[..2].to_string();
                let second_half = &word[2..].to_string();
                let word_processed = format!("{}{}", first_half.bold(), second_half);
                debug!("{} -> {} [3-Letter word]", word, word_processed);
                words_processed.push(word_processed);
                continue;
            }

            // Word is anything else
            let split_point = word.len() / 2;
            let first_half = &word[..split_point].to_string();
            let second_half = &word[split_point..].to_string();
            let word_processed = format!("{}{}", first_half.bold(), second_half);
            debug!("{} -> {} [3-Letter word]", word, word_processed);
            words_processed.push(word_processed);

            // let second_half = if self.is_dimmed {
            //     second_half.dimmed().to_string()
            // } else {
            //     second_half.to_string()
            // };
        }
        debug!("Successfully processed all words!");

        println!("----------------------------------");
        let words_joined = words_processed.join("");
        println!("{}", words_joined);

        self
    }
}
