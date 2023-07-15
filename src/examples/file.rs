extern crate txt_processor;

use txt_processor::*;

fn main() {
    let t = TxtProcessor::from_file("./src/examples/file.txt").unwrap();
    let q = TxtProcessor::from_str(String::from("В этот лес завороженный,\r
По пушинкам серебра,\r
Я с винтовкой заряженной\r
На охоту шел вчера. "));
    assert_eq!(q.get_content(), t.get_content());
}