extern crate itertools;
extern crate regex;

use self::itertools::Itertools;
use self::regex::Regex;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

#[inline]
fn count_words(text: &String) -> usize {
    let word_delimiters = [' ', '\n', '\t', ',', '.', '!', '?', ';', ':', '"', '(', ')'];

    let mut word_count = 0;
    let mut is_word = false;

    for c in text.chars() {
        if word_delimiters.contains(&c) {
            is_word = false;
        } else if !is_word {
            is_word = true;
            word_count += 1;
        }
    }

    word_count
}

#[inline]
fn collect_words(text: &String) -> Vec<String> {
    let word_delimiters = [
        ' ', '\n', '\t', ',', '.', '!', '?', ';', ':', '"', '(', ')', '\0',
    ];
    let mut words = vec![];
    let mut curr_word = String::new();

    for c in text.chars() {
        if !word_delimiters.contains(&c) {
            curr_word.push(c);
        } else {
            if !curr_word.is_empty() {
                words.push(curr_word.clone());
                curr_word.clear();
            }
        }
    }

    words
}

/// TxtProcessor struct
#[derive(Debug)]
pub struct TxtProcessor {
    pub(crate) content: String,
    pub(crate) file_path: Option<String>,
    pub(crate) lines: usize,
    pub(crate) words: usize,
}

/// Struct for error
#[derive(Clone, Debug)]
pub struct Error(pub String);

impl std::fmt::Display for Error {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for Error {
    #[inline]
    fn description(&self) -> &str {
        &self.0
    }
}

impl TxtProcessor { 
    /// Reads text from file
    pub fn from_file(file_path: &str) -> Result<Self, Error> {
        let mut str = String::new();
        let file = OpenOptions::new().read(true).open(file_path);
        let mut file = match file {
            Ok(file) => file,
            Err(_) => {
                return Err(Error(format!(
                    "Can't open file with filename '{file_path}'"
                )))
            }
        };

        let res = file.read_to_string(&mut str);
        match res {
            Ok(_) => (),
            Err(_) => {
                return Err(Error(format!(
                    "Can't read file with filename '{file_path}'"
                )))
            }
        };

        let lines = str.lines().count();
        let words = count_words(&str);
        str.push(' ');
        Ok(TxtProcessor {
            content: str,
            file_path: Some(file_path.to_string()),
            lines,
            words,
        })
    }
    /// Reads text from string
    pub fn from_str(str: String) -> Self {
        let lines = str.lines().count();
        let words = count_words(&str);
        TxtProcessor {
            content: str,
            file_path: None,
            lines,
            words,
        }
    }
    /// Reads text from file
    pub fn count_word_occurences(&self, word: &str) -> usize {
        let words = collect_words(&self.content);
        let mut c = 0;
        for w in words {
            if w == word {
                c += 1;
            }
        }
        c
    }
    /// Counts how many times a word occurs in a text
    pub fn contains(&self, word: &str) -> bool {
        collect_words(&self.content).contains(&word.to_string())
    }
    /// Finds unique words in a text
    pub fn find_unique(&self) -> Vec<String> {
        let words = collect_words(&self.content);
        words.into_iter().unique().collect::<Vec<String>>()
    }
    /// Finds all words matching the condition
    pub fn filter(&self, condition: &dyn Fn(&str) -> bool) -> Vec<String> {
        let words = collect_words(&self.content);
        let mut ret = vec![];
        for w in words {
            if condition(&w) {
                ret.push(w);
            }
        }
        ret
    }
    /// Finds the first ocuurence of a word in a text
    pub fn find_first_occurence(&self, word: &str) -> Result<usize, Error> {
        let words = collect_words(&self.content);
        let res = words.iter().position(|w| w == word);
        match res {
            Some(index) => Ok(index),
            None => return Err(Error(format!("No word '{word}' in content"))),
        }
    }
    /// Finds all occurrences of a word in a text
    pub fn search_occurences(&self, word: &str) -> Vec<usize> {
        let words = collect_words(&self.content);
        let mut res = vec![];
        let mut c = 0;
        for w in words {
            if w == word {
                res.push(c);
            }
            c += 1;
        }
        res
    }
    /// Finds word on that position in a text
    pub fn search_with_index(&self, index: usize) -> Result<String, Error> {
        let words = collect_words(&self.content);
        if index < words.len() {
            Ok(words[index].clone())
        } else {
            Err(Error(String::from("Wrong index value")))
        }
    }
    /// Replaces all words 'from' on word 'to'
    pub fn replace_word(&mut self, from: &str, to: &str) {
        let regex = Regex::new(&(r"\b".to_string() + from + r"\b")).unwrap();
        self.content = regex.replace_all(&self.content, to).to_string();
        self.words = count_words(&self.content);
    }
    /// Replaces all characters 'from' on character 'to'
    pub fn replace_char(&mut self, from: char, to: char) {
        self.content = self.content.replace(from, &to.to_string());
        self.words = count_words(&self.content);
    }
    /// Rewrites file with self.content
    pub fn rewrite_file(&self, file_path: &str) -> Result<(), Error> {
        let file = OpenOptions::new().write(true).open(file_path);
        let mut file = match file {
            Ok(file) => file,
            Err(_) => {
                return Err(Error(format!(
                    "Can't open file with filename '{file_path}'"
                )))
            }
        };
        let res = write!(file, "{}", self.content);
        match res {
            Ok(_) => Ok(()),
            Err(_) => {
                return Err(Error(format!(
                    "Can't write to file with filename '{file_path}'"
                )))
            }
        }
    }
    /// Appends self.content to file
    pub fn append_file(&self, file_path: &str) -> Result<(), Error> {
        let file = OpenOptions::new().append(true).open(file_path);
        let mut file = match file {
            Ok(file) => file,
            Err(_) => {
                return Err(Error(format!(
                    "Can't open file with filename '{file_path}'"
                )))
            }
        };
        let res = write!(file, "{}", self.content);
        match res {
            Ok(_) => Ok(()),
            Err(_) => {
                return Err(Error(format!(
                    "Can't write to file with filename '{file_path}'"
                )))
            }
        }
    }
    /// Fix spaces in text
    pub fn fix(&mut self) {
        let mut result = String::new();
        let word_delimiters = [',', '.', '!', '?', ';', ':', '"', '(', ')'];
        let mut i = 0;

        for c in self.content.chars() {
            if i < self.content.chars().count()-1 {
                if !word_delimiters.contains(&c) {
                    result.push(c);
                }
                else if self.content.chars().nth(i + 1).unwrap() != ' ' && self.content.chars().nth(i + 1).unwrap() != '\n' {
                    result.push(c);
                    result.push(' ');
                } 
                else {
                    result.push(c);
                }
            }
            i += 1;
        }
        result.push(self.content.chars().last().unwrap());
        self.content = result;
        self.words = count_words(&self.content);
    }
    /// Search for all words matching this regular expression
    pub fn search_with_regex(&self, regex: Regex) -> Vec<String> {
        let mut w = vec![];
        for cap in regex.find_iter(&self.content) {
            w.push(String::from(cap.as_str()));
        }
        w
    }
    /// Adds text to this TxtProcessor's content
    pub fn add_to_content(&mut self, text: String) {
        let w = count_words(&text);
        self.content = self.content.clone() + &text;
        self.words = self.words + w;
        self.lines = self.lines + text.lines().count();
    }

    /// Returns lines amount
    pub fn get_lines(&self) -> usize {
        self.lines
    }
    /// Returns words amount
    pub fn get_words(&self) -> usize {
        self.words
    }
    /// Returns content
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
    /// Returns file path
    pub fn get_file_path(&self) -> Option<String> {
        self.file_path.clone()
    }
}
