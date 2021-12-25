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

    pub fn new<T: Into<String>>(name: T, flag_type: FlagType) -> Self {
        let name = name.into();

        Self {
            name,
            description: None,
            flag_type,
            alias: None,
        }
    }

    pub fn description<T: Into<String>>(mut self, description: T) -> Self {
        self.description = Some(description.into());
        self
    }
}