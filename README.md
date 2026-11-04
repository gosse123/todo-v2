# Todo CLI App

> Une application de gestion de tâches en ligne de commande, développée en **Rust**, permettant de créer, consulter, terminer et supprimer des tâches avec une persistance locale au format JSON.

![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-green)
![CLI](https://img.shields.io/badge/interface-CLI-blue)

---

## Table des matières

- [Fonctionnalités](#fonctionnalités)
- [Installation](#installation)
- [Utilisation](#utilisation)
- [Structure du projet](#structure-du-projet)
- [Dépendances](#dépendances-principales)
- [Stockage des données](#stockage-des-données)
- [Améliorations futures](#améliorations-futures)
- [Licence](#licence)

---

# Fonctionnalités

- Ajouter une tâche de manière interactive
- Afficher toutes les tâches
- Afficher les détails d'une ou plusieurs tâches
- Marquer une tâche comme terminée
- Supprimer une ou plusieurs tâches
- Sauvegarde automatique au format JSON
- Affichage coloré dans le terminal

---

# Installation

## Depuis crates.io

*(Disponible après publication)*

```bash
cargo install todo-gosse
```

---

## Depuis GitHub

```bash
git clone https://github.com/gosse123/todo-v2.git
cd todo-v2
cargo build --release
```

Le binaire compilé est disponible dans :

```text
target/release/todo-gosse
```

Pour installer le projet localement :

```bash
cargo install --path .
```

---

# Utilisation

## Ajouter une tâche

```bash
todo-gosse ajoute
```

Le programme demandera :

- le nom
- la description

---

## Afficher toutes les tâches

```bash
todo-gosse affiche
```

Exemple :

```text
1. ✓ Apprendre Rust
2. ☐ Faire les exercices
3. ☐ Publier le projet
```

---

## Afficher une ou plusieurs tâches

```bash
todo-gosse affiche Rust Projet
```

Exemple :

```text
Tâche : Rust

Description :
Apprendre les lifetimes

Terminée :
Oui
```

Si une tâche n'existe pas :

```text
Erreur : aucune tâche nommée "Python" n'existe.
```

---

## Marquer une tâche comme terminée

```bash
todo-gosse faire 2
```

Le numéro correspond à celui affiché par `affiche` (la numérotation
commence à 1). Un numéro qui n'existe pas est refusé avec un message
d'erreur et le fichier n'est pas modifié.

---

## Supprimer une ou plusieurs tâches

```bash
todo-gosse supprimer Rust Projet
```

Seule la première tâche portant ce nom est supprimée. Un nom inconnu est
signalé sur la sortie d'erreur, les autres suppressions sont quand même
appliquées.

---

# Stockage des données

Toutes les tâches sont enregistrées automatiquement dans un fichier JSON.

Au premier lancement :

- le dossier est créé automatiquement ;
- le fichier JSON est créé si nécessaire ;
- un fichier vide est interprété comme une liste vide.

Aucune configuration supplémentaire n'est nécessaire.

Le fichier est stocké dans le dossier de données de l'application :

```text
~/.local/share/todo-gosse/tasks.json
```

La variable d'environnement `XDG_DATA_HOME` est respectée sur Linux.

---

## Modifier une tâche

```bash
todo-gosse modifie Rust
```

Le programme demande le nouveau nom, la nouvelle description et la date
d'échéance, au format `%d-%m-%Y` (par exemple `01-02-2026`). Si la date
est invalide, la tâche n'est pas modifiée du tout.

---

# Structure du projet

```text
todo-gosse
│
├── src
│   ├── lib.rs        racine de la bibliothèque
│   ├── main.rs       binaire : parse les arguments et lance l'application
│   ├── app.rs        répartition des commandes
│   ├── cli.rs        définition des arguments (clap)
│   ├── prompt.rs     saisie interactive (trait Prompter)
│   ├── task.rs       modèles Task et Etat
│   ├── todo_list.rs  opérations sur la liste de tâches
│   ├── storage.rs    lecture et écriture du fichier JSON
│   ├── display.rs    mise en forme des tâches
│   ├── date.rs       lecture d'une date d'échéance
│   └── errors.rs     type d'erreur TodoError
│
├── tests
│   ├── app.rs        tests des commandes de bout en bout
│   ├── cli.rs        tests du binaire
│   └── common        prompteur de test
│
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENCE
└── .gitignore
```

Chaque module a une seule responsabilité : `storage.rs` ne fait que lire
et écrire le fichier, `display.rs` que mettre en forme, `todo_list.rs`
que modifier la liste de tâches. Les fonctions d'affichage renvoient
un texte et les fonctions de stockage reçoivent le chemin du fichier :
tout est donc testable sans terminal ni fichier de données réel.

---

# Dépendances principales

| Crate | Description |
|--------|-------------|
| clap | Gestion des arguments de la ligne de commande |
| dialoguer | Saisie interactive |
| serde | Sérialisation des données |
| serde_json | Stockage JSON |
| colored | Couleurs dans le terminal |
| directories | Emplacement du fichier de données |
| chrono | Dates de création, de mise à jour et d'échéance |

`tempfile` est ajouté en dépendance de développement, pour les tests.

---

## Tests

```bash
cargo test
```

Les tests unitaires vivent dans chaque module (`src/*.rs`), les tests
de bout en bout des commandes dans `tests/app.rs` et les tests du
binaire dans `tests/cli.rs`.

---

# Améliorations futures

- [x] Modifier une tâche existante
- [x] Ajouter une date d'échéance
- [ ] Ajouter des priorités
- [ ] Ajouter des catégories
- [ ] Recherche de tâches
- [x] Tests unitaires
- [ ] Synchronisation cloud
- [ ] Export CSV / Markdown
- [ ] Notifications

---

# Pourquoi ce projet ?

Ce projet a été développé afin de mettre en pratique plusieurs concepts fondamentaux de Rust :

- Ownership
- Borrowing
- Structures
- Enums
- Gestion des erreurs
- Sérialisation JSON
- Lecture et écriture de fichiers
- Organisation d'un projet Rust
- Développement d'une application CLI avec Clap

---

# Licence

Distribué sous licence **MIT**.

Voir le fichier `LICENCE` pour plus d'informations.
