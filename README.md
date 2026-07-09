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

---

## Supprimer une ou plusieurs tâches

```bash
todo-gosse supprimer Rust Projet
```

---

# Stockage des données

Toutes les tâches sont enregistrées automatiquement dans un fichier JSON.

Au premier lancement :

- le dossier est créé automatiquement ;
- le fichier JSON est créé si nécessaire ;
- un fichier vide est interprété comme une liste vide.

Aucune configuration supplémentaire n'est nécessaire.

---

# Structure du projet

```text
todo-gosse
│
├── src
│   ├── main.rs
│   ├── cli.rs
│   ├── task.rs
│   └── storage.rs
│
├── Cargo.toml
├── README.md
├── LICENSE
└── .gitignore
```

---

# Dépendances principales

| Crate | Description |
|--------|-------------|
| clap | Gestion des arguments de la ligne de commande |
| dialoguer | Saisie interactive |
| serde | Sérialisation des données |
| serde_json | Stockage JSON |
| colored | Couleurs dans le terminal |

---

# Améliorations futures

- [ ] Modifier une tâche existante
- [ ] Ajouter une date d'échéance
- [ ] Ajouter des priorités
- [ ] Ajouter des catégories
- [ ] Recherche de tâches
- [ ] Tests unitaires
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

Voir le fichier `LICENSE` pour plus d'informations.