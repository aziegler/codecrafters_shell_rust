use rustyline::{Helper, completion::Completer, highlight::Highlighter, hint::Hinter, validate::Validator};

use crate::{ShellCommand, fs::PathCollection};

pub struct AutoComplHelper{}
impl AutoComplHelper {
    pub(crate) fn default() -> Option<AutoComplHelper> {
       Some(AutoComplHelper {  })
    }
}

impl Helper for AutoComplHelper{
    
}

impl Completer for AutoComplHelper{
    type Candidate =String;
    fn complete(
            &self,
            line: &str,
            pos: usize,
            _ctx: &rustyline::Context<'_>,
        ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
            
            let mut compl: Vec<String> = Vec::new();
            for command in ShellCommand::COMMANDS {
                let cand = command.to_str();
                if cand.starts_with(&line[..pos]){
                    compl.push(cand.to_string()+" ");
                }
            }
            let files = PathCollection::build().unwrap().list();
            for file in files { 
                if file.starts_with(&line[..pos]){
                    compl.push(file.to_string()+" ");
                }
            }
            Ok((0, compl))
    }
}

impl Validator for AutoComplHelper{}

impl Highlighter for AutoComplHelper{}

impl Hinter for AutoComplHelper{
    type Hint = &'static str;
}
