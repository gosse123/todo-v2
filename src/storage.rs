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
