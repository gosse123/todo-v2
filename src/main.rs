mod cli;
mod storage;
mod task;
use clap::{Parser};

use storage::*;
use cli::{Cli,Action};
use dialoguer::Input;
use task::Task;

const FILE_PATH:&str =  "file.json";

fn main()->std::io::Result<()>{

    let mut tasks = load(FILE_PATH)?;
    let args = Cli::parse();
    match args.mode {
        Action::Affiche{name_aff} => {
            if name_aff.is_empty(){
                print_summary(&tasks);
            }else {
                for name in name_aff {
                    print_detail_by_name(&tasks, &name);
                }
            }
        }
        Action::Ajoute=> {
           let name: String = Input::new()
                .with_prompt("Nom de la tâche")
                .interact_text()
                .unwrap();

            let description: String = Input::new()
                .with_prompt("Description")
                .interact_text()
                .unwrap();

            tasks.push(Task { name, description, completed: false });
                    save(&tasks, FILE_PATH)?;
        }
        Action::Supprimer { rm_nom } => {
            for name in rm_nom{
                remove_task(&mut tasks, &name);
            }
            save(&tasks, FILE_PATH)?;

        }
        _=>println!("commande non reconnu")
    }

    Ok(())
}
