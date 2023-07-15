extern crate txt_processor;

use txt_processor::*;

fn main() {
    let mut t = TxtProcessor::from_str(String::from("В этот лес завороженный,По пушинкам серебра.Я с винтовкой заряженной На охоту шел вчера."));
    t.fix();
    assert_eq!("В этот лес завороженный, По пушинкам серебра, Я с винтовкой заряженной На охоту шел вчера.", t.get_content());
}