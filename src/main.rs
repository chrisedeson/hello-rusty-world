use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

// Represents a single to-do task with metadata.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    priority: u8,
    completed: bool,
}

// Manages a collection of tasks and provides operations on them.
struct TodoApp {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TodoApp {
    // Creates a new empty TodoApp.
    fn new() -> Self {
        TodoApp {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    // Adds a new task to the app with the given title and priority.
    fn add_task(&mut self, title: String, priority: u8) {
        let task = Task {
            id: self.next_id,
            title,
            priority: priority.min(5).max(1), // Clamp priority between 1 and 5
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✓ Task added successfully!");
    }

    // Displays all tasks in a formatted table.
    fn display_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks yet. Add one to get started!");
            return;
        }

        println!("\n{:<4} {:<30} {:<8} {:<12}", "ID", "Task", "Priority", "Status");
        println!("{}", "-".repeat(60));

        for task in &self.tasks {
            let status = if task.completed { "✓ Complete" } else { "Pending" };
            println!(
                "{:<4} {:<30} {:<8} {:<12}",
                task.id, task.title, task.priority, status
            );
        }
        println!();
    }

    // Marks a task as complete by its ID.
    fn mark_complete(&mut self, id: u32) -> Result<(), String> {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("✓ Task marked as complete!");
                return Ok(());
            }
        }
        Err(format!("Task with ID {} not found.", id))
    }

    // Deletes a task by its ID.
    fn delete_task(&mut self, id: u32) -> Result<(), String> {
        let initial_len = self.tasks.len();
        self.tasks.retain(|task| task.id != id);
        if self.tasks.len() < initial_len {
            println!("✓ Task deleted successfully!");
            Ok(())
        } else {
            Err(format!("Task with ID {} not found.", id))
        }
    }

    // Saves all tasks to a JSON file.
    fn save_tasks(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(filename, json)?;
        Ok(())
    }

    // Loads tasks from a JSON file. Creates an empty file if it doesn't exist.
    fn load_tasks(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        match fs::read_to_string(filename) {
            Ok(contents) => {
                if contents.trim().is_empty() {
                    // File exists but is empty; start fresh
                    self.tasks = Vec::new();
                } else {
                    self.tasks = serde_json::from_str(&contents)?;
                    // Update next_id to avoid reusing IDs
                    if let Some(task) = self.tasks.iter().max_by_key(|t| t.id) {
                        self.next_id = task.id + 1;
                    }
                }
                Ok(())
            }
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                // File doesn't exist; create an empty one
                fs::write(filename, "[]")?;
                self.tasks = Vec::new();
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    // Filters and displays tasks by priority level.
    fn filter_by_priority(&self, priority: u8) {
        let filtered: Vec<&Task> = self.tasks.iter().filter(|t| t.priority == priority).collect();

        if filtered.is_empty() {
            println!("No tasks with priority {}.", priority);
            return;
        }

        println!("\n{:<4} {:<30} {:<8} {:<12}", "ID", "Task", "Priority", "Status");
        println!("{}", "-".repeat(60));

        for task in filtered {
            let status = if task.completed { "✓ Complete" } else { "Pending" };
            println!(
                "{:<4} {:<30} {:<8} {:<12}",
                task.id, task.title, task.priority, status
            );
        }
        println!();
    }
}

// Main entry point. Runs the interactive menu loop.
fn main() {
    let mut app = TodoApp::new();
    let filename = "tasks.json";

    // Load existing tasks from file on startup.
    if let Err(e) = app.load_tasks(filename) {
        eprintln!("Warning: Could not load tasks: {}", e);
    }

    println!("Welcome to Rust To-Do Manager!");
    println!("Type 'help' for available commands.\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let trimmed = input.trim();

        match trimmed {
            "1" | "add" => {
                print!("Task title: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read title");

                print!("Priority (1-5): ");
                io::stdout().flush().unwrap();
                let mut priority_str = String::new();
                io::stdin()
                    .read_line(&mut priority_str)
                    .expect("Failed to read priority");

                if let Ok(priority) = priority_str.trim().parse::<u8>() {
                    app.add_task(title.trim().to_string(), priority);
                } else {
                    println!("Invalid priority. Please enter a number between 1 and 5.");
                }
            }
            "2" | "list" => {
                app.display_tasks();
            }
            "3" | "complete" => {
                print!("Enter task ID to mark complete: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin()
                    .read_line(&mut id_str)
                    .expect("Failed to read ID");

                if let Ok(id) = id_str.trim().parse::<u32>() {
                    if let Err(e) = app.mark_complete(id) {
                        println!("Error: {}", e);
                    }
                } else {
                    println!("Invalid ID. Please enter a number.");
                }
            }
            "4" | "delete" => {
                print!("Enter task ID to delete: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin()
                    .read_line(&mut id_str)
                    .expect("Failed to read ID");

                if let Ok(id) = id_str.trim().parse::<u32>() {
                    if let Err(e) = app.delete_task(id) {
                        println!("Error: {}", e);
                    }
                } else {
                    println!("Invalid ID. Please enter a number.");
                }
            }
            "5" | "filter" => {
                print!("Filter by priority (1-5): ");
                io::stdout().flush().unwrap();
                let mut priority_str = String::new();
                io::stdin()
                    .read_line(&mut priority_str)
                    .expect("Failed to read priority");

                if let Ok(priority) = priority_str.trim().parse::<u8>() {
                    app.filter_by_priority(priority);
                } else {
                    println!("Invalid priority. Please enter a number between 1 and 5.");
                }
            }
            "help" => {
                println!("\nAvailable commands:");
                println!("  1 or 'add'      - Add a new task");
                println!("  2 or 'list'     - Display all tasks");
                println!("  3 or 'complete' - Mark a task as complete");
                println!("  4 or 'delete'   - Delete a task");
                println!("  5 or 'filter'   - Filter tasks by priority");
                println!("  'help'          - Show this help menu");
                println!("  'quit'          - Exit the program\n");
            }
            "quit" | "exit" => {
                // Save tasks before exiting.
                if let Err(e) = app.save_tasks(filename) {
                    eprintln!("Error saving tasks: {}", e);
                } else {
                    println!("Tasks saved. Goodbye!");
                }
                break;
            }
            _ => {
                println!("Unknown command. Type 'help' for available commands.");
            }
        }
    }
}
