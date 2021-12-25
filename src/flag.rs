use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Flag {
    pub name: String,
    pub description: Option<String>,
    pub flag_type: FlagType,
    pub alias: Option<Vec<String>>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum FlagType {
    Bool,
    String,
    Int,
    Float,
}

#[derive(PartialEq, Clone, Debug)]
pub enum FlagValue {
    Bool(bool),
    String(String),
    Int(isize),
    Float(f64),
}

impl Flag {
    
}