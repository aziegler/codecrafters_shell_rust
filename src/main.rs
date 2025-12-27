pub mod commands;
pub mod fs;
pub mod helper;
pub mod history;

#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env,
    io::{stderr, stdout},
    process::{Command, Output},
};

use rustyline::{Editor, config::Configurer, error::ReadlineError, history::FileHistory};

use crate::{
    commands::ShellCommand, fs::PathCollection, helper::AutoComplHelper, history::HistoryContainer,
};

fn main() -> Result<(), ReadlineError> {
    let mut rl: Editor<AutoComplHelper, FileHistory> = Editor::new()?;
    rl.set_helper(AutoComplHelper::default());
    rl.set_completion_type(rustyline::CompletionType::List);
    let mut history = HistoryContainer::new();
    if let Ok(hist_file) = env::var("HISTFILE") {
        history.read_file(hist_file.as_str(), rl.history_mut())?;
    }

    loop {
        let cmd_line = rl.readline("$ ")?;
        history.add(rl.history_mut(), cmd_line.clone());
        let mut args = cmd_line.split_whitespace();
        let (Some(cmd), args) = (
            args.next(),
            args.map(|s| s.to_owned()).collect::<Vec<String>>(),
        ) else {
            panic!("WTF!")
        };
        let mut out_buf = String::new();
        let mut err_buf = String::new();
        if let Ok(c) = ShellCommand::parse(cmd, &args) {
            // use String buffers which implement std::fmt::Write expected by commands

            let should_cont = c.run(
                &args,
                &mut out_buf,
                &mut err_buf,
                rl.history_mut(),
                &mut history,
            )?;
            // flush collected output to real stdout/stderr

            if !should_cont {
                break;
            }
        } else {
            let path = PathCollection::build().unwrap();
            if path.find(cmd.to_string()).is_some() {
                let output = Command::new(cmd).args(args).output()?;
                match (str::from_utf8(&output.stdout),str::from_utf8(&output.stdout)){
                    (Ok(out), Ok(err)) => {
                        out_buf = out.to_string();
                        err_buf = err.to_string();
                    },
                    _ => return Err(ReadlineError::Eof)
                }
            } else {
                println!("{cmd}: command not found")
            }
        };
        if !out_buf.is_empty() {
            print!("{}", out_buf);
            stdout().flush().ok();
        }
        if !err_buf.is_empty() {
            eprint!("{}", err_buf);
            stderr().flush().ok();
        }
    }
    if let Ok(hist_file) = env::var("HISTFILE") {
        history.write_file(&hist_file)?;
    }
    Ok(())
}
