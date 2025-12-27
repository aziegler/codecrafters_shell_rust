use std::io::Write;
use std::fs::File;

use regex::{Error, Regex};

pub(crate) fn setup_redirs(cmd_line: String) -> Result<(String, Box<dyn Write>, Box<dyn Write>),Error> {
    let regexp = Regex::new("(.*)>(.*)")?;
    if regexp.is_match(&cmd_line) {
        let Some(cap) = regexp.captures(&cmd_line) else{
            return Err(regex::Error::Syntax("No match".to_string()));
        };
        let (Some(cmd),Some(file)) = ((cap.get(1)),(cap.get(2))) else{
            panic!("lalala");
        };
        let stdout = Box::new(File::create(file.as_str()).expect("File creation failed"));
        let stderr = Box::new(std::io::stderr());
        Ok((cmd.as_str().to_string(), stdout, stderr))
    } else {
        let stdout = Box::new(std::io::stdout());
        let stderr = Box::new(std::io::stderr());
        Ok((cmd_line, stdout, stderr))
    }
}