use rustyline::{
    Helper, completion::Completer, highlight::Highlighter, hint::Hinter, validate::Validator,
};

use crate::{commands::ShellCommand, fs::PathCollection};

pub struct AutoComplHelper {}
impl AutoComplHelper {
    pub(crate) fn default() -> Option<AutoComplHelper> {
        Some(AutoComplHelper {})
    }
}

impl Helper for AutoComplHelper {}

impl Completer for AutoComplHelper {
    type Candidate = String;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let mut compl: Vec<String> = Vec::new();
        for command in ShellCommand::COMMANDS {
            let cand = command.to_str();
            if cand.starts_with(&line[..pos]) {
                compl.push(cand.to_string() + " ");
            }
        }
        if !compl.is_empty() {
            return Ok((0, compl));
        }
        let files = PathCollection::build().unwrap().list();
        for file in files {
            if file.starts_with(&line[..pos]) {
                compl.push(file.to_string());
            }
        }
        if compl.len() == 1 {
            compl = compl
                .iter()
                .map(|s| (s.to_owned() + " ").to_string())
                .collect()
        }
        compl.sort();
        Ok((0, compl))
    }
}

impl Validator for AutoComplHelper {}

impl Highlighter for AutoComplHelper {}

impl Hinter for AutoComplHelper {
    type Hint = &'static str;
}
