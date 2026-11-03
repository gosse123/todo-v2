# AGENTS.md

Single-crate Rust CLI (`todo-gosse`, edition 2024 → needs Rust ≥ 1.85). One library target (`src/lib.rs`) plus one thin binary (`src/main.rs`). No workspace, no CI, no `rust-toolchain`/clippy config files.

## Build & verify

- `cargo build`, `cargo run -- <subcommand>`; the binary is named `todo-gosse`.
- Gate order: `cargo check` → `cargo clippy --all-targets` → `cargo test` → `cargo fmt --check`. All four are clean on this branch — keep them that way.
- `cargo test` runs 36 tests. There is no test framework: plain `#[test]` plus `tempfile`, the only dev-dependency. Unit tests live in each `src/*.rs` (`#[cfg(test)] mod tests`), command-level tests in `tests/app.rs`, binary-level tests in `tests/cli.rs`.
- `tests/common/mod.rs` holds `FakePrompter` (replaces the interactive prompts); `tests/cli.rs` launches the built binary through `env!("CARGO_BIN_EXE_todo-gosse")` with `XDG_DATA_HOME` pointed at a temp dir. Reuse both instead of inventing new plumbing.
- Tests never touch the real user data: storage functions take the path as a parameter, and the default path lives only in `storage::data_file()`.
- The crate is rustfmt-formatted now; `cargo fmt` only rewrites what you changed.

## Layout — one responsibility per file

- `src/lib.rs` — module root.
- `src/main.rs` — binary only: parse the CLI, resolve `storage::data_file()`, call `app::run`, print `TodoError`'s `Display` and exit 1. No business logic.
- `src/app.rs` — command dispatch: `run(cli: &Cli, path: &Path, prompter: &dyn Prompter)`.
- `src/cli.rs` — clap definitions. French variants (`Affiche`, `Ajoute`, `Supprimer`, `Faire`, `Modifie`) become kebab-case subcommands.
- `src/prompt.rs` — the `Prompter` trait plus `DialoguerPrompter`. The trait is the **only** seam over `dialoguer`; trimming happens in `app.rs`, not in the implementation, so a fake behaves exactly like the real prompter.
- `src/task.rs` — `Task`, `Etat`, `TaskUpdate`. `Task::new` returns `Result` (an empty name is invalid); `Etat` unit variants serialize as plain strings.
- `src/todo_list.rs` — `remove_task`, `as_done`, `update_tasks`. The shared `find_index` helper resolves a name to the **first** match only.
- `src/storage.rs` — `data_file`, `load(path)`, `save(path, tasks)`. A missing or empty file is an empty list.
- `src/display.rs` — `print_summary` and `print_detail_by_name` **return a `String`** instead of printing; `app.rs` does the printing. `print_detail_by_name` returns `None` for an unknown name.
- `src/date.rs` — `parse_date`, `%d-%m-%Y` only.
- `src/errors.rs` — `TodoError`. `main` prints its `Display`, so the French text here is what the user actually sees.

## Data file (real user data — change the schema carefully)

- Path: `ProjectDirs::from("com", "nahounou", "todo-gosse")` → on Linux `~/.local/share/todo-gosse/tasks.json` (honours `XDG_DATA_HOME`).
- On-disk shape: `name`, `description`, `status` (`"Todo"` | `"Inprogress"` | `"Done"`), `createat`, `updateat`, `due_date`. Timestamps are RFC3339 UTC; `due_date` is `YYYY-MM-DD`.
- `Etat` has no `rename_all`: unit variants serialize as `"Todo"`/`"Inprogress"`/`"Done"`. Do not change that without migrating the file.

## CLI contracts

- `affiche [names...]` — with no names prints the summary numbered **from 1**; with names prints details (exact, case-sensitive name).
- `faire <index...>` — takes those same **1-based** indices; an out-of-range index is `TodoError::IndexInvalide` and nothing is written.
- `supprimer <name...>` — removes the **first** task with that name; an unknown name is reported on stderr while the other deletions still happen (exit 0).
- `modifie <name>` — first match only; the date is parsed **before** any write, so an invalid date changes nothing (`TodoError::DateInvalide`).
- `ajoute` and `modifie` are interactive (`dialoguer`) and need a TTY, so they are not scriptable in CI.
- `ajoute` accepts an empty description but rejects an empty name; `modifie` rejects both an empty name and an empty description.

## Conventions

- User-facing text (prompts, errors, README) is French; keep new strings French. Identifiers and CLI verbs are French too (`Etat`, `remove_task`, `Faire`).
- `Cargo.lock` is committed — keep it in sync when dependencies change.
- The license file is `LICENCE` (British spelling), not `LICENSE`.
