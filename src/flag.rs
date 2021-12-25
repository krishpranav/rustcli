use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Flag {
    pub name: String,
    pub description: Option<String>,
    pub flag_type: FlagType,
    pub alias: Option<Vec<String>>,
}
