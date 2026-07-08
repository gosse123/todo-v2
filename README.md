# Todo CLI App

Une application en ligne de commande pour gérer une liste de tâches, écrite en Rust.

## Fonctionnalités

- Ajout de tâches de façon interactive (nom + description)
- Affichage résumé (numéro, statut, nom) ou détaillé (par nom de tâche)
- Marquage de tâches comme terminées
- Suppression de tâches par nom
- Persistance locale au format JSON

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (édition 2021 ou plus récente)
- Cargo (installé avec Rust)

## Installation

Cloner le dépôt puis compiler le projet :

```bash
git clone git@github.com:gosse123/todo-v2.git
cd todo-cli-app
cargo build --release
```

Le binaire compilé se trouve ensuite dans `target/release/todo-cli-app`.

Pour l'utiliser comme commande globale sur ta machine :

```bash
cargo install --path .
```

## Utilisation

### Ajouter une tâche

```bash
todo-cli-app ajoute
```

Le programme demande interactivement le nom et la description de la tâche.

### Afficher les tâches

Sans argument, affiche un résumé (numéro, case, nom) :

```bash
todo-cli-app affiche
```

```
1. ✓ rust
2. ☐ python
```

Avec un ou plusieurs noms, affiche le détail complet de chaque tâche :

```bash
todo-cli-app affiche rust python
```

```
Tâche "rust":
  Description : apprendre les lifetimes
  Terminée    : oui

Tâche "python":
  Description : script FastAPI
  Terminée    : non
```

Si un nom ne correspond à aucune tâche, une erreur est affichée pour ce nom précis, sans bloquer l'affichage des autres :

```
Erreur : aucune tâche nommée "inexistant" n'existe.
```

### Marquer une tâche comme terminée

```bash
todo-cli-app done <numero>
```

### Supprimer une tâche

```bash
todo-cli-app supprimer <nom> [<nom> ...]
```

Plusieurs noms peuvent être passés en une seule commande. Comme pour `affiche`, un nom invalide produit une erreur ciblée sans empêcher la suppression des autres tâches valides.

## Stockage des données

Les tâches sont enregistrées dans un fichier JSON local. Le dossier parent est créé automatiquement si nécessaire, et un fichier vide est traité comme une liste sans tâches (aucune erreur au premier lancement).

## Structure du projet

```
todo-cli-app/
├── src/
│   ├── main.rs      # Point d'entrée, routage des commandes
│   ├── cli.rs        # Définition des arguments et sous-commandes (clap)
│   ├── task.rs       # Définition de la structure Task
│   └── storage.rs    # Chargement / sauvegarde JSON, affichage, suppression
├── Cargo.toml
└── README.md
```

## Dépendances principales

| Crate         | Rôle                                        |
|---------------|---------------------------------------------|
| `clap`        | Parsing des arguments et sous-commandes     |
| `dialoguer`   | Saisie interactive (ajout de tâche)         |
| `serde`       | Sérialisation / désérialisation des tâches  |
| `serde_json`  | Format de stockage JSON                     |
| `colored`     | Mise en couleur de l'affichage terminal     |

## Pistes d'amélioration

- Gestion des doublons de noms (actuellement, une opération par nom cible la première correspondance trouvée)
- Tests d'intégration automatisés
- Historique / horodatage des tâches
- Édition d'une tâche existante (modifier nom ou description sans supprimer/recréer)

## Licence

À définir selon les besoins du projet.