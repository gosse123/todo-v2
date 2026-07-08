use clap::{Parser,Subcommand};


#[derive(Debug, Parser)]
#[command(name = "gosse-todo", about = "Gestionnaire de tache")]
pub struct Cli{
    #[command(subcommand)]
    pub mode:Option<Action>
}

#[derive(Subcommand,Debug)]
pub enum Action {
    Affiche{
        #[arg(num_args = 0..)]
        name_aff:Vec<String>
    },
    Ajoute,
    Supprimer{
        #[arg(required = true, num_args = 1..)]
        rm_nom:Vec<String>
    },
    Faire{
        #[arg(required = true, num_args = 1..)]
        indices: Vec<usize>,
    }
    
}
