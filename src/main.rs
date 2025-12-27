pub mod commands;
pub mod fs;
pub mod helper;
pub mod history;
pub mod io_config;

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

        let Ok((cmd, mut out_buf, mut err_buf, is_redirected)) = io_config::setup_redirs(cmd_line) else{
            panic!("Panic");
        };

        let mut args = cmd.split_whitespace();
        let (Some(cmd), args) = (
            args.next(),
            args.map(|s| s.to_owned()).collect::<Vec<String>>(),
        ) else {
            panic!("WTF!")
        };
        if let Ok(c) = ShellCommand::parse(cmd, &args) {
            let should_cont = c.run(
                &args,
                &mut out_buf,
                &mut err_buf,
                rl.history_mut(),
                &mut history,
            )?;

            if !should_cont {
                break;
            }
        } else {
            let path = PathCollection::build().unwrap();
            if path.find(cmd.to_string()).is_some() {
                if is_redirected {
                    // When redirected, capture output and write to buffers
                    let child = Command::new(cmd)
                        .args(args)
                        .stdout(std::process::Stdio::piped())
                        .stderr(std::process::Stdio::piped())
                        .spawn()
                        .expect("Failed to start command");
                    
                    let output = child.wait_with_output()?;
                    if !output.stdout.is_empty() {
                        out_buf.write_all(&output.stdout)?;
                    }
                    if !output.stderr.is_empty() {
                        err_buf.write_all(&output.stderr)?;
                    }
                } else {
                    // When not redirected, let command write directly to terminal
                    let mut child = Command::new(cmd)
                        .args(args)
                        .spawn()
                        .expect("Failed to start command");
                    
                    child.wait()?;
                }
            } else {
                writeln!(err_buf, "{cmd}: command not found")?;
            }
        };
        
        // Flush the output buffers
        out_buf.flush().ok();
        err_buf.flush().ok();
    }
    if let Ok(hist_file) = env::var("HISTFILE") {
        history.write_file(&hist_file)?;
    }
    Ok(())
}
