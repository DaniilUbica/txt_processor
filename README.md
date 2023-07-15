# txt_processor [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url]

## A little library for text processing.

## Features
* Read text from file
* Append/Write text to file
* Fix spaces in text
* Count words in text
* Count lines in text
* Search words with index
* Search words with filter
* Search words with regex
* Count word's occurences

## Usage
```toml
[dependencies]
txt_processor = "0.1.2"
```
## Examples
```rust
extern crate txt_processor;

use txt_processor::*;

fn main() {
    let mut t = TxtProcessor::from_str(String::from("This is text for example!"));
    t.replace_word("is", "replaced is");
    assert_eq!(1, t.get_lines());
    assert_eq!(5, t.get_words());
    assert_eq!("This replaced is text for example!", t.get_content());
}
```

# You can find more examples in [examples](https://github.com/DaniilUbica/txt_processor/tree/master/src/examples) folder

[documentation-img]: https://docs.rs/txt_processor/badge.svg
[documentation-url]: https://docs.rs/txt_processor
[package-img]: https://img.shields.io/crates/v/txt_processor.svg
[package-url]: https://crates.io/crates/txt_processor
