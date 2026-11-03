use core::fmt;

#[derive(Debug)]
pub enum TodoError {
    CreateDirError(std::io::Error),
    SereliasisationError(serde_json::Error),
    AppDireError,
    InvalidInputError(dialoguer::Error),
    TaskNotFound(String),
    NomInvalide,
    DescriptionInvalide,
    DateInvalide(String),
    IndexInvalide(usize),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::CreateDirError(e) => {
                write!(f, "erreur lors de la creation du dossier: {}", e)
            }
            TodoError::SereliasisationError(e) => {
                write!(f, "erreur d'ecriture/lectures des donnes: {}", e)
            }
            TodoError::AppDireError => write!(
                f,
                "erreur lors de la mise en place du chemin de dossier de l'application"
            ),
            TodoError::InvalidInputError(e) => write!(f, "entre invalide : {}", e),
            TodoError::TaskNotFound(e) => write!(f, "tache introuvable  : {}", e),
            TodoError::NomInvalide => write!(f, "le nom de la tâche ne peut pas être vide"),
            TodoError::DescriptionInvalide => {
                write!(f, "la description de la tâche ne peut pas être vide")
            }
            TodoError::DateInvalide(e) => write!(
                f,
                "date d'échéance invalide : {e} (format attendu : %d-%m-%Y)"
            ),
            TodoError::IndexInvalide(i) => write!(f, "aucune tâche à la position {i}"),
        }
    }
}

impl From<std::io::Error> for TodoError {
    fn from(value: std::io::Error) -> Self {
        TodoError::CreateDirError(value)
    }
}

impl From<serde_json::Error> for TodoError {
    fn from(value: serde_json::Error) -> Self {
        TodoError::SereliasisationError(value)
    }
}

impl From<dialoguer::Error> for TodoError {
    fn from(value: dialoguer::Error) -> Self {
        TodoError::InvalidInputError(value)
    }
}
