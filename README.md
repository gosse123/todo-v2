# Todo CLI App 
une application de ligne de commande pour gérer une liste de tâches.

# fonctionnalités

- ajouter une tâche.
- marquer une tâche comme terminée.
- supprimer une tâche.
- sauvegarder les tâches dans un fichier.

## structure d'une tâche
```
struct Task {
    id: u32,
    title: String,
    desciption:String
    completed: bool,

}
```
