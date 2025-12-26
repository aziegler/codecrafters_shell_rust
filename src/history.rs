use std::{fs::{File, read_to_string}, io::Write};

use rustyline::error::ReadlineError;


pub struct History  {
    content : Vec<String>
}

impl History   {

    pub fn new() -> Self {
        History { content: Vec::new() }
    }

    pub fn add(&mut self, cmd : String) {
        self.content.push(cmd);
    }

    pub fn append_file(&mut self, path:&str)-> Result<(),ReadlineError>{
            let contents = read_to_string(path)?;
            contents.lines().for_each(|l| {
                if !l.is_empty(){
                    self.content.push(l.to_string());
                }
            });
            Ok(())
    }

     fn write_file(&self, file_name: &str) -> Result<(),ReadlineError> {
        let mut file = File::create(file_name)?;
        self.content.iter().for_each(|l| {
            let _ = writeln!(file,"{}",l);
        });
        Ok(())
    }

    pub fn display(&self, count:usize){
        self.content.iter().enumerate().rev().take(count).rev().for_each(|(idx,command)| {
                let loc = idx + 1;
                println!("    {loc} {command}");
        });
    }

    pub fn run(&mut self, args: Vec<&str>)-> Result<(),ReadlineError>{
        let mut length:usize = self.content.len();
        if let Some(first_arg) = args.first(){
            match *first_arg {
                "-r" => {
                        let Some(file_name) = args.get(1) else {
                            return Err(ReadlineError::Interrupted);
                        };
                    self.append_file(file_name)?;
                    return Ok(());
                },
                "-w"=> {
                    let Some(file_name) = args.get(1) else {
                            return Err(ReadlineError::Interrupted);
                    };
                    self.write_file(file_name)?;
                    return Ok(());
                },
                c => {
                   if let Ok(arg) = c.parse::<usize>() {
                        length = arg;
                     }    
                }
            }             
        }
        self.display(length);
        Ok(())
    }
    
   
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
