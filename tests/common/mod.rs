use std::cell::RefCell;
use std::collections::VecDeque;
use todo_gosse::errors::TodoError;
use todo_gosse::prompt::Prompter;

pub struct FakePrompter {
    answers: RefCell<VecDeque<String>>,
}

impl FakePrompter {
    pub fn new(answers: &[&str]) -> Self {
        let answers = answers.iter().map(|answer| answer.to_string()).collect();
        FakePrompter {
            answers: RefCell::new(answers),
        }
    }
}

impl Prompter for FakePrompter {
    fn ask(&self, _label: &str) -> Result<String, TodoError> {
        Ok(self
            .answers
            .borrow_mut()
            .pop_front()
            .expect("le FakePrompter n'a plus de réponse"))
    }
}
