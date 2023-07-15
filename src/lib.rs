//! # txt_processor
//!
//! A little library for text processing
//! ## Example
//! ```rust
//! extern crate txt_processor;
//!
//! use txt_processor::*;
//! 
//! fn main() {
//!     let mut t = TxtProcessor::from_str(String::from("This is text for example!"));
//!     t.replace_word("is", "replaced is");
//!     assert_eq!(1, t.get_lines());
//!     assert_eq!(5, t.get_words());
//!     assert_eq!("This replaced is text for example!", t.get_content());
//! }
//! 
//! ```
/// TxtProcessor module
pub mod processor;
pub use processor::*;
