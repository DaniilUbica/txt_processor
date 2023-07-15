extern crate txt_processor;

use txt_processor::*;

fn main() {
    let t = TxtProcessor::from_str(String::from("Crazy Fredrick bought many very exquisite opal jewels."));
    let words_with_len_4 = t.filter(&|w| w.len() == 4); // collect all words with length = 4
    assert_eq!(vec!["many", "very", "opal"], words_with_len_4);
}