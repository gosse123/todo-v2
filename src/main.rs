use clap::Parser;
use todo_gosse::app;
use todo_gosse::cli::Cli;
use todo_gosse::errors::TodoError;
use todo_gosse::prompt::DialoguerPrompter;
use todo_gosse::storage;

fn main() -> Result<(), TodoError> {
    let cli = Cli::parse();
    let prompter = DialoguerPrompter;
    let path = storage::data_file()?;

    app::run(&cli, &path, &prompter)
}
