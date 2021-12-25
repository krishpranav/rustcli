use std::error;
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Errors {
    NotFound,
    Undefined,
    TypeError,
    ValueTypeError,
    ArgumentError,
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Errors::NotFound => f.write_str("NotFound"),
            Errors::Undefined => f.write_str("Undefined"),
            Errors::TypeError => f.write_str("TypeError"),
            Errors::ValueTypeError => f.write_str("ValueTypeError"),
            Errors::ArgumentError => f.write_str("ArgumentError"),
        }
    }
}

impl error::Error for Errors {
    fn description(&self) -> &str {
        match *self {
            Errors::NotFound => "Flag not found",
            Errors::Undefined => "Flag undefined",
            Errors::TypeError => "Flag type mismatch",
            Errors::ValueTypeError => "Value type mismatch",
            Errors::ArgumentError => "Illegal argument",
        }
    }
}