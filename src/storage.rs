use crate::task::Task;
use std::fs;
use std::io::Write;
use std::path::Path;
use colored::Colorize;

pub fn save(item:&Task,path:&str)-> std::io::Result<()>{
    let chemin = Path::new(&path);

    if let Some(dossier_parent) = chemin.parent(){
        fs::create_dir_all(dossier_parent)?;
    }
    let json =serde_json::to_string_pretty(item)?;
    let mut file = fs::File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load(path: &str) -> std::io::Result<Vec<String>> {
    let data = fs::read_to_string(path)?;
    let items: Vec<String> = serde_json::from_str(&data).unwrap();
    Ok(items)
}

pub fn print_list(tasks: &[Task]) {
    for (i, task) in tasks.iter().enumerate() {
        let n = i + 1;
        if task.completed {
            println!(
                "{}. {} {} {}",
                n,
                "✓".green(),
                task.name.dimmed(),
                format!("({})", task.description).dimmed()
            );
        } else {
            println!(
                "{}. {} {} ({})",
                n,
                "☐".blue(),
                task.name,
                task.description
            );
        }
    }
}