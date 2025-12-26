#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut buff = String::new();
    let _ = io::stdin().read_line(&mut buff);
    let command = buff.trim();
    print!("{command}: command not found");
}
