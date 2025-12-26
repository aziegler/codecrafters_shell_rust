pub mod fs;
pub mod history;
pub mod helper;

#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, process::Command, str::FromStr};

use rustyline::{Editor, config::Configurer, error::ReadlineError, history::FileHistory};

use crate::{fs::PathCollection, helper::AutoComplHelper, history::HistoryContainer};

enum ShellCommand {
    Echo,
    Exit,
    Type,
    History,
    PWD,
    CD
}

impl ShellCommand{
    fn to_str(&self) -> &str {
        match self {
            ShellCommand::Echo => "echo",
            ShellCommand::Exit => "exit",
            ShellCommand::Type => "type",
            ShellCommand::History => "history",
            ShellCommand::PWD=> "pwd",
            ShellCommand::CD => "cd"
        }
    }

    

    const COMMANDS:[Self;5] = [ShellCommand::Echo, ShellCommand::Exit,ShellCommand::Type,ShellCommand::History, ShellCommand::PWD];
}

impl FromStr for ShellCommand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(ShellCommand::Exit),
            "echo" => Ok(ShellCommand::Echo),
            "type" => Ok(ShellCommand::Type),
            "history" => Ok(ShellCommand::History),
            "pwd" => Ok(ShellCommand::PWD),
            "cd" => Ok(ShellCommand::CD),
            _ => Err("Not Found"),
        }
    }
}

fn main() -> Result<(), ReadlineError> {
    let mut rl: Editor<AutoComplHelper, FileHistory> = Editor::new()?;
    rl.set_helper(AutoComplHelper::default());
    rl.set_completion_type(rustyline::CompletionType::List);
    let mut history = HistoryContainer::new();
    if let Ok(hist_file) =  env::var("HISTFILE"){
        history.read_file(hist_file.as_str(), rl.history_mut())?;
    }

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
                ShellCommand::Exit => break ,
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
                ShellCommand::PWD => {
                    let dir = env::current_dir()?;
                    let Some(dir) = dir.to_str() else{
                        panic!("Unknown path");
                    };
                    println!("{dir}");
                }
                ShellCommand::CD=>{
                    let arg = args.first().unwrap();
                    let change = env::set_current_dir(arg);
                    match change{
                        Ok(_) => (),
                        Err(_) => println!("cd: {arg}: No such file or directory"),
                    }
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
    if let Ok(hist_file) =  env::var("HISTFILE"){
        history.write_file(&hist_file)?;
    }
    Ok(())
}
