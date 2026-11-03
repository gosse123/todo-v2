use crate::errors::TodoError;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub status: Etat,
    pub createat: DateTime<Utc>,
    pub updateat: Option<DateTime<Utc>>,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Etat {
    Todo,
    Inprogress,
    Done,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaskUpdate {
    pub name: String,
    pub description: String,
    pub due_date: NaiveDate,
}

impl Task {
    pub fn new(name: String, description: String) -> Result<Task, TodoError> {
        if name.trim().is_empty() {
            return Err(TodoError::NomInvalide);
        }
        Ok(Task {
            name,
            description,
            status: Etat::Todo,
            createat: Utc::now(),
            updateat: None,
            due_date: None,
        })
    }
}
