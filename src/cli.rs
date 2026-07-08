use clap::{Parser,Subcommand};


#[derive(Debug, Parser)]
#[command(name = "gosse-todo", about = "Gestionnaire de tache")]
pub struct Cli{
    #[command(subcommand)]
    pub mode:Action
}

#[derive(Subcommand,Debug)]
pub enum Action {
    Affiche{
        num_of_task:Vec<u32>
    },
    Ajoute,
    Supprimer{
        rm_nom:Vec<String>
    },
    Faire{
        done_nom:Vec<String>
    }
    
}
