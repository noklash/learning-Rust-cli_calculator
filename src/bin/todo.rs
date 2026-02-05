use std::io;

// 1. Define the structure of a Todo item
struct Todo {
    id: u32,
    task: String,
    completed: bool,
}

fn main() {
    // 2. The Vector (Vec) is a heap-allocated, growable array
    let mut todos: Vec<Todo> = Vec::new();
    let mut next_id = 1;

    println!("--- GMMAWAVE TODO CLI ---");

    loop {
        println!("\nEnter command: (add [task] / list / done [id] / quit)");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let input = input.trim();

        // 3. Simple Command Parsing
        if input == "quit" {
            break;
        } else if input.starts_with("add ") {
            let task = input.replace("add ", "");
            todos.push(Todo {
                id: next_id,
                task,
                completed: false,
            });
            next_id += 1;
        } else if input == "list" {
            for item in &todos { // 4. BORROWING the list for iteration
                let status = if item.completed { "[X]" } else { "[ ]" };
                println!("{} {} - {}", item.id, status, item.task);
            }
        } else if input.starts_with("done ") {
            let id_str = input.replace("done ", "");
            if let Ok(id) = id_str.parse::<u32>() {
                // 5. Finding and modifying an item in the vector
                for item in &mut todos { // Borrowing MUTABLY
                    if item.id == id {
                        item.completed = true;
                    }
                }
            }
        }
    }
}