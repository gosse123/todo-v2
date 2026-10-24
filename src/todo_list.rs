use crate::errors::TodoError;
use crate::task::{Etat,Task};

pub fn remove_task(tasks: &mut Vec<Task>, name: &str)->Result<Task,TodoError> {
    let index = tasks.iter().position(|t|t.name == name).ok_or_else(||TodoError::TaskNotFound(name.to_string()))?;

    Ok(tasks.remove(index))
}


