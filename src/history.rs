use std::{
    fs::{File, OpenOptions, read_to_string},
    io::Write,
};

use rustyline::{error::ReadlineError, history::{FileHistory, History}};

pub struct HistoryContainer {
    content: Vec<String>,
    mem_content: Vec<String>
}

impl  HistoryContainer  {
    pub fn new() -> Self {
        HistoryContainer {
            content: Vec::new(),
            mem_content: Vec::new(),           
        }
    }

    pub fn add(&mut self, hist: &mut FileHistory, cmd: String) {
        self.content.push(cmd.clone());
        self.mem_content.push(cmd.clone());
        hist.add(&cmd);
    }

    pub fn read_file(&mut self, path: &str, hist: &mut FileHistory) -> Result<(), ReadlineError> {
        let contents = read_to_string(path)?;
        contents.lines().for_each(|l| {
            if !l.is_empty() {
                self.add(hist,l.to_string());
            }
        });
        Ok(())
    }

    fn append_to_file(&mut self, path: &str) -> Result<(),ReadlineError> {
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        self.mem_content.iter().for_each(|l| {
            let _ = writeln!(file, "{}", l);
        });
        self.mem_content = Vec::new();
        Ok(())
    }

    fn write_file(&self, file_name: &str) -> Result<(), ReadlineError> {
        let mut file = File::create(file_name)?;
        self.content.iter().for_each(|l| {
            let _ = writeln!(file, "{}", l);
        });
        Ok(())
    }

    pub fn display(&self, count: usize) {
        self.content
            .iter()
            .enumerate()
            .rev()
            .take(count)
            .rev()
            .for_each(|(idx, command)| {
                let loc = idx + 1;
                println!("    {loc} {command}");
            });
    }

    pub fn run(&mut self, args: Vec<&str>,  hist: &mut FileHistory) -> Result<(), ReadlineError> {
        let mut length: usize = self.content.len();
        if let Some(first_arg) = args.first() {
            match *first_arg {
                "-r" => {
                    let Some(file_name) = args.get(1) else {
                        return Err(ReadlineError::Interrupted);
                    };
                    self.read_file(file_name, hist)?;
                    return Ok(());
                }
                "-w" => {
                    let Some(file_name) = args.get(1) else {
                        return Err(ReadlineError::Interrupted);
                    };
                    self.write_file(file_name)?;
                    return Ok(());
                }
                "-a" => {
                    let Some(file_name) = args.get(1) else {
                        return Err(ReadlineError::Interrupted);
                    };
                    self.append_to_file(file_name)?;
                    return Ok(());
                }
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

