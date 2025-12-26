use rustyline::{Helper, completion::Completer, highlight::Highlighter, hint::Hinter, validate::Validator};

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
            let possible_commands = vec!["echo","exit"];
            let mut compl: Vec<String> = Vec::new();
            for cand in possible_commands {
                if cand.starts_with(&line[..pos]){
                    compl.push(cand.to_string());
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
