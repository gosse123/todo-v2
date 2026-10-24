use crate::errors::TodoError;
use crate::task::{Etat, Task};
use chrono::{NaiveDate, Utc};
use colored::Colorize;
use directories::ProjectDirs;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn data_file() -> Result<PathBuf, TodoError> {
    let proj_dirs = match ProjectDirs::from("com", "nahounou", "todo-gosse") {
        Some(value) => value,
        None => return Err(TodoError::AppDireError),
    };
    let data_dir = proj_dirs.data_dir();

    fs::create_dir_all(data_dir)?;
    Ok(data_dir.join("tasks.json"))
}

pub fn save(item: &[Task]) -> Result<(), TodoError> {
    let json = serde_json::to_string_pretty(item)?;
    let path: PathBuf = data_file()?;
    let mut file = fs::File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load() -> Result<Vec<Task>, TodoError> {
    let path: PathBuf = data_file()?;
    if !path.exists() {
        return Ok(Vec::new()); // premier lancement : liste vide, pas d'erreur
    }
    let data = fs::read_to_string(path)?;
    let items = serde_json::from_str(&data)?;
    Ok(items)
}

pub fn print_summary(tasks: &[Task]) {
    for (i, task) in tasks.iter().enumerate() {
        let n = i + 1;
        match task.status {
            Etat::Done => println!("{}. {} {}", n, "✓".green(), task.name.dimmed()),
            Etat::Inprogress => println!("{}. {} {}", n, "o".yellow(), task.name.dimmed()),
            Etat::Todo => println!("{}. {} {}", n, "☐".blue(), task.name),
        }
    }
}


pub fn print_detail_by_name(tasks: &[Task], name: &str) {
    match tasks.iter().find(|t| t.name == name) {
        Some(task) => {
            println!("Tâche \"{}\":", task.name);
            println!("Description : {}", task.description);
            println!("Date de creation de la tâche: {}", task.createat);
            if let Some(temps) = task.updateat {
                println!("Dernire mise a jour de la tâche: {}", temps);
            }
            println!(
                "status : {}",
                match &task.status {
                    Etat::Todo => "non terminé",
                    Etat::Inprogress => "en cours",
                    Etat::Done => "terminé",
                }
            );
            if let Some(due_date) = task.due_date {
                println!("date d'échance: {due_date}")
            } else {
                println!("pas de date d'échance")
            }
        }
        None => {
            eprintln!("Erreur : aucune tâche nommée \"{}\" n'existe.", name);
        }
    }
}


pub fn update_tasks(
    hold_new: &str,
    new_name: &str,
    new_des: &str,
    tasks: &mut Vec<Task>,
    due_date: &str,
) -> bool {
    let mut found = false;
    if new_name.trim().is_empty() || new_name.trim().is_empty() || due_date.trim().is_empty() {
        found = false;
    } else {
        for task in tasks {
            if task.name == hold_new {
                found = true;
                task.name = new_name.to_string();

                task.description = new_des.to_string();
                if let Ok(valde) = parse_date(due_date.trim()) {
                    task.due_date = Some(valde)
                } else {
                    println!("format invalide utilise se format: %Y-%m-%d")
                }
                task.updateat = Some(Utc::now());
            }
        }
    }
    found
}

fn parse_date(date: &str) -> Result<NaiveDate, String> {
    match NaiveDate::parse_from_str(date, "%d-%m-%Y") {
        Ok(valide) => Ok(valide),
        Err(_) => return Err("format invalide".to_string()),
    }
}
