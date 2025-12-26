pub mod fs;
pub mod history;

#[allow(unused_imports)]
use std::io::{self, Write};
use std::{process::Command, str::FromStr};

use rustyline::{DefaultEditor, error::ReadlineError};

use crate::{fs::PathCollection, history::HistoryContainer};

enum ShellCommand {
    Echo,
    Exit,
    Type,
    History,
}

impl FromStr for ShellCommand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(ShellCommand::Exit),
            "echo" => Ok(ShellCommand::Echo),
            "type" => Ok(ShellCommand::Type),
            "history" => Ok(ShellCommand::History),
            _ => Err("Not Found"),
        }
    }
}

fn main() -> Result<(), ReadlineError> {
    let mut rl = DefaultEditor::new()?;
    let mut history = HistoryContainer::new();
    
    loop {
        let cmd_line = rl.readline("$ ")?;

        let mut args = cmd_line.split_whitespace();
        let (Some(cmd), args) = (args.next(), args.collect::<Vec<&str>>()) else {
            panic!("WTF!")
        };
        history.add(rl.history_mut(), cmd_line.clone());
        if let Ok(c) = cmd.parse::<ShellCommand>() {
            match c {
                ShellCommand::Echo => println!("{}", args.join(" ")),
                ShellCommand::Exit => return Ok(()),
                ShellCommand::Type => {
                    let arg = args.first().unwrap();
                    if arg.to_owned().parse::<ShellCommand>().is_ok() {
                        println!("{arg} is a shell builtin");
                    } else {
                        let path = PathCollection::build().unwrap();
                        if let Some(full_path) = path.find(arg.to_string()) {
                            println!("{arg} is {full_path}");
                        } else {
                            println!("{arg}: not found");
                        }
                    }
                }
                ShellCommand::History => {
                    history.run(args, rl.history_mut())?;
                }
            }
        } else {
            let path = PathCollection::build().unwrap();
            if path.find(cmd.to_string()).is_some() {
                let _ = Command::new(cmd).args(args).spawn().expect("CMD").wait();
            } else {
                println!("{cmd}: command not found")
            }
        };
    }
}
