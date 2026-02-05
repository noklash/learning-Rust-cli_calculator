use std::io::{self, BufRead};

#[derive(Debug)] // ← lets us print Todo easily for debugging
struct Todo {
    id: u32,
    task: String,
    completed: bool,
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    let mut next_id: u32 = 1;

    println!("=== TODO APP (add/list/done/quit) ===");

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap(); // makes sure prompt shows immediately

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        let input = input.trim().to_lowercase();

        if input == "quit" {
            println!("Goodbye!");
            break;
        }

        if input.starts_with("add ") {
            let task = input.strip_prefix("add ").unwrap().trim().to_string();
            if task.is_empty() {
                println!("Please enter a task after 'add'");
                continue;
            }

            todos.push(Todo {
                id: next_id,
                task,
                completed: false,
            });

            println!("Added task #{}", next_id);
            next_id += 1;
        }
        else if input == "list" {
            if todos.is_empty() {
                println!("No tasks yet!");
            } else {
                for todo in &todos {
                    let mark = if todo.completed { "[x]" } else { "[ ]" };
                    println!("{} {} {}", todo.id, mark, todo.task);
                }
            }
        }
        else if input.starts_with("done ") {
            let id_part = input.strip_prefix("done ").unwrap().trim();
            if let Ok(id) = id_part.parse::<u32>() {
                let mut found = false;
                for todo in &mut todos {
                    if todo.id == id {
                        todo.completed = true;
                        println!("Marked task #{} as done!", id);
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("Task #{} not found.", id);
                }
            } else {
                println!("Invalid ID — use a number");
            }
        }
        else {
            println!("Unknown command. Try: add [task] / list / done [id] / quit");
        }
    }
}