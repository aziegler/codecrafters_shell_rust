#[allow(unused_imports)]
use std::io::{self, Write};
use std::str::FromStr;

enum Command{Echo,Exit,Type}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(Command::Exit),
            "echo" => Ok(Command::Echo),
            "type" => Ok(Command::Type),
            _ => Err("Not Found"),
        }
    }
}

fn main() {
    // TODO: Uncomment the code below to pass the first stage
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
        if let Ok(c) = cmd.parse::<Command>(){
            match c {
                Command::Echo => println!("{}",args.join(" ")),
                Command::Exit => return,
                Command::Type => {
                    let arg = args.first().unwrap();
                    if arg.to_owned().parse::<Command>().is_ok(){
                        println!("{arg} is a shell builtin");
                    }else{
                        println!("{arg}: not found");
                    }
                },
            }
        }else{
            println!("{cmd}: command not found")
        };
        
    }
}
