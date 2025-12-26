#[allow(unused_imports)]
use std::io::{self, Write};

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
        match cmd {
            "exit" => return,
            "echo" => println!("{}",args.join(" ")),
            c => println!("{c}: command not found")
        }
    }
}
