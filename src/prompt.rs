use crate::errors::TodoError;
use dialoguer::Input;

pub trait Prompter {
    fn ask(&self, label: &str) -> Result<String, TodoError>;
}

pub struct DialoguerPrompter;

impl Prompter for DialoguerPrompter {
    fn ask(&self, label: &str) -> Result<String, TodoError> {
        let answer = Input::new().with_prompt(label).interact_text()?;
        Ok(answer)
    }
}
