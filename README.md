# Rust To-Do List Manager

A command-line to-do list application written in Rust that demonstrates core language concepts including structs, file I/O, error handling, and pattern matching.

## Overview

This project is a functional task manager that allows users to:
- Add tasks with priority levels (1-5)
- View all tasks in a formatted list
- Mark tasks as complete
- Delete tasks
- Filter tasks by priority
- Automatically save/load tasks to/from JSON

The software demonstrates Rust's unique features including ownership, borrowing, error handling patterns, and the use of external crates (serde_json) for serialization.

## Development Environment

* Rust (Edition 2021)
* Cargo (Rust's package manager)
* Visual Studio Code with Rust Analyzer extension
* Dependencies:
  - `serde` - Serialization framework
  - `serde_json` - JSON support

Rust is a systems programming language focused on three goals: safety, speed, and concurrency. This project showcases how Rust's type system and ownership model prevent common bugs at compile time.

## How to Build and Run

Ensure you have Rust and Cargo installed. If not, visit [rustup.rs](https://rustup.rs/).

### Build the project:

```bash
cd rust-todo-manager
cargo build --release
```

### Run the application:

```bash
cargo run --release
```

Or run the compiled binary directly:

```bash
./target/release/rust_todo_manager
```

### Example Usage:

```
Welcome to Rust To-Do Manager!
Type 'help' for available commands.

> add
Task title: Buy groceries
Priority (1-5): 3
✓ Task added successfully!

> list
ID   Task                       Priority Status
------------------------------------------------------------
1    Buy groceries              3        Pending

> complete
Enter task ID to mark complete: 1
✓ Task marked as complete!

> quit
Tasks saved. Goodbye!
```

## Code Structure

- `Task` struct: Represents individual to-do items with id, title, priority, and completed status.
- `TodoApp` struct: Manages the task collection and implements all business logic.
- `main()`: Interactive loop that parses user commands and calls appropriate methods.
- File I/O: Tasks are serialized to `tasks.json` using serde_json.
- Error Handling: Uses `Result<T, E>` for operations that might fail (file I/O, parsing).

## Key Learning Objectives

- **Structs and impl blocks**: Define data structures and associated methods.
- **Ownership and Borrowing**: Practice Rust's memory safety model through function parameters.
- **Pattern Matching**: Use `match` for command parsing and error handling.
- **Error Handling**: Work with `Result<T, E>` instead of exceptions.
- **File I/O**: Read and write JSON files using Cargo crates.
- **Collections**: Use `Vec<T>` for dynamic task storage.

## Future Enhancements

- Implement due dates for tasks
- Add recurring task support
- Export tasks to CSV format
- Add a graphical UI using a Rust web framework
- Write unit tests using Rust's built-in test framework

## Compilation and Execution Notes

The Rust compiler is strict about type safety and ownership, which helps prevent bugs at compile time. First-time Rust developers may encounter compiler errors that feel cryptic at first, but they are designed to guide you toward correct code. The compiler's error messages are generally helpful—read them carefully!

To check code without building:

```bash
cargo check
```

To run tests (when added):

```bash
cargo test
```

## Author

Christopher Edeson Effiong  
CSE 310: Applied Programming  
BYU-Pathway Worldwide

## Learning Log

Time spent: 21.5 hours  

## Additional Resources

* [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - Official guide
* [Rust By Example](https://doc.rust-lang.org/rust-by-example/) - Practical examples
* [serde Documentation](https://serde.rs/) - Serialization framework
* [Cargo Book](https://doc.rust-lang.org/cargo/) - Package manager guide
