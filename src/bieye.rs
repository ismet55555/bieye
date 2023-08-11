use colored::{ColoredString, Colorize};
use log::debug;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use regex::Regex;
use std::collections::HashMap;

const REGEX_DIVIDE_WORDS: &str = r"\w+|[^\w]|[\s]";
const REGEX_WHITESPACE: &str = r"^[\s]+$";
const REGEX_PUNCTUATION: &str = r"^[\W_]+$";
const REGEX_NUMBER: &str = r"^\d+$";
const REGEX_CONTRACTION: &str = r"^\w+'\w+$";
const REGEX_LESS_THAN_THREE: &str = r"^\w{1,2}$";
const REGEX_THREE_LETTER_WORD: &str = r"^\w{3}$";

const THREAD_COUNT_LIMIT: usize = 100;
const WORD_COUNT_LIMIT_THREADED: usize = 500;

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
    fn color_text(&self, text: &str) -> ColoredString {
        if self.is_colored {
            text.yellow()
        } else {
            text.normal()
        }
    }

    // Dim text if self.is_dimmed is true
    fn dim_text(&self, text: &str) -> ColoredString {
        if self.is_dimmed {
            text.dimmed()
        } else {
            text.normal()
        }
    }

    // Process a single word
    fn process_word(&self, word: &str, regex_patterns: &HashMap<&str, Regex>) -> String {
        let word_processed = match word {
            word if regex_patterns.get("whitespace").unwrap().is_match(word) => {
                debug!("[Whitespace]");
                word.normal()
            }

            word if regex_patterns.get("punctuation").unwrap().is_match(word) => {
                let word_style = self.dim_text(word);
                debug!("{} -> {} [Punctuation]", &word_style, &word_style);
                word_style
            }

            word if regex_patterns.get("number").unwrap().is_match(word) => {
                let word_style = self.color_text(word).bold();
                debug!("{} -> {} [Number]", &word_style, &word_style);
                word_style
            }

            word if regex_patterns.get("contraction").unwrap().is_match(word) => {
                let word_style = self.color_text(word).bold();
                debug!("{} -> {} [Contraction]", &word, &word.bold());
                word_style
            }

            word if regex_patterns
                .get("less_than_three")
                .unwrap()
                .is_match(word) =>
            {
                let word_style = self.color_text(word).bold();
                debug!("{} -> {} [Less than 3 letters]", &word, &word.bold());
                word_style
            }

            word if regex_patterns
                .get("three_letter_word")
                .unwrap()
                .is_match(word) =>
            {
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
        word_processed.to_string()
    }

    /// Process loaded text
    pub fn process_text(&mut self) -> &Self {
        // Compile regex patterns
        // TODO: Handle these .unwrap() calls correctly
        let regex_whitespace: Regex = Regex::new(REGEX_WHITESPACE).unwrap();
        let regex_punctuation: Regex = Regex::new(REGEX_PUNCTUATION).unwrap();
        let regex_number: Regex = Regex::new(REGEX_NUMBER).unwrap();
        let regex_contraction: Regex = Regex::new(REGEX_CONTRACTION).unwrap();
        let regex_less_than_three: Regex = Regex::new(REGEX_LESS_THAN_THREE).unwrap();
        let regex_three_letter_word: Regex = Regex::new(REGEX_THREE_LETTER_WORD).unwrap();

        let mut regex_patterns = HashMap::new();
        regex_patterns.insert("whitespace", regex_whitespace);
        regex_patterns.insert("punctuation", regex_punctuation);
        regex_patterns.insert("number", regex_number);
        regex_patterns.insert("contraction", regex_contraction);
        regex_patterns.insert("less_than_three", regex_less_than_three);
        regex_patterns.insert("three_letter_word", regex_three_letter_word);

        // Regex pattern to divide text into words
        let re_words = Regex::new(REGEX_DIVIDE_WORDS).unwrap();

        // Divided input text into words
        let words: Vec<&str> = re_words
            .find_iter(self.text_input.as_str())
            .map(|mat| mat.as_str())
            .collect();
        debug!("Number of words loaded: {}", words.len());

        let words_processed: Vec<String> = if words.len() > WORD_COUNT_LIMIT_THREADED {
            // Concurrent/Threaded processing
            debug!("Processing words concurrently with thread pool ...");
            let pool = ThreadPoolBuilder::new()
                .num_threads(THREAD_COUNT_LIMIT)
                .build()
                .unwrap();

            let words_thread_out = pool.install(|| {
                words
                    .par_iter()
                    .enumerate()
                    .map(|(index, word)| (index, self.process_word(word, &regex_patterns)))
                    .collect()
            });

            // Sort the processed words based on the original word index to maintain order
            let mut sorted_words: Vec<(usize, String)> = words_thread_out;
            sorted_words.sort_by_key(|(index, _)| *index);

            // Extract the processed words in order and ignore the indices
            sorted_words.into_iter().map(|(_, word)| word).collect()
        } else {
            // Sequential processing
            debug!("Processing words sequentially ...");
            words
                .iter()
                .map(|word| self.process_word(word, &regex_patterns))
                .collect()
        };

        debug!("Successfully processed all words!");
        self.text_output = words_processed.join("");
        self
    }
}
