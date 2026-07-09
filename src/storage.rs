use crate::task::Task;
use std::fs;
use std::io::Write;
use std::path::{PathBuf};
use colored::Colorize;
use directories::ProjectDirs;
use crate::errors::TodoError;

fn data_file()->Result<PathBuf,TodoError>{
    let proj_dirs = match ProjectDirs::from("com","nahounou","todo-gosse")  {
        Some(value)=>value,
        None=>return Err(TodoError::AppDireError)
    };
    let data_dir = proj_dirs.data_dir();

    fs::create_dir_all(data_dir)?;
    Ok(data_dir.join("tasks.json"))
}

pub fn save(item:&[Task])-> Result<(),TodoError>{
    let json =serde_json::to_string_pretty(item)?;
    let path:PathBuf =  data_file()?;
    let mut file = fs::File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load() -> Result<Vec<Task>,TodoError> {
    let path:PathBuf =  data_file()?;
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
        if task.completed {
            println!("{}. {} {}", n, "✓".green(), task.name.dimmed());
        } else {
            println!("{}. {} {}", n, "☐".blue(), task.name);
        }
    }
}
pub fn as_done(tasks: &mut [Task],num:usize){
    for (i , task) in tasks.iter_mut().enumerate() {
        if i == num-1{
            task.completed = true;
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