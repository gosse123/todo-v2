use crate::task::Task;
use std::fs;
use std::io::Write;
use std::path::Path;
use colored::Colorize;

pub fn save(item:&[Task],path:&str)-> std::io::Result<()>{
    let json =serde_json::to_string_pretty(item)?;
    let mut file = fs::File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load(path: &str) -> std::io::Result<Vec<Task>> {
    let chemin = Path::new(path);
    if !chemin.exists() {
        return Ok(Vec::new()); // premier lancement : liste vide, pas d'erreur
    }
    let data = fs::read_to_string(path)?;
    let items = serde_json::from_str(&data).unwrap();
    Ok(items)
}


pub fn print_summary(tasks: &[Task]) {
    for (i, task) in tasks.iter().enumerate() {
        let n = i + 1;
        if task.completed {
            println!("{}. {} {}", n, "✓".green(), task.name.dimmed());
        } else {
            println!("{}. {} {}", n, "☐".blue(), task.name);
        }
    }
}

pub fn print_detail_by_name(tasks: &[Task], name: &str) {
    match tasks.iter().find(|t| t.name == name) {
        Some(task) => {
            println!("Tâche \"{}\":", task.name);
            println!("  Description : {}", task.description);
            println!("  Terminée    : {}", if task.completed { "oui" } else { "non" });
        }
        None => {
            eprintln!("Erreur : aucune tâche nommée \"{}\" n'existe.", name);
        }
    }
}
pub fn remove_task(tasks: &mut Vec<Task>, name: &str) {
    match tasks.iter().position(|t| t.name == name) {
        Some(index) => {
            tasks.remove(index);
            println!("Tâche \"{}\" supprimée.", name);
        }
        None => {
            eprintln!("Erreur : aucune tâche nommée \"{}\" n'existe.", name);
        }
    }
}