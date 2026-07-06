use std::{fs, io::Write};
use clap::{Parser,Subcommand};
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "gosse-todo", about = "Gestionnaire de tache")]
struct Cli{
    #[command(subcommand)]
    mode:Action
}

#[derive(Subcommand,Debug)]
enum Action {
    Affiche,
    Ajoute{
        add_nom:Vec<String>
    },
    Supprimer{
        rm_nom:Vec<String>
    },
    Faire{
        done_nom:Vec<String>
    }
    
}


pub fn read_input(path:&str)->std::io::Result<()>{
    let args = Cli::parse();
    match args.mode {
        Action::Affiche => {
            let data = load(path)?;
            for i in data{
                println!("{i}");
            }
        }
        Action::Ajoute { add_nom } => {
           save(&add_nom, path)?;
        }
        Action::Supprimer { rm_nom } => {
            println!("Suppression de : {:?}", rm_nom);
        }
        _=>println!("autre")
    }

    Ok(())
    
}

fn save(item:&Vec<String>,path:&str)-> std::io::Result<()>{
    let chemin = Path::new(path);

    if let Some(dossier_parent) = chemin.parent(){
        fs::create_dir_all(dossier_parent)?;
    }
    let json =serde_json::to_string_pretty(item)?;
    let mut file = fs::File::create(path)?;
    file.write(json.as_bytes())?;
    Ok(())
}

fn load(path: &str) -> std::io::Result<Vec<String>> {
    let data = fs::read_to_string(path)?;
    let items: Vec<String> = serde_json::from_str(&data).unwrap();
    Ok(items)
}
