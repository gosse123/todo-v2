use crate::errors::TodoError;
use crate::task::{Etat, Task, TaskUpdate};
use chrono::Utc;

pub fn remove_task(tasks: &mut Vec<Task>, name: &str) -> Result<Task, TodoError> {
    let index = find_index(tasks, name).ok_or_else(|| TodoError::TaskNotFound(name.to_string()))?;
    Ok(tasks.remove(index))
}

pub fn as_done(tasks: &mut [Task], num: usize) -> Result<(), TodoError> {
    let index = num.checked_sub(1).ok_or(TodoError::IndexInvalide(num))?;
    let task = tasks.get_mut(index).ok_or(TodoError::IndexInvalide(num))?;
    task.status = Etat::Done;
    Ok(())
}

pub fn update_tasks(
    current_name: &str,
    update: TaskUpdate,
    tasks: &mut [Task],
) -> Result<(), TodoError> {
    if update.name.trim().is_empty() {
        return Err(TodoError::NomInvalide);
    }
    if update.description.trim().is_empty() {
        return Err(TodoError::DescriptionInvalide);
    }
    let index = find_index(tasks, current_name)
        .ok_or_else(|| TodoError::TaskNotFound(current_name.to_string()))?;
    let task = &mut tasks[index];
    task.name = update.name;
    task.description = update.description;
    task.due_date = Some(update.due_date);
    task.updateat = Some(Utc::now());
    Ok(())
}

fn find_index(tasks: &[Task], name: &str) -> Option<usize> {
    tasks.iter().position(|task| task.name == name)
}
