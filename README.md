# todo_cli

A small, dependency-light TODO list manager for the terminal, written in Rust as a learning project. Each invocation runs a single command against a JSON file on disk, so the task list persists across runs.

## Features

- Add, list, complete, and remove tasks
- Tasks are persisted to a local `tasks.json` file (pretty-printed, human-readable)
- Task IDs are reused after deletion (based on the current highest ID + 1), so the list stays compact
- Corrupted or unreadable data files are detected and reported instead of being silently overwritten
- Clear, `stderr`-based error messages with correct process exit codes, so the tool composes well in shell scripts

## Installation

Requires a recent Rust toolchain (install via [rustup](https://rustup.rs/) if you don't have one).

```bash
git clone https://codeberg.org/voidptrx/todo_rs.git
cd todo_rs
cargo build --release
```

The compiled binary will be at `target/release/todo_cli`. Copy it somewhere on your `$PATH` if you want to run it as `todo` from anywhere:

```bash
cp target/release/todo_cli ~/.local/bin/todo
```

## Usage

```bash
todo add <header> <summary>   # add a new task
todo list                     # list all tasks
todo complete <id>            # mark a task as done
todo remove <id>              # delete a task
```

### Example

```bash
$ todo add "Market" "Buy milk and eggs"
Task added.

$ todo add "Dotfiles" "Configure runit services"
Task added.

$ todo list
[ ] 1: Market - Buy milk and eggs
[ ] 2: Dotfiles - Configure runit services

$ todo complete 1
Task 1 marked as done.

$ todo list
[x] 1: Market - Buy milk and eggs
[ ] 2: Dotfiles - Configure runit services

$ todo remove 2
Task 2 was removed.
```

## Data storage

Tasks are stored as JSON in `tasks.json`, created in the directory the command is run from:

```json
{
  "tasks": [
    {
      "id": 1,
      "header": "Market",
      "summary": "Buy milk and eggs",
      "is_done": true
    }
  ]
}
```

> **Note:** the file path is currently relative to the current working directory rather than a fixed location such as `$XDG_DATA_HOME`. This is a known limitation, see [Roadmap](#roadmap).

## Project structure

```
src/
├── main.rs     # argument parsing and command dispatch
├── models.rs   # Task / TodoData data model and in-memory operations
└── storage.rs  # loading from and saving to disk
```

The data model and storage layer have no dependency on the CLI layer, which keeps the door open for alternative front-ends (e.g. a TUI) built on the same core.

## Roadmap

- [ ] Store the data file in a proper XDG-compliant location instead of the current working directory
- [ ] Atomic writes (write to a temp file, then rename) to avoid a corrupted file on interruption
- [ ] Optional interactive TUI mode (built with [ratatui](https://github.com/ratatui/ratatui)) alongside the current one-shot CLI commands

## License

MIT (or update to your preference).
