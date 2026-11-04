use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn repertoire_avec_une_tache(nom: &str) -> TempDir {
    let dir = tempfile::tempdir().unwrap();
    let data_dir = dir.path().join("todo-gosse");
    fs::create_dir_all(&data_dir).unwrap();
    fs::write(
        data_dir.join("tasks.json"),
        format!(
            r#"[{{"name":"{nom}","description":"Description","status":"Todo","createat":"2026-01-01T00:00:00Z","updateat":null,"due_date":null}}]"#
        ),
    )
    .unwrap();
    dir
}

fn todo_gosse(dir: &TempDir, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_todo-gosse"))
        .env("XDG_DATA_HOME", dir.path())
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn affiche_numerote_les_taches_depuis_1() {
    let dir = repertoire_avec_une_tache("rust");

    let sortie = todo_gosse(&dir, &["affiche"]);

    assert!(sortie.status.success());
    let stdout = String::from_utf8_lossy(&sortie.stdout);
    assert!(stdout.contains("1."));
    assert!(stdout.contains("rust"));
}

#[test]
fn affiche_d_une_tache_inconnue_signale_l_erreur_sans_echouer() {
    let dir = repertoire_avec_une_tache("rust");

    let sortie = todo_gosse(&dir, &["affiche", "python"]);

    assert!(sortie.status.success());
    let stderr = String::from_utf8_lossy(&sortie.stderr);
    assert!(stderr.contains("aucune tâche nommée"));
}

#[test]
fn faire_avec_un_index_hors_bornes_renvoie_un_code_erreur() {
    let dir = repertoire_avec_une_tache("rust");

    let sortie = todo_gosse(&dir, &["faire", "9"]);

    assert_eq!(sortie.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&sortie.stderr);
    assert!(stderr.contains("aucune tâche à la position"));
}

#[test]
fn supprime_la_tache_nommee_dans_le_fichier() {
    let dir = repertoire_avec_une_tache("rust");

    let sortie = todo_gosse(&dir, &["supprimer", "rust"]);

    assert!(sortie.status.success());
    let contenu = fs::read_to_string(dir.path().join("todo-gosse/tasks.json")).unwrap();
    assert_eq!(contenu.trim(), "[]");
}

#[test]
fn une_commande_inconnue_renvoie_un_code_erreur() {
    let dir = repertoire_avec_une_tache("rust");

    let sortie = todo_gosse(&dir, &["commande-inconnue"]);

    assert_eq!(sortie.status.code(), Some(2));
}
