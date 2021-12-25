use crate::error::Error;
use crate::{Flag, FlagType, FlagValue};

pub struct Context {
    pub args: Vec<String>,
    flags: Option<Vec<(String, Result<FlagValue, Error>)>>,
    help_text: String,
}

impl Context {
    pub fn new(args: Vec<String>, flags: Option<Vec<Flag>>, help_text: String) -> Self {
        let mut v = Vec::new();
        let mut parsed_args = args;
        let flags_val = match flags {
            Some(flags) => {
                for flag in flags {
                    if let Some(index) = flag.option_index(&parsed_args) {
                        parsed_args.remove(index);

                        let val = if flag.flag_type != FlagType::Bool {
                            if parsed_args.len() <= index {
                                None
                            } else {
                                Some(parsed_args.remove(index))
                            }
                        } else {
                            None
                        };
                        v.push((flag.name.to_string(), flag.value(val)))
                    } else {
                        v.push((flag.name.to_string(), Err(Error::NotFound)))
                    }
                }
                Some(v)
            }
            None => None,
        };

        Self {
            args: parsed_args,
            flags: flags_val,
            help_text,
        }
    }

    fn result_flag_value(&self, name: &str) -> Result<FlagValue, Error> {
        let flag = self
            .flags
            .as_ref()
            .and_then(|flags| flags.iter().find(|flag| flag.0 == name));

        match flag {
            Some(f) => match &f.1 {
                Ok(val) => Ok(val.to_owned()),
                Err(e) => Err(e.to_owned()),
            },
            None => Err(Error::Undefined),
        }
    }

    pub fn bool_flag(&self, name: &str) -> bool {
        let r = self.result_flag_value(name);
        match r {
            Ok(FlagValue::Bool(val)) => val,
            _ => false,
        }
    }

    pub fn string_flag(&self, name: &str) -> Result<String, Error> {
        let r = self.result_flag_value(name)?;
        match r {
            FlagValue::String(val) => Ok(val),
            _ => Err(Error::TypeError),
        }
    }

    pub fn int_flag(&self, name: &str) -> Result<isize, Error> {
        let r = self.result_flag_value(name)?;
        match r {
            FlagValue::Int(val) => Ok(val),
            _ => Err(Error::TypeError),
        }
    }

    pub fn float_flag(&self, name: &str) -> Result<f64, Error> {
        let r = self.result_flag_value(name)?;
        match r {
            FlagValue::Float(val) => Ok(val),
            _ => Err(Error::TypeError),
        }
    }

    pub fn help(&self) {
        println!("{}", self.help_text);
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::{Context, Flag, FlagType};

    #[test]
    fn context_test() {
        let args = vec![
            "cli".to_string(),
            "command".to_string(),
            "args".to_string(),
            "--bool".to_string(),
            "--string".to_string(),
            "test".to_string(),
            "--int".to_string(),
            "100".to_string(),
            "--float".to_string(),
            "1.23".to_string(),
            "--invalid_float".to_string(),
            "invalid".to_string(),
        ];
        let flags = vec![
            Flag::new("bool", FlagType::Bool),
            Flag::new("string", FlagType::String),
            Flag::new("int", FlagType::Int),
            Flag::new("float", FlagType::Float),
            Flag::new("invalid_float", FlagType::Float),
            Flag::new("not_specified", FlagType::String),
        ];
        let context = Context::new(args, Some(flags), "".to_string());

        assert_eq!(context.bool_flag("bool"), true);
        assert_eq!(context.string_flag("string"), Ok("test".to_string()));
        assert_eq!(context.int_flag("int"), Ok(100));
        assert_eq!(context.float_flag("float"), Ok(1.23));

        assert_eq!(context.int_flag("string"), Err(Error::TypeError));

        assert_eq!(
            context.float_flag("invalid_float"),
            Err(Error::ValueTypeError)
        );

        assert_eq!(
            context.string_flag("not_registered"),
            Err(Error::Undefined)
        );

        assert_eq!(
            context.string_flag("not_specified"),
            Err(Error::NotFound)
        );
    }
}