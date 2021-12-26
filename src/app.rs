use crate::{Action, Command, Context, Flag, FlagType, Help};

#[derive(Default)]
pub struct App {
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub usage: Option<String>,
    pub version: Option<String>,
    pub commands: Option<Vec<Command>>,
    pub action: Option<Action>,
    pub flags: Option<Vec<Flag>>,
}

impl App {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into()
            ..Self::default()
        }
    }
}