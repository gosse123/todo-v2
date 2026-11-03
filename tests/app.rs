mod common;

use chrono::NaiveDate;
use common::FakePrompter;
use std::path::PathBuf;
use todo_gosse::app;
use todo_gosse::cli::{Action, Cli};
use todo_gosse::errors::TodoError;
use todo_gosse::storage;
use todo_gosse::task::{Etat, Task};

fn fichier_de_donnees() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("tasks.json");
    (dir, path)
}

fn tache(nom: &str) -> Task {
    Task::new(nom.to_string(), "Description".to_string()).unwrap()
}

#[test]
fn ajoute_une_tache_dans_le_fichier() {
    let (_dir, path) = fichier_de_donnees();
    let prompter = FakePrompter::new(&["  Acheter du pain  ", "  Au boulanger  "]);
    let cli = Cli {
        mode: Some(Action::Ajoute),
    };

    app::run(&cli, &path, &prompter).unwrap();

    let tasks = storage::load(&path).unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].name, "Acheter du pain");
    assert_eq!(tasks[0].description, "Au boulanger");
    assert_eq!(tasks[0].status, Etat::Todo);
}

#[test]
fn ajouter_une_tache_sans_nom_echoue_sans_ecrire_de_fichier() {
    let (_dir, path) = fichier_de_donnees();
    let prompter = FakePrompter::new(&["   ", "Description"]);
    let cli = Cli {
        mode: Some(Action::Ajoute),
    };

    let resultat = app::run(&cli, &path, &prompter);

    assert!(matches!(resultat, Err(TodoError::NomInvalide)));
    assert!(!path.exists());
}

#[test]
fn modifie_une_tache_et_enregistre_la_nouvelle_date() {
    let (_dir, path) = fichier_de_donnees();
    storage::save(&path, &[tache("Ancienne")]).unwrap();
    let prompter = FakePrompter::new(&["Nouvelle", "Nouvelle description", "01-02-2026"]);
    let cli = Cli {
        mode: Some(Action::Modifie {
            update_name: "Ancienne".to_string(),
        }),
    };

    app::run(&cli, &path, &prompter).unwrap();

    let tasks = storage::load(&path).unwrap();
    assert_eq!(tasks[0].name, "Nouvelle");
    assert_eq!(tasks[0].description, "Nouvelle description");
    assert_eq!(
        tasks[0].due_date,
        Some(NaiveDate::from_ymd_opt(2026, 2, 1).unwrap())
    );
    assert!(tasks[0].updateat.is_some());
}

#[test]
fn une_date_invalide_ne_modifie_aucune_tache() {
    let (_dir, path) = fichier_de_donnees();
    storage::save(&path, &[tache("Ancienne")]).unwrap();
    let prompter = FakePrompter::new(&["Nouvelle", "Nouvelle description", "2026-02-01"]);
    let cli = Cli {
        mode: Some(Action::Modifie {
            update_name: "Ancienne".to_string(),
        }),
    };

    let resultat = app::run(&cli, &path, &prompter);

    assert!(matches!(resultat, Err(TodoError::DateInvalide(_))));
    let tasks = storage::load(&path).unwrap();
    assert_eq!(tasks[0].name, "Ancienne");
    assert_eq!(tasks[0].description, "Description");
    assert!(tasks[0].updateat.is_none());
}

#[test]
fn supprimer_une_tache_enregistre_le_fichier() {
    let (_dir, path) = fichier_de_donnees();
    storage::save(&path, &[tache("a"), tache("b")]).unwrap();
    let cli = Cli {
        mode: Some(Action::Supprimer {
            rm_nom: vec!["a".to_string()],
        }),
    };

    app::run(&cli, &path, &FakePrompter::new(&[])).unwrap();

    let tasks = storage::load(&path).unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].name, "b");
}

#[test]
fn supprimer_une_tache_inconnue_continue_les_autres_suppressions() {
    let (_dir, path) = fichier_de_donnees();
    storage::save(&path, &[tache("a"), tache("b")]).unwrap();
    let cli = Cli {
        mode: Some(Action::Supprimer {
            rm_nom: vec!["inconnue".to_string(), "a".to_string()],
        }),
    };

    app::run(&cli, &path, &FakePrompter::new(&[])).unwrap();

    let tasks = storage::load(&path).unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].name, "b");
}

#[test]
fn faire_terminne_la_tache_a_l_index_1() {
    let (_dir, path) = fichier_de_donnees();
    storage::save(&path, &[tache("a"), tache("b")]).unwrap();
    let cli = Cli {
        mode: Some(Action::Faire { indices: vec![1] }),
    };

    app::run(&cli, &path, &FakePrompter::new(&[])).unwrap();

    let tasks = storage::load(&path).unwrap();
    assert_eq!(tasks[0].status, Etat::Done);
    assert_eq!(tasks[1].status, Etat::Todo);
}

#[test]
fn faire_avec_un_index_hors_bornes_echoue_sans_ecrire_de_fichier() {
    let (_dir, path) = fichier_de_donnees();
    storage::save(&path, &[tache("a")]).unwrap();
    let cli = Cli {
        mode: Some(Action::Faire { indices: vec![9] }),
    };

    let resultat = app::run(&cli, &path, &FakePrompter::new(&[]));

    assert!(matches!(resultat, Err(TodoError::IndexInvalide(9))));
    let tasks = storage::load(&path).unwrap();
    assert_eq!(tasks[0].status, Etat::Todo);
}

#[test]
fn affiche_ne_modifie_pas_le_fichier() {
    let (_dir, path) = fichier_de_donnees();
    storage::save(&path, &[tache("a")]).unwrap();
    let avant = std::fs::read(&path).unwrap();
    let cli = Cli {
        mode: Some(Action::Affiche { name_aff: vec![] }),
    };

    app::run(&cli, &path, &FakePrompter::new(&[])).unwrap();

    assert_eq!(std::fs::read(&path).unwrap(), avant);
}
