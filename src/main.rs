pub mod fs;

#[allow(unused_imports)]
use std::io::{self, Write};
use std::{process::Command, str::FromStr};

use crate::fs::PathCollection;

enum ShellCommand{Echo,Exit,Type,History}

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

fn main() {
    let mut history : Vec<String> = Vec::new();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buff = String::new();
        let _ = io::stdin().read_line(&mut buff);
        let cmd_line = buff.trim();
        let mut args = cmd_line.split_whitespace();
        let (Some(cmd),args) = (args.next(),args.collect::<Vec<&str>>())else{
            panic!("WTF!")
        };
        history.push(buff.clone());
        if let Ok(c) = cmd.parse::<ShellCommand>(){
            match c {
                ShellCommand::Echo => println!("{}",args.join(" ")),
                ShellCommand::Exit => return,
                ShellCommand::Type => {
                    let arg = args.first().unwrap();
                    if arg.to_owned().parse::<ShellCommand>().is_ok(){
                        println!("{arg} is a shell builtin");
                    }else{
                        let path = PathCollection::build().unwrap();
                        if let Some(full_path)= path.find(arg.to_string()){
                            println!("{arg} is {full_path}");
                        }else{
                            println!("{arg}: not found");
                        }
                    }
                },
                ShellCommand::History => {
                    history.iter().enumerate().for_each(|(idx,command)| {
                        let loc = idx + 1;
                        print!("    {loc} {command}");
                    });
                },
            }
        }else{
            let path = PathCollection::build().unwrap();
            if path.find(cmd.to_string()).is_some(){
                let _ = Command::new(cmd).args(args).spawn().expect("CMD").wait();
             
            }else{
                println!("{cmd}: command not found")
            }
        };
        
    }
}
