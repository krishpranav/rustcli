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
        if name.starts_with('-') {
            panic!(
                r#""{}" is invalid flag name. Flag name cannnot start with "-"."#,
                name
            )
        }
        if name.contains('=') {
            panic!(
                r#""{}" is invalid flag name. Flag name cannnot contain "="."#,
                name
            )
        }
        if name.contains(' ') {
            panic!(
                r#""{}" is invalid flag name. Flag name cannnot contain blankspaces."#,
                name
            )
        }

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

    pub fn alias<T: Into<String>>(mut self, name: T) -> Self {
        if let Some(ref mut alias) = self.alias {
            (*alias).push(name.into());
        } else {
            self.alias = Some(vec![name.into()]);
        }
        self
    }

    pub fn option_index(&self, v: &[String]) -> Option<usize> {
        match &self.alias {
            Some(alias) => v.iter().position(|r| {
                r == &format!("--{}", &self.name) || alias.iter().any(|a| r == &format!("-{}", a))
            }),
            None => v.iter().position(|r| r == &format!("--{}", &self.name)),
        }
    }

}