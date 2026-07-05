use clap::{Parser,Subcommand};

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
// struct Task {
//     id: u32,
//     title: String,
//     completed: bool,
// }

pub fn read_input() {
    let args = Cli::parse();
    match args.mode {
        Action::Affiche => {
            println!("Affichage de la liste des joueurs...");
        }
        Action::Ajoute { add_nom } => {
            println!("Ajout de : {:?}", add_nom);
        }
        Action::Supprimer { rm_nom } => {
            println!("Suppression de : {:?}", rm_nom);
        }
        _=>println!("autre")
    }
    
}
