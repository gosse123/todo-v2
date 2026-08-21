mod cli;
mod errors;
mod storage;
mod task;

use crate::errors::TodoError;
use clap::Parser;
use cli::{Action, Cli};
use dialoguer::Input;
use storage::*;
use task::Task;

fn main() -> Result<(), TodoError> {
    let mut tasks = load()?;
    let args = Cli::parse();
    match args.mode {
        Some(Action::Affiche { name_aff }) => {
            if name_aff.is_empty() {
                print_summary(&tasks);
            } else {
                for name in name_aff {
                    print_detail_by_name(&tasks, &name);
                }
            }
        }
        Some(Action::Ajoute) => {
            let name: String = Input::new()
                .with_prompt("Nom de la tâche")
                .interact_text()?;
            let name = name.trim().to_string();

            let description: String = Input::new().with_prompt("Description").interact_text()?;
            let description = description.trim().to_string();

            tasks.push(Task::new(name, description));
            save(&tasks)?;
        }
        Some(Action::Supprimer { rm_nom }) => {
            for name in rm_nom {
                remove_task(&mut tasks, &name);
            }
            save(&tasks)?;
        }
        Some(Action::Faire { indices }) => {
            for num in indices {
                as_done(&mut tasks, num);
            }
            save(&tasks)?;
        }
        Some(Action::Modifie { indice }) => {
            println!("Modifie avec succes")
        }
        _ => println!("commande not reconnu"),
    }

    Ok(())
}
