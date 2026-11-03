use crate::errors::TodoError;
use crate::task::{Etat, Task, TaskUpdate};
use chrono::Utc;

pub fn remove_task(tasks: &mut Vec<Task>, name: &str) -> Result<Task, TodoError> {
    let index = find_index(tasks, name).ok_or_else(|| TodoError::TaskNotFound(name.to_string()))?;
    Ok(tasks.remove(index))
}

pub fn as_done(tasks: &mut [Task], num: usize) -> Result<(), TodoError> {
    let index = num.checked_sub(1).ok_or(TodoError::IndexInvalide(num))?;
    let task = tasks.get_mut(index).ok_or(TodoError::IndexInvalide(num))?;
    task.status = Etat::Done;
    Ok(())
}

pub fn update_tasks(
    current_name: &str,
    update: TaskUpdate,
    tasks: &mut [Task],
) -> Result<(), TodoError> {
    if update.name.trim().is_empty() {
        return Err(TodoError::NomInvalide);
    }
    if update.description.trim().is_empty() {
        return Err(TodoError::DescriptionInvalide);
    }
    let index = find_index(tasks, current_name)
        .ok_or_else(|| TodoError::TaskNotFound(current_name.to_string()))?;
    let task = &mut tasks[index];
    task.name = update.name;
    task.description = update.description;
    task.due_date = Some(update.due_date);
    task.updateat = Some(Utc::now());
    Ok(())
}

fn find_index(tasks: &[Task], name: &str) -> Option<usize> {
    tasks.iter().position(|task| task.name == name)
}

#[cfg(test)]
mod tests {
    use super::{as_done, remove_task, update_tasks};
    use crate::errors::TodoError;
    use crate::task::{Etat, Task, TaskUpdate};
    use chrono::NaiveDate;

    fn tache(nom: &str) -> Task {
        Task::new(nom.to_string(), "Description".to_string()).unwrap()
    }

    fn mise_a_jour(nom: &str, description: &str) -> TaskUpdate {
        TaskUpdate {
            name: nom.to_string(),
            description: description.to_string(),
            due_date: NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
        }
    }

    #[test]
    fn supprime_la_premiere_tache_du_meme_nom() {
        let mut tasks = vec![tache("a"), tache("a"), tache("b")];

        let removed = remove_task(&mut tasks, "a").unwrap();

        assert_eq!(removed.name, "a");
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].name, "a");
        assert_eq!(tasks[1].name, "b");
    }

    #[test]
    fn supprimer_une_tache_inconnue_renvoie_une_erreur_sans_toucher_la_liste() {
        let mut tasks = vec![tache("a")];

        let resultat = remove_task(&mut tasks, "z");

        assert!(matches!(resultat, Err(TodoError::TaskNotFound(_))));
        assert_eq!(tasks.len(), 1);
    }

    #[test]
    fn faire_terminne_la_tache_a_l_index_1() {
        let mut tasks = vec![tache("a"), tache("b")];

        as_done(&mut tasks, 1).unwrap();

        assert_eq!(tasks[0].status, Etat::Done);
        assert_eq!(tasks[1].status, Etat::Todo);
    }

    #[test]
    fn faire_avec_un_index_hors_bornes_renvoie_une_erreur_sans_changer_le_statut() {
        let mut tasks = vec![tache("a")];

        assert!(matches!(
            as_done(&mut tasks, 9),
            Err(TodoError::IndexInvalide(9))
        ));
        assert!(matches!(
            as_done(&mut tasks, 0),
            Err(TodoError::IndexInvalide(0))
        ));
        assert_eq!(tasks[0].status, Etat::Todo);
    }

    #[test]
    fn modifie_la_premiere_tache_du_meme_nom_seulement() {
        let mut tasks = vec![tache("a"), tache("a")];

        update_tasks("a", mise_a_jour("b", "Nouvelle"), &mut tasks).unwrap();

        assert_eq!(tasks[0].name, "b");
        assert_eq!(tasks[0].description, "Nouvelle");
        assert_eq!(
            tasks[0].due_date,
            Some(NaiveDate::from_ymd_opt(2026, 2, 1).unwrap())
        );
        assert!(tasks[0].updateat.is_some());
        assert_eq!(tasks[1].name, "a");
    }

    #[test]
    fn modifier_une_tache_inconnue_renvoie_une_erreur_sans_toucher_la_liste() {
        let mut tasks = vec![tache("a")];

        let resultat = update_tasks("z", mise_a_jour("b", "Nouvelle"), &mut tasks);

        assert!(matches!(resultat, Err(TodoError::TaskNotFound(_))));
        assert_eq!(tasks[0].name, "a");
    }

    #[test]
    fn modifier_avec_un_nom_vide_renvoie_une_erreur_sans_toucher_la_liste() {
        let mut tasks = vec![tache("a")];

        let resultat = update_tasks("a", mise_a_jour("   ", "Nouvelle"), &mut tasks);

        assert!(matches!(resultat, Err(TodoError::NomInvalide)));
        assert_eq!(tasks[0].name, "a");
    }

    #[test]
    fn modifier_avec_une_description_vide_renvoie_une_erreur_sans_toucher_la_liste() {
        let mut tasks = vec![tache("a")];

        let resultat = update_tasks("a", mise_a_jour("b", "   "), &mut tasks);

        assert!(matches!(resultat, Err(TodoError::DescriptionInvalide)));
        assert_eq!(tasks[0].name, "a");
        assert_eq!(tasks[0].description, "Description");
    }
}
