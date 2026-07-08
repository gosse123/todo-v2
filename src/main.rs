mod cli;
mod storage;
mod task;
use clap::{Parser};

use storage::*;
use cli::{Cli,Action};
use dialoguer::Input;
use task::Task;

const FILE_PATH:&str =  "/home/gosse/Documents/todo-cli-app/data";

fn main()->std::io::Result<()>{
let args = Cli::parse();
    match args.mode {
        Action::Affiche{num_of_task} => {
            if num_of_task.is_empty(){
                print_list(&Task);
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

            Task.push(Task { name, description, completed: false });
                    save(&data, FILE_PATH)?;
        }
        Action::Supprimer { rm_nom } => {
            println!("Suppression de : {:#?}", rm_nom);
            let mut data = load(FILE_PATH)?;
            for i in rm_nom {
                if let Some(v) = data.iter().position(move |x| *x == i) {
                    data.remove(v);
                }
            }
            save(&data, FILE_PATH)?;

        }
        _=>println!("commande non reconnu")
    }

    Ok(())
}
