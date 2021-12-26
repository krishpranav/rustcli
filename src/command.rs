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

    pub fn flag(mut self, flag: Flag) -> Self {
        if let Some(ref mut flags) = self.flags {
            (*flags).push(flag);
        } else {
            self.flags = Some(vec![flag]);
        }
        self
    }

    pub fn alias<T: Into<String>>(mut self, name: T) -> Self {
        if let Some(ref mut alias) = self.alias {
            (*alias).push(name.into())
        } else {
            self.alias = Some(vec![name.into()]);
        }
        self
    }

    pub fn run(&self, args: Vec<String>) {
        if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
            self.help();
            return;
        }
        match self.action {
            Some(action) => action(&Context::new(args, self.flags.clone(), self.help_text())),
            None => self.help(),
        }
    }
}