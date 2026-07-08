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
        Some(Action::Affiche{name_aff}) => {
            if name_aff.is_empty(){
                print_summary(&tasks);
            }else {
                for name in name_aff {

                    print_detail_by_name(&tasks, &name);
                }
            }
        }
        Some(Action::Ajoute)=> {
           let name: String = Input::new()
                .with_prompt("Nom de la tâche")
                .interact_text()
                .unwrap();
            let name = name.trim().to_string();

            let description: String = Input::new()
                .with_prompt("Description")
                .interact_text()
                .unwrap();
            let description = description.trim().to_string();

            tasks.push(Task { name, description, completed: false });
            save(&tasks, FILE_PATH)?;
        }
        Some(Action::Supprimer { rm_nom }) => {
            for name in rm_nom{
                remove_task(&mut tasks, &name);
            }
            save(&tasks, FILE_PATH)?;

        }
        Some(Action::Faire { indices })=>{
            for num in indices{
                as_done(&mut tasks, num);
            }
            save(&tasks, FILE_PATH)?;
        }
        _=>println!("commande not reconnu")
    }

    Ok(())
}
