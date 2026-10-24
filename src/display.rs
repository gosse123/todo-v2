use crate::task::{Task,Etat};
use colored::Colorize;

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


