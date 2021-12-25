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