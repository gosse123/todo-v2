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

#[cfg(test)]
mod tests {
    use super::{Etat, Task};
    use crate::errors::TodoError;

    #[test]
    fn une_nouvelle_tache_est_a_faire_sans_date_ni_mise_a_jour() {
        let task = Task::new("Acheter du pain".to_string(), "Au boulanger".to_string()).unwrap();

        assert_eq!(task.name, "Acheter du pain");
        assert_eq!(task.description, "Au boulanger");
        assert_eq!(task.status, Etat::Todo);
        assert!(task.updateat.is_none());
        assert!(task.due_date.is_none());
    }

    #[test]
    fn refuse_un_nom_vide() {
        let resultat = Task::new("   ".to_string(), "Description".to_string());

        assert!(matches!(resultat, Err(TodoError::NomInvalide)));
    }
}
