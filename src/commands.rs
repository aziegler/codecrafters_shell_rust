use std::{
    env::{self, Args}, io::Write, str::FromStr
};

use rustyline::{error::ReadlineError, history::FileHistory};

use crate::{fs::PathCollection, history::HistoryContainer};

pub enum ShellCommand {
    Echo { args: Vec<String> },
    Exit,
    Type { arg: String },
    History { args: [Option<String>; 2] },
    PWD,
    CD { arg: String },
}

impl ShellCommand {
    pub fn to_str(&self) -> &str {
        match self {
            ShellCommand::Echo { args: _ } => "echo",
            ShellCommand::Exit => "exit",
            ShellCommand::Type { arg: _ } => "type",
            ShellCommand::History { args: __ } => "history",
            ShellCommand::PWD => "pwd",
            ShellCommand::CD { arg: _ } => "cd",
        }
    }

    fn take_arg(args: &Vec<String>) -> Result<String, ReadlineError> {
        let arg = args.first();
        match arg {
            Some(arg) => Ok(arg.clone()),
            None => Err(ReadlineError::Eof),
        }
    }

    fn take_2_opt_arg(args: &Vec<String>) -> [Option<String>; 2] {
        [args.first().cloned(), args.get(1).cloned()]
    }

    pub fn parse(s: &str, args: &Vec<String>) -> Result<Self, ReadlineError> {
        match s {
            "exit" => Ok(ShellCommand::Exit),
            "echo" => Ok(ShellCommand::Echo {
                args: args.to_vec(),
            }),
            "type" => Ok(ShellCommand::Type {
                arg: ShellCommand::take_arg(args)?,
            }),
            "history" => Ok(ShellCommand::History {
                args: ShellCommand::take_2_opt_arg(args),
            }),
            "pwd" => Ok(ShellCommand::PWD),
            "cd" => Ok(ShellCommand::CD {
                arg: ShellCommand::take_arg(args)?,
            }),
            _ => Err(ReadlineError::Eof),
        }
    }

    pub const COMMANDS: [Self; 6] = [
        ShellCommand::Echo { args: vec![] },
        ShellCommand::Exit,
        ShellCommand::Type { arg: String::new() },
        ShellCommand::History { args: [None, None] },
        ShellCommand::PWD,
        ShellCommand::CD { arg: String::new() },
    ];

    pub fn run(
        &self,
        args: &Vec<String>,
        out: &mut Box<dyn Write>,
        err: &mut Box<dyn Write>,
        history: &mut FileHistory,
        history_container: &mut HistoryContainer,
    ) -> Result<bool, ReadlineError> {
        match self {
            ShellCommand::Echo { args } => {
                writeln!(out, "{}", args.join(" "))?;
                Ok(true)
            }
            ShellCommand::Exit => Ok(false),
            ShellCommand::Type { arg } => {
                if ShellCommand::parse(arg, args).is_ok() {
                    writeln!(out, "{arg} is a shell builtin")?;
                } else {
                    let path = PathCollection::build().unwrap();
                    if let Some(full_path) = path.find(arg.to_string()) {
                        writeln!(out, "{arg} is {full_path}")?;
                    } else {
                        writeln!(err, "{arg}: not found")?;
                    }
                }
                Ok(true)
            }
            ShellCommand::History { args } => {
                history_container.run(args.clone(), history)?;
                Ok(true)
            }
            ShellCommand::PWD => {
                let dir = env::current_dir()?;
                let Some(dir) = dir.to_str() else {
                    return Err(ReadlineError::Interrupted);
                };
                writeln!(out, "{dir}")?;
                Ok(true)
            }
            ShellCommand::CD { arg } => {
                if *arg == "~" {
                    let Ok(home) = env::var("HOME") else {
                        writeln!(err, "HOME should be set")?;
                        return Ok(true);
                    };
                    env::set_current_dir(home)?;
                    return Ok(true);
                }
                let change = env::set_current_dir(arg);
                match change {
                    Ok(_) => Ok(true),
                    Err(_) => {
                        writeln!(err, "cd: {arg}: No such file or directory")?;
                        Ok(true)
                    }
                }
            }
        }
    }
}
