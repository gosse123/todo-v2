use crate::errors::TodoError;
use crate::task::Task;
use directories::ProjectDirs;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn data_file() -> Result<PathBuf, TodoError> {
    let proj_dirs =
        ProjectDirs::from("com", "nahounou", "todo-gosse").ok_or(TodoError::AppDireError)?;
    let data_dir = proj_dirs.data_dir();

    fs::create_dir_all(data_dir)?;
    Ok(data_dir.join("tasks.json"))
}

pub fn load(path: &Path) -> Result<Vec<Task>, TodoError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(path)?;
    if data.trim().is_empty() {
        return Ok(Vec::new());
    }
    Ok(serde_json::from_str(&data)?)
}

pub fn save(path: &Path, tasks: &[Task]) -> Result<(), TodoError> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty());
    if let Some(parent) = parent {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(tasks)?;
    let mut file = fs::File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{load, save};
    use crate::errors::TodoError;
    use crate::task::Task;

    fn tache(nom: &str) -> Task {
        Task::new(nom.to_string(), "Description".to_string()).unwrap()
    }

    #[test]
    fn un_fichier_inexistant_est_une_liste_vide() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tasks.json");

        assert!(load(&path).unwrap().is_empty());
    }

    #[test]
    fn un_fichier_vide_est_une_liste_vide() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tasks.json");
        std::fs::write(&path, "   \n").unwrap();

        assert!(load(&path).unwrap().is_empty());
    }

    #[test]
    fn sauvegarde_puis_charge_les_memes_taches() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tasks.json");
        let tasks = vec![tache("a"), tache("b")];

        save(&path, &tasks).unwrap();

        assert_eq!(load(&path).unwrap(), tasks);
    }

    #[test]
    fn un_fichier_invalide_renvoie_une_erreur_de_serialisation() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tasks.json");
        std::fs::write(&path, "{ pas du json }").unwrap();

        assert!(matches!(
            load(&path),
            Err(TodoError::SereliasisationError(_))
        ));
    }
}
