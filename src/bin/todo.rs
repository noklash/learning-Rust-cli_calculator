use std::io::{self, Write}; // Removed unused BufRead, added Write

#[derive(Debug)]
struct Todo {
    id: u32,
    task: String,
    completed: bool,
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    let mut next_id: u32 = 1;

    println!("=== GMMAWAVE TODO CLI ===");
    println!("Commands: add <task>  |  list  |  done <id>  |  quit\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input_buffer = String::new();
        io::stdin().read_line(&mut input_buffer).expect("Failed to read");
        
        // Convert to lowercase and get a string slice (&str) for matching
        let input = input_buffer.trim().to_lowercase();
        let input_slice = input.as_str(); 

        match input_slice {
            "quit" => {
                println!("See you next time! 👋");
                break;
            }
            "list" => {
                if todos.is_empty() {
                    println!("No tasks yet.");
                } else {
                    for todo in &todos {
                        let mark = if todo.completed { "[✔]" } else { "[ ]" };
                        println!("{:3} {} {}", todo.id, mark, todo.task);
                    }
                }
            }
            // Use 'guards' (the 'if' part) to handle dynamic strings in match
            s if s.starts_with("add ") => {
                let task = s.strip_prefix("add ").unwrap().trim();
                if !task.is_empty() {
                    todos.push(Todo {
                        id: next_id,
                        task: task.to_string(),
                        completed: false,
                    });
                    println!("✓ Added task #{}", next_id);
                    next_id += 1;
                }
            }
            s if s.starts_with("done ") => {
                let id_part = s.strip_prefix("done ").unwrap().trim();
                if let Ok(id) = id_part.parse::<u32>() {
                    let mut found = false;
                    for todo in &mut todos {
                        if todo.id == id {
                            todo.completed = true;
                            println!("✓ Task #{} marked done", id);
                            found = true;
                            break;
                        }
                    }
                    if !found { println!("Task not found."); }
                }
            }
            _ => println!("Unknown command."),
        }
    }
}