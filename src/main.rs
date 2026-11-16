//! todo — a tiny command-line todo list.
//!
//! This is a beginner-friendly first Rust project: the whole program lives
//! in this one file, uses ONLY the Rust standard library (no external
//! crates), and stores your tasks in a plain text file you can open and
//! edit with any text editor.
//!
//! # Examples
//! ```text
//! todo add "Buy milk"        # add a task
//! todo list                  # show all tasks
//! todo done 1                # mark task 1 as done
//! ```

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

/// Default name of the file where tasks are saved (in the current folder).
const DEFAULT_FILE: &str = "todo.txt";

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

/// One task on the list.
///
/// `done` says whether the task is finished, `text` is what you wrote.
struct Task {
    done: bool,
    text: String,
}

impl Task {
    /// Create a new task that is not done yet.
    fn new(text: String) -> Task {
        Task { done: false, text }
    }

    /// Turn the task into one line of the save file: `[x] Buy milk`.
    fn to_line(&self) -> String {
        let marker = if self.done { "[x]" } else { "[ ]" };
        format!("{} {}", marker, self.text)
    }

    /// Read one line of the save file back into a `Task`.
    ///
    /// Lines starting with `[x] ` are done, lines with `[ ] ` are open.
    /// Anything else (for example a line typed by hand) counts as an open
    /// task, so we never lose data.
    fn from_line(line: &str) -> Task {
        if let Some(text) = line.strip_prefix("[x] ") {
            Task { done: true, text: text.to_string() }
        } else if let Some(text) = line.strip_prefix("[ ] ") {
            Task { done: false, text: text.to_string() }
        } else {
            Task { done: false, text: line.to_string() }
        }
    }
}

// ---------------------------------------------------------------------------
// Storage: reading and writing the todo.txt file
// ---------------------------------------------------------------------------

/// Where the list is saved.
///
/// By default it is `todo.txt` in the folder you run the program from.
/// Set the `TODO_FILE` environment variable to use a different file.
fn storage_path() -> PathBuf {
    match env::var_os("TODO_FILE") {
        Some(value) => PathBuf::from(value),
        None => PathBuf::from(DEFAULT_FILE),
    }
}

/// Read the file and return the tasks.
///
/// A missing file is not an error — it just means the list is empty.
fn load_tasks(path: &Path) -> Vec<Task> {
    let contents = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(_) => return Vec::new(),
    };
    contents
        .lines()                                  // split the text into lines
        .filter(|line| !line.trim().is_empty())   // skip blank lines
        .map(Task::from_line)                     // each line becomes a Task
        .collect()                                // gather them into a Vec<Task>
}

/// Write all tasks back to the file, one per line.
fn save_tasks(path: &Path, tasks: &[Task]) -> Result<(), String> {
    let mut contents = String::new();
    for task in tasks {
        contents.push_str(&task.to_line());
        contents.push('\n');
    }
    fs::write(path, contents)
        .map_err(|error| format!("could not write to {}: {}", path.display(), error))
}

// ---------------------------------------------------------------------------
// Small helpers shared by several commands
// ---------------------------------------------------------------------------

/// Turn the command-line number (like "2") into a Vec index (like 1).
///
/// Users count tasks from 1, but Rust `Vec`s count from 0, so we subtract 1.
fn parse_index(args: &[String]) -> Result<usize, String> {
    let raw = match args.first() {
        Some(value) => value,
        None => {
            return Err("please provide a task number, for example: todo done 1".to_string())
        }
    };
    let number: usize = raw
        .parse()
        .map_err(|_| format!("'{}' is not a valid task number", raw))?;
    if number == 0 {
        return Err("task numbers start at 1, not 0".to_string());
    }
    Ok(number - 1)
}

/// Print the help message shown when you run `todo help` (or a bad command).
fn print_help() {
    println!("A tiny command-line todo list. Tasks are stored in a plain text file.");
    println!();
    println!("USAGE:");
    println!("    todo <COMMAND> [ARGUMENTS]");
    println!();
    println!("COMMANDS:");
    println!("    add <text...>    Add a new task             todo add Buy milk");
    println!("    list             Show all tasks             todo list");
    println!("    done <number>    Mark a task as done        todo done 2");
    println!("    undo <number>    Mark a task as not done    todo undo 2");
    println!("    remove <number>  Delete a task              todo remove 3");
    println!("    clear            Delete all tasks           todo clear");
    println!("    help             Show this help message");
    println!();
    println!("The list is saved in 'todo.txt' in the current folder.");
    println!("Set the TODO_FILE environment variable to use another file.");
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// `todo add Buy milk` — append a new task to the list.
fn cmd_add(args: &[String]) -> Result<(), String> {
    // All the words after `add` become the task text, so quotes are optional.
    let text = args.join(" ");
    if text.trim().is_empty() {
        return Err("please describe the task, for example: todo add \"Buy milk\"".to_string());
    }

    let path = storage_path();
    let mut tasks = load_tasks(&path);
    tasks.push(Task::new(text));
    let id = tasks.len(); // 1-based number the user will see
    save_tasks(&path, &tasks)?;

    println!("Added task #{}: {}", id, tasks[id - 1].text);
    Ok(())
}

/// `todo list` — print every task with its number and a done checkbox.
fn cmd_list() -> Result<(), String> {
    let path = storage_path();
    let tasks = load_tasks(&path);

    if tasks.is_empty() {
        println!("Your list is empty. Add your first task with: todo add \"Buy milk\"");
        return Ok(());
    }

    // `enumerate()` gives us (position, item) pairs, counting from 0.
    for (index, task) in tasks.iter().enumerate() {
        let marker = if task.done { "x" } else { " " };
        println!("{:>3}. [{}] {}", index + 1, marker, task.text);
    }

    let done_count = tasks.iter().filter(|task| task.done).count();
    println!(
        "\n{} task(s): {} done, {} still to do",
        tasks.len(),
        done_count,
        tasks.len() - done_count
    );
    Ok(())
}

/// `todo done 2` / `todo undo 2` — set the "done" flag of one task.
fn cmd_set_done(args: &[String], done: bool) -> Result<(), String> {
    let index = parse_index(args)?;
    let path = storage_path();
    let mut tasks = load_tasks(&path);

    if index >= tasks.len() {
        return Err(format!(
            "there is no task {}. Use 'todo list' to see the valid numbers.",
            index + 1
        ));
    }

    tasks[index].done = done;
    save_tasks(&path, &tasks)?;

    if done {
        println!("Nice! Task #{} is finished:", index + 1);
    } else {
        println!("Task #{} is back on your plate:", index + 1);
    }
    println!("    {}", tasks[index].text);
    Ok(())
}

/// `todo remove 3` — delete one task from the list.
fn cmd_remove(args: &[String]) -> Result<(), String> {
    let index = parse_index(args)?;
    let path = storage_path();
    let mut tasks = load_tasks(&path);

    if index >= tasks.len() {
        return Err(format!(
            "there is no task {}. Use 'todo list' to see the valid numbers.",
            index + 1
        ));
    }

    // Vec::remove shifts the later tasks left, so the numbering stays tight.
    let removed = tasks.remove(index);
    save_tasks(&path, &tasks)?;

    println!("Removed task #{}: {}", index + 1, removed.text);
    Ok(())
}

/// `todo clear` — delete all tasks (and the save file itself).
fn cmd_clear() -> Result<(), String> {
    let path = storage_path();
    match fs::remove_file(&path) {
        Ok(()) => println!("All tasks removed. The list is empty again."),
        Err(_) => println!("The list is already empty (no file at {}).", path.display()),
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// main: look at the first argument and call the right command
// ---------------------------------------------------------------------------

fn main() {
    // env::args() gives the executable path too, so skip(1) drops it.
    let args: Vec<String> = env::args().skip(1).collect();

    // No command at all is treated like `list` — the friendliest default.
    let command = args.first().map(String::as_str).unwrap_or("list");

    // Everything after the command word, for example the task number.
    let rest: &[String] = args.get(1..).unwrap_or(&[]);

    let result = match command {
        "add" => cmd_add(rest),
        "list" | "ls" => cmd_list(),
        "done" => cmd_set_done(rest, true),
        "undo" => cmd_set_done(rest, false),
        "remove" | "rm" => cmd_remove(rest),
        "clear" => cmd_clear(),
        "help" | "--help" | "-h" => {
            print_help();
            Ok(())
        }
        "version" | "--version" | "-V" => {
            println!("todo 0.1.0");
            Ok(())
        }
        other => {
            eprintln!("Error: unknown command '{}'.\n", other);
            print_help();
            process::exit(2);
        }
    };

    // Commands return Result<(), String>: Ok(()) on success, or an error
    // message we print here and turn into a non-zero exit code.
    if let Err(message) = result {
        eprintln!("Error: {}", message);
        process::exit(1);
    }
}

// ---------------------------------------------------------------------------
// Tests — run them with `cargo test`
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_roundtrip_through_file_format() {
        let open_task = Task::new("Read chapter 4".to_string());
        let done_task = Task { done: true, text: "Buy milk".to_string() };

        let open_line = open_task.to_line();
        let done_line = done_task.to_line();

        assert_eq!(open_line, "[ ] Read chapter 4");
        assert_eq!(done_line, "[x] Buy milk");

        let open_back = Task::from_line(&open_line);
        let done_back = Task::from_line(&done_line);
        assert!(!open_back.done);
        assert_eq!(open_back.text, "Read chapter 4");
        assert!(done_back.done);
        assert_eq!(done_back.text, "Buy milk");
    }

    #[test]
    fn hand_edited_lines_without_marker_still_load() {
        let task = Task::from_line("An old line typed by hand");
        assert!(!task.done);
        assert_eq!(task.text, "An old line typed by hand");
    }

    #[test]
    fn parse_index_converts_one_based_number_to_zero_based_index() {
        let args = vec!["2".to_string()];
        assert_eq!(parse_index(&args).unwrap(), 1);
    }

    #[test]
    fn parse_index_rejects_missing_zero_and_garbage() {
        assert!(parse_index(&[]).is_err());
        assert!(parse_index(&["0".to_string()]).is_err());
        assert!(parse_index(&["abc".to_string()]).is_err());
        assert!(parse_index(&["-1".to_string()]).is_err());
    }

    #[test]
    fn load_save_roundtrip_keeps_order_and_status() {
        let path = PathBuf::from("todo_test_tmp.txt");
        let tasks = vec![
            Task::new("First".to_string()),
            Task { done: true, text: "Second".to_string() },
        ];

        save_tasks(&path, &tasks).expect("saving should work");
        let loaded = load_tasks(&path);
        let _ = fs::remove_file(&path); // clean up even though asserts come next

        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].text, "First");
        assert!(!loaded[0].done);
        assert_eq!(loaded[1].text, "Second");
        assert!(loaded[1].done);
    }

    #[test]
    fn missing_file_means_empty_list() {
        let loaded = load_tasks(Path::new("definitely-missing-file-todo.txt"));
        assert!(loaded.is_empty());
    }
}
