use std::{
    fs::{File, OpenOptions, read_to_string},
    io::Write,
};

use rustyline::{
    error::ReadlineError,
    history::{FileHistory, History},
};

pub struct HistoryContainer {
    content: Vec<String>,
    mem_content: Vec<String>,
}

impl HistoryContainer {
    pub fn new() -> Self {
        HistoryContainer {
            content: Vec::new(),
            mem_content: Vec::new(),
        }
    }

    pub fn add(&mut self, hist: &mut FileHistory, cmd: String) {
        self.content.push(cmd.clone());
        self.mem_content.push(cmd.clone());
        let _ = hist.add(&cmd);
    }

    pub fn read_file(&mut self, path: &str, hist: &mut FileHistory) -> Result<(), ReadlineError> {
        let contents = read_to_string(path)?;
        contents.lines().for_each(|l| {
            if !l.is_empty() {
                self.add(hist, l.to_string());
            }
        });
        Ok(())
    }

    fn append_to_file(&mut self, path: &str) -> Result<(), ReadlineError> {
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        self.mem_content.iter().for_each(|l| {
            let _ = writeln!(file, "{}", l);
        });
        self.mem_content = Vec::new();
        Ok(())
    }

    pub fn write_file(&self, file_name: &str) -> Result<(), ReadlineError> {
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

    pub fn run(
        &mut self,
        args: [Option<String>; 2],
        hist: &mut FileHistory,
    ) -> Result<(), ReadlineError> {
        match args {
            [None, None] => self.display(self.content.len()),
            [None, Some(_)] => return Err(ReadlineError::Eof),
            [Some(c), None] => {
                if let Ok(arg) = c.parse::<usize>() {
                    self.display(arg);
                } else {
                    return Err(ReadlineError::Eof);
                }
            }
            [Some(flag), Some(filename)] => match flag.as_str() {
                "-r" => {
                    self.read_file(&filename, hist)?;
                }
                "-w" => {
                    self.write_file(&filename)?;
                }
                "-a" => {
                    self.append_to_file(&filename)?;
                }
                _ => return Err(ReadlineError::Eof),
            },
        }
        Ok(())
    }
}

impl Default for HistoryContainer {
    fn default() -> Self {
        Self::new()
    }
}
