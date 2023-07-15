extern crate txt_processor;

use txt_processor::*;

fn main() {
    let mut t = TxtProcessor::from_str(String::from("В этот лес завороженный,
    По пушинкам серебра,"));
    let q = TxtProcessor::from_str(String::from("Я с винтовкой заряженной
    На охоту шел вчера."));

    t.add_to_content(q.get_content());
    assert_eq!("В этот лес завороженный,
    По пушинкам серебра,Я с винтовкой заряженной
    На охоту шел вчера.", t.get_content());
    assert_eq!(15, t.get_words());
    assert_eq!(4, t.get_lines());

    t.rewrite_file("./src/examples/file.txt").unwrap();
}