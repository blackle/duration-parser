use std::fmt;
use std::error::Error as StdError;

#[derive(Debug, PartialEq)]
pub enum Error {
    Overflow,
    NoData,
    UnknownSuffix,
    UnparsableNumber,
    UnparsableExtras,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Overflow => f.write_str("Overflow"),
            Error::NoData => f.write_str("NoData"),
            Error::UnknownSuffix => f.write_str("UnknownSuffix"),
            Error::UnparsableNumber => f.write_str("UnparsableNumber"),
            Error::UnparsableExtras => f.write_str("UnparsableExtras"),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Overflow => "Duration too big",
            Error::NoData => "Duration empty",
            Error::UnknownSuffix => "Duration contains unknown unit suffix",
            Error::UnparsableNumber => "Duration contains unparsable number",
            Error::UnparsableExtras => "Duration contains extra, erroneous characters",
        }
    }
}
