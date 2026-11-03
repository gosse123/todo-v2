use crate::task::{Etat, Task};
use colored::Colorize;

pub fn print_summary(tasks: &[Task]) -> String {
    let mut summary = String::new();
    for (i, task) in tasks.iter().enumerate() {
        let number = i + 1;
        let (marker, name) = match task.status {
            Etat::Done => ("✓".green().to_string(), task.name.dimmed().to_string()),
            Etat::Inprogress => ("o".yellow().to_string(), task.name.dimmed().to_string()),
            Etat::Todo => ("☐".blue().to_string(), task.name.clone()),
        };
        if !summary.is_empty() {
            summary.push('\n');
        }
        summary.push_str(&format!("{number}. {marker} {name}"));
    }
    summary
}

pub fn print_detail_by_name(tasks: &[Task], name: &str) -> Option<String> {
    let task = tasks.iter().find(|t| t.name == name)?;

    let mut detail = String::new();
    detail.push_str(&format!("Tâche \"{}\":\n", task.name));
    detail.push_str(&format!("Description : {}\n", task.description));
    detail.push_str(&format!(
        "Date de creation de la tâche: {}\n",
        task.createat
    ));
    if let Some(updateat) = task.updateat {
        detail.push_str(&format!("Dernire mise a jour de la tâche: {updateat}\n"));
    }
    detail.push_str(&format!(
        "status : {}\n",
        match task.status {
            Etat::Todo => "non terminé",
            Etat::Inprogress => "en cours",
            Etat::Done => "terminé",
        }
    ));
    match task.due_date {
        Some(due_date) => detail.push_str(&format!("date d'échance: {due_date}")),
        None => detail.push_str("pas de date d'échance"),
    }
    Some(detail)
}

#[cfg(test)]
mod tests {
    use super::{print_detail_by_name, print_summary};
    use crate::task::{Etat, Task};

    fn tache(nom: &str) -> Task {
        Task::new(nom.to_string(), "Description".to_string()).unwrap()
    }

    #[test]
    fn le_resume_numerote_les_taches_depuis_1() {
        colored::control::set_override(false);
        let tasks = vec![tache("première"), tache("deuxième")];

        let summary = print_summary(&tasks);

        assert_eq!(summary, "1. ☐ première\n2. ☐ deuxième");
    }

    #[test]
    fn le_resume_marque_une_tache_terminée() {
        colored::control::set_override(false);
        let mut tasks = vec![tache("achevée")];
        tasks[0].status = Etat::Done;

        let summary = print_summary(&tasks);

        assert_eq!(summary, "1. ✓ achevée");
    }

    #[test]
    fn le_resume_d_une_liste_vide_est_vide() {
        colored::control::set_override(false);

        assert_eq!(print_summary(&[]), "");
    }

    #[test]
    fn le_detail_d_une_tache_connue_contient_son_nom_et_sa_description() {
        colored::control::set_override(false);
        let tasks = vec![tache("rust")];

        let detail = print_detail_by_name(&tasks, "rust").unwrap();

        assert!(detail.starts_with("Tâche \"rust\":"));
        assert!(detail.contains("Description : Description"));
    }

    #[test]
    fn le_detail_d_une_tache_inconnue_est_absent() {
        let tasks = vec![tache("rust")];

        assert!(print_detail_by_name(&tasks, "python").is_none());
    }
}
