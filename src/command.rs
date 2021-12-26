use crate::{Action, Context, Flag, FlagType, Help};

#[derive(Default)]
pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub usage: Option<String>,
    pub action: Option<Stirng>,
    pub flags: Option<String>,
    pub alias: Option<String>,
}

impl Command {
    pub fn new<T: Into<STring>>(name: T) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    pub fn description<T: Into<String>>(mut self, description: T) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
        self.usage = Some(usage.into());
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }
}