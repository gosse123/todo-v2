use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub status: Etat,
    pub createat: DateTime<Utc>,
    pub updateat: Option<DateTime<Utc>>,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Etat {
    Todo,
    Inprogress,
    Done,
}

impl Task {
    pub fn new(nom: String, des: String) -> Task {
        Task {
            name: nom,
            description: des,
            status: Etat::Todo,
            createat: Utc::now(),
            updateat: None,
            due_date: None,
        }
    }
}
