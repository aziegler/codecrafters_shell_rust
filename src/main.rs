#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, os::unix::fs::PermissionsExt, path::Path, str::FromStr};

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

struct PathCollection{
    paths : Vec<String>
}

impl FromStr for PathCollection{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       Ok(
        PathCollection{
         paths : s.split(":").map(|s| s.to_owned()).collect()
        })
    }
}

impl PathCollection {
    fn build() -> Result<Self,&'static str> {
        let Ok(path) = env::var("PATH") else {
            return Err("Path not set");
        };
        PathCollection::from_str(&path)
    }

    fn find(&self, cmd: String) -> Option<String> {
        for path in &self.paths {
            let path = Path::new(&path).join(&cmd);
            if !path.exists() {
                continue;
            }
            let Ok(meta) = path.metadata() else { continue;};

            if meta.permissions().mode() & 0o111 != 0 {
                return path.to_str().map(|s| s.to_string());               
            }            
        }
        None
    }
}

fn main() {
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
                        let path = PathCollection::build().unwrap();
                        if let Some(full_path)= path.find(arg.to_string()){
                            println!("{arg} is {full_path}");
                        }else{
                            println!("{arg}: not found");
                        }
                    }
                },
            }
        }else{
            println!("{cmd}: command not found")
        };
        
    }
}
