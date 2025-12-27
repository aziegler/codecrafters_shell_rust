use std::{fs::OpenOptions, io::Write};
use std::fs::File;

use regex::{Error, Regex};

pub(crate) fn setup_redirs(cmd_line: String) -> Result<(String, Box<dyn Write>, Box<dyn Write>),Error> {
    let out_regex = Regex::new("(.*) (1>|>) (.*)")?;
    let err_regex = Regex::new("(.*) 2> (.*)")?;
    let mut stdout: Box<dyn Write> = Box::new(std::io::stdout());
    let mut stderr:  Box<dyn Write> = Box::new(std::io::stderr());
    let mut cmd = cmd_line.clone();
    if out_regex.is_match(&cmd_line) {
        let Some(cap) = out_regex.captures(&cmd_line) else{
            return Err(regex::Error::Syntax("No match".to_string()));
        };
        let (Some(cmd_val),Some(file)) = ((cap.get(1)),(cap.get(3))) else{
            panic!("lalala");
        };
        let file = OpenOptions::new().create(true).write(true).truncate(true).open(file.as_str().trim()).expect("Failed to create file");
        stdout = Box::new(file);
        cmd = cmd_val.as_str().to_string();
    } 
    if err_regex.is_match(&cmd_line){
        let Some(cap) = err_regex.captures(&cmd_line) else{
            return Err(regex::Error::Syntax("No match".to_string()));
        };
        let (Some(cmd_val),Some(file)) = ((cap.get(1)),(cap.get(2))) else{
            panic!("Incorrect regex");
        };
        let file = OpenOptions::new().create(true).write(true).truncate(true).open(file.as_str().trim()).expect("Failed to create file");
        stderr = Box::new(file);
        cmd = cmd_val.as_str().to_string();
    }
    Ok((cmd, stdout, stderr))
}