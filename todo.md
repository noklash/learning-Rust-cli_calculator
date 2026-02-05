
## Lesson 3: Building a Todo CLI – Understanding Memory & Lists in Rust

Coming from JavaScript? In JS you do `todos.push({})` and forget about memory.  
Rust makes you think about **where** data lives — but it gives you **safety guarantees** in return (no random crashes or memory leaks).

### Stack vs Heap – Super Simple Version

- **Stack** = fast drawer, fixed size  
  → numbers, booleans, small fixed things live here

- **Heap** = big flexible storage room  
  → `String`, `Vec<Todo>`, anything that can grow or change size lives here

`Vec` (vector) = Rust's growable array → lives on the **heap**  
→ almost the same feeling as JS `[]`, but with strict rules about who "owns" the data.

### Hands-on: Todo CLI

Create this file: `src/bin/todo.rs`

```rust
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
```

Run it:

```bash
cargo run --bin todo
```

### Key Concepts Explained (TS/JS → Rust)

1. **`String` vs `&str`**

   | Type     | Meaning                              | Lives on | Can I change it? | Common use |
   |----------|--------------------------------------|----------|------------------|------------|
   | `String` | Owned text — full copy on heap       | Heap     | Yes              | Store in struct, return from fn |
   | `&str`   | View / borrow of text                | —        | No               | Function arguments, literals |

   → `input.trim()` gives `&str` → we call `.to_string()` to make owned `String`

2. **`&` and `&mut` – Borrowing**

   ```rust
   for todo in &todos { ... }          // read-only view (like const in JS)
   for todo in &mut todos { ... }      // read + write permission
   ```

   Without `&`, Rust thinks you want to **steal** the items out of the Vec → error!

3. **Ownership = Automatic Cleanup**

   ```rust
   todos.push(Todo { ... });
   ```

   → The `Vec` now **owns** that `Todo`  
   → When `main()` ends → `Vec` is dropped → all `Todo`s are automatically freed  
   → No garbage collector, no `free()`, no leaks

### Quick Tips for Next Steps

- Want to save todos to a file? → Look at `std::fs` and `serde_json`
- Want better parsing? → Consider the `clap` crate for real CLI args
- Errors annoying? → Later you'll learn `Result` + `?` operator

### Great References (Start Here)

- [Rust Book – Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [Rust Book – Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [Rust Book – References & Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

You've now built your first real Rust program with dynamic data — awesome progress!  
Let me know how it runs or if you want to add save/load next. 🚀