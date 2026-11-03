use clap::Parser;
use std::process::ExitCode;
use todo_gosse::app;
use todo_gosse::cli::Cli;
use todo_gosse::prompt::DialoguerPrompter;
use todo_gosse::storage;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let prompter = DialoguerPrompter;

    match storage::data_file().and_then(|path| app::run(&cli, &path, &prompter)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Erreur : {e}");
            ExitCode::FAILURE
        }
    }
}
