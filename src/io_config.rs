use std::{fs::OpenOptions, io::Write};

use regex::{Error, Regex};

pub(crate) fn setup_redirs(
    cmd_line: String,
) -> Result<(String, Box<dyn Write>, Box<dyn Write>), Error> {

    let redir_regex = Regex::new("(.*) (>|1>|>>|1>>|2>|2>>) (.*)").expect("Wrong regexp");    
    let mut stdout: Box<dyn Write> = Box::new(std::io::stdout());
    let mut stderr: Box<dyn Write> = Box::new(std::io::stderr());
    let mut cmd = cmd_line.clone();
    if redir_regex.is_match(&cmd_line) {
        let Some(cap) = redir_regex.captures(&cmd_line) else {
            return Err(regex::Error::Syntax("No match".to_string()));
        };
        let (Some(cmd_val), Some(redir), Some(file_name)) = ((cap.get(1)),cap.get(2), (cap.get(3))) else {
            panic!("lalala");
        };
        let mut file = &mut OpenOptions::new();        
        file = file
            .create(true)
            .write(true);
            
        match redir.as_str() {
            ">"|"1>"|"2>" => {
                file = file.truncate(true);
            }
            ">>"|"1>>"|"2>>" => {
                file = file.append(true);
            }
            _ => {panic!("Unknown redir");}
        }
        
        let open_file = file
        .open(file_name.as_str().trim())
        .expect("Failed to create file");
        
        match redir.as_str() {
            ">"|"1>"|"1>>"|">>" => {
                stdout = Box::new(open_file);
            }
            "2>"|"2>>" => {
                stderr = Box::new(open_file);
            }
            _ => {panic!("Unknown redir");}
        }
        
        cmd = cmd_val.as_str().to_string();
    }
    Ok((cmd, stdout, stderr))
}
