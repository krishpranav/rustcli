use crate::error::Error;
use crate::{Flag, FlagType, FlagValue};

pub struct Context {
    pub args: Vec<String>,
    flags: Option<Vec<(String, Result<FlagValue, FlagError>)>>,
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
                        v.push((flag.name.to_string(), Err(FlagError::NotFound)))
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

    fn result_flag_value(&self, name: &str) -> Result<FlagValue, FlagError> {
        let flag = self
            .flags
            .as_ref()
            .and_then(|flags| flags.iter().find(|flag| flag.0 == name));

        match flag {
            Some(f) => match &f.1 {
                Ok(val) => Ok(val.to_owned()),
                Err(e) => Err(e.to_owned()),
            },
            None => Err(FlagError::Undefined),
        }
    }
}