use crate::task::{Etat, Task};
use colored::Colorize;

pub fn print_summary(tasks: &[Task]) -> String {
    let mut summary = String::new();
    for (i, task) in tasks.iter().enumerate() {
        let number = i + 1;
        let (marker, name) = match task.status {
            Etat::Done => ("✓".green().to_string(), task.name.dimmed().to_string()),
            Etat::Inprogress => ("o".yellow().to_string(), task.name.dimmed().to_string()),
            Etat::Todo => ("☐".blue().to_string(), task.name.clone()),
        };
        if !summary.is_empty() {
            summary.push('\n');
        }
        summary.push_str(&format!("{number}. {marker} {name}"));
    }
    summary
}

pub fn print_detail_by_name(tasks: &[Task], name: &str) -> Option<String> {
    let task = tasks.iter().find(|t| t.name == name)?;

    let mut detail = String::new();
    detail.push_str(&format!("Tâche \"{}\":\n", task.name));
    detail.push_str(&format!("Description : {}\n", task.description));
    detail.push_str(&format!(
        "Date de creation de la tâche: {}\n",
        task.createat
    ));
    if let Some(updateat) = task.updateat {
        detail.push_str(&format!("Dernire mise a jour de la tâche: {updateat}\n"));
    }
    detail.push_str(&format!(
        "status : {}\n",
        match task.status {
            Etat::Todo => "non terminé",
            Etat::Inprogress => "en cours",
            Etat::Done => "terminé",
        }
    ));
    match task.due_date {
        Some(due_date) => detail.push_str(&format!("date d'échance: {due_date}")),
        None => detail.push_str("pas de date d'échance"),
    }
    Some(detail)
}
