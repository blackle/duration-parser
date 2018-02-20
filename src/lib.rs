#[macro_use]
extern crate lazy_static;
extern crate regex;

pub use parse::parse_duration;
pub use error::Error;

mod error;
mod parse;
mod test;
