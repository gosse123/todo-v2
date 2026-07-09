use core::fmt;

#[derive(Debug)]
pub enum TodoError{
    CreateDirError(std::io::Error),
    SereliasisationError(serde_json::Error),
    AppDireError,
    InvalidInputError(dialoguer::Error)

}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoError::CreateDirError(e)=>write!(f,"erreur lors de la creation du dossier: {}",e),
            TodoError::SereliasisationError(e)=>write!(f,"erreur d'ecriture/lectures des donnes: {}",e),
            TodoError::AppDireError=>write!(f,"erreur lors de la mise en place du chemin de dossier de l'application"),
            TodoError::InvalidInputError(e)=>write!(f,"entre invalide : {}",e)
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