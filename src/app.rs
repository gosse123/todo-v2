use crate::cli::{Action, Cli};
use crate::date;
use crate::display;
use crate::errors::TodoError;
use crate::prompt::Prompter;
use crate::storage;
use crate::task::{Task, TaskUpdate};
use crate::todo_list;
use std::path::Path;

pub fn run(cli: &Cli, path: &Path, prompter: &dyn Prompter) -> Result<(), TodoError> {
    let mut tasks = storage::load(path)?;

    match &cli.mode {
        Some(Action::Affiche { name_aff }) => {
            if name_aff.is_empty() {
                let summary = display::print_summary(&tasks);
                if !summary.is_empty() {
                    println!("{summary}");
                }
            } else {
                for name in name_aff {
                    match display::print_detail_by_name(&tasks, name) {
                        Some(detail) => println!("{detail}"),
                        None => eprintln!("Erreur : aucune tâche nommée \"{name}\" n'existe."),
                    }
                }
            }
        }
        Some(Action::Ajoute) => {
            let name = prompter.ask("Nom de la tâche")?.trim().to_string();
            let description = prompter.ask("Description")?.trim().to_string();
            tasks.push(Task::new(name, description));
            storage::save(path, &tasks)?;
        }
        Some(Action::Supprimer { rm_nom }) => {
            for name in rm_nom {
                let _ = todo_list::remove_task(&mut tasks, name);
            }
            storage::save(path, &tasks)?;
        }
        Some(Action::Faire { indices }) => {
            for num in indices {
                todo_list::as_done(&mut tasks, *num)?;
            }
            storage::save(path, &tasks)?;
        }
        Some(Action::Modifie { update_name }) => {
            let new_name = prompter
                .ask(&format!("Entrée le nouveau nom de {update_name}"))?
                .trim()
                .to_string();
            let new_description = prompter
                .ask(&format!("Entrée la nouvelle description de {update_name}"))?
                .trim()
                .to_string();
            let due_date_input = prompter.ask("Entrée la date d'échance format(%d-%m-%Y)")?;
            let due_date = date::parse_date(due_date_input.trim())?;

            todo_list::update_tasks(
                update_name,
                TaskUpdate {
                    name: new_name.clone(),
                    description: new_description.clone(),
                    due_date,
                },
                &mut tasks,
            )?;
            storage::save(path, &tasks)?;
            println!("Modifie {update_name} avec succes: {new_name} et {new_description}");
        }
        None => println!("commande non reconnue"),
    }

    Ok(())
}
