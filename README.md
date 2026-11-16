# rust-todo — a tiny command-line todo list in Rust

`rust-todo` is a complete, beginner-friendly first Rust project. It is a
command-line tool that keeps a simple todo list: you add tasks, list them,
tick them off, and delete them — all from the terminal.

It was written to be **read as well as run**:

- **Zero dependencies** — it uses only the Rust standard library, so
  `cargo build` works offline and there is nothing mysterious to install.
- **One file** — the whole program is `src/main.rs` (about 300 lines),
  organised into clearly labelled sections with beginner comments throughout.
- **Plain-text storage** — your tasks live in `todo.txt` (one task per line),
  which you can open, read, and hand-edit with any text editor.

## Features

| Command | What it does |
| --- | --- |
| `todo add <text...>` | Add a new task (quotes optional: `todo add Buy milk`) |
| `todo list` | Show all tasks with numbers and `[x]`/`[ ]` checkboxes |
| `todo done <number>` | Mark a task as done |
| `todo undo <number>` | Mark a task as not done |
| `todo remove <number>` | Delete one task |
| `todo clear` | Delete all tasks |
| `todo help` | Show the help message |

## Requirements

- [Rust](https://rustup.rs) 1.56 or newer (the project uses edition 2021).
  Install it with: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Build and run

```console
$ cd rust-todo
$ cargo build --release          # compiles to target/release/todo
$ cargo run -- add "Buy milk"    # or run directly during development
    Finished `release` profile [optimized] target(s) in 2.76s

$ ./target/release/todo add "Buy milk"
Added task #1: Buy milk
```

To make the `todo` command available everywhere, install it onto your PATH:

```console
$ cargo install --path .   # copies the binary to ~/.cargo/bin/todo
```

## Example session

This is a real transcript from the tested program:

```console
$ todo list
Your list is empty. Add your first task with: todo add "Buy milk"

$ todo add "Buy milk"
Added task #1: Buy milk
$ todo add "Read chapter 4 of the Rust book"
Added task #2: Read chapter 4 of the Rust book
$ todo add "Write my first unit test"
Added task #3: Write my first unit test

$ todo list
  1. [ ] Buy milk
  2. [ ] Read chapter 4 of the Rust book
  3. [ ] Write my first unit test

3 task(s): 0 done, 3 still to do

$ todo done 2
Nice! Task #2 is finished:
    Read chapter 4 of the Rust book

$ todo list
  1. [ ] Buy milk
  2. [x] Read chapter 4 of the Rust book
  3. [ ] Write my first unit test

3 task(s): 1 done, 2 still to do

$ todo remove 1
Removed task #1: Buy milk

$ todo undo 1
Task #1 is back on your plate:
    Read chapter 4 of the Rust book
```

The program also gives friendly errors and exits with a non-zero code when
something goes wrong (useful for scripts):

```console
$ todo done 9
Error: there is no task 9. Use 'todo list' to see the valid numbers.   # exit code 1

$ todo done abc
Error: 'abc' is not a valid task number                                # exit code 1

$ todo add
Error: please describe the task, for example: todo add "Buy milk"      # exit code 1
```

## Where are my tasks stored?

By default the list is saved in `todo.txt` in the folder where you run the
program. Each task is one line:

```text
[ ] Buy milk
[x] Read chapter 4 of the Rust book
```

`[x]` means done, `[ ]` means still open. Because it is plain text, you can
edit the file by hand and the program will happily load it. To keep the list
somewhere else (for example in your home folder so it works from any
directory), set the `TODO_FILE` environment variable:

```console
$ TODO_FILE="$HOME/todo.txt" todo add "Ship the project"
```

## How the code is organised (`src/main.rs`)

The file reads top to bottom, one section per idea:

1. **Data model** — a `Task` struct (`done: bool`, `text: String`) with
   `to_line()` / `from_line()` methods that convert between a task and one
   line of the save file.
2. **Storage** — `storage_path()` decides where the list lives,
   `load_tasks()` reads the file into a `Vec<Task>` (a missing file just
   means an empty list), and `save_tasks()` writes it back.
3. **Helpers** — `parse_index()` converts the user's 1-based task number
   into Rust's 0-based `Vec` index, and `print_help()` shows the usage text.
4. **Commands** — `cmd_add`, `cmd_list`, `cmd_set_done`, `cmd_remove`,
   `cmd_clear`. Every command returns `Result<(), String>` so errors are
   values instead of panics.
5. **`main`** — reads `std::env::args()`, matches the first argument against
   the command names, calls the right function, and prints any error message
   before exiting with a non-zero status.
6. **Tests** — six `#[test]` functions covering the file-format round trip,
   tolerant parsing of hand-edited lines, and index validation.

Key Rust ideas demonstrated: `struct` and `impl`, ownership and borrowing
(`&str` vs `String`, `&Vec`/slices), the `Option`/`Result` types with `?`,
iterators (`map`, `filter`, `enumerate`), pattern matching in `match`,
`strip_prefix`, formatted printing, and unit tests.

## Running the tests

```console
$ cargo test
running 6 tests
test tests::hand_edited_lines_without_marker_still_load ... ok
test tests::missing_file_means_empty_list ... ok
test tests::load_save_roundtrip_keeps_order_and_status ... ok
test tests::parse_index_converts_one_based_number_to_zero_based_index ... ok
test tests::parse_index_rejects_missing_zero_and_garbage ... ok
test tests::task_roundtrip_through_file_format ... ok

test result: ok. 6 passed; 0 failed; 0 ignored
```

## Project structure

```text
rust-todo/
├── Cargo.toml      # the package manifest: name, version, edition, dependencies
├── .gitignore      # keeps build output and todo.txt out of version control
├── README.md       # this file
└── src/
    └── main.rs     # the whole program, heavily commented
```

## Ideas for extending it

Good next exercises, roughly in order of difficulty:

1. Add a `search` command that prints only tasks containing a given word.
2. Support `todo done 1 3 5` (several numbers at once).
3. Add a priority (for example `todo add !high Pay rent`) and sort `list` by it.
4. Store due dates and colour overdue tasks using the `serde` and `chrono`
   crates — your first external dependencies.
5. Replace manual argument parsing with the `clap` crate and compare the two.
6. Persist to JSON instead of text, or add a `--json` output flag for `list`.
