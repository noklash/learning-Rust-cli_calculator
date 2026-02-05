### Environment Verification

The switch to the `msvc` toolchain resolved the `dlltool.exe` error. This confirms your system is now correctly using the Microsoft C++ Build Tools for compilation.

### Resolving Deprecation Warnings

In Rust, "Deprecated" means the code still works (as seen in your successful run), but the library authors have introduced a newer, preferred way to write it. The `rand` crate was recently updated to version `0.9`, which renamed several core functions.

To clean up your code and match the latest standards, update your `src/bin/guessing_game.rs` as follows:

```rust
// Replace: let secret_number = rand::thread_rng().gen_range(1..=100);
// With the new 0.9 syntax:
let secret_number = rand::rng().random_range(1..=100);

```

---

## Lesson 3: Memory Layout & The Todo CLI

To build a Todo List, you need to store multiple items in memory. In JS, you just push to an array. In Rust, you must understand **where** that data is stored to manage it safely.

### 1. Stack vs. Heap

* **The Stack:** Fast, fixed size. Stores primitive types (integers, booleans) and pointers.
* **The Heap:** Slower, flexible size. Stores dynamic data like `String` and `Vec` (Arrays).

### 2. Implementation: The Todo CLI

Create a new file: `src/bin/todo.rs`.

```rust
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

```

---

### Concepts Explained for the TS/Node Developer

#### A. Strings: `String` vs `&str`

This is the most common hurdle.

* **`String`:** Owned, heap-allocated. Use this when you need to modify text or store it in a struct (like our `Todo`).
* **`&str`:** A "slice" or reference to text. It is a pointer to a string owned by someone else. In the code above, `input.trim()` returns a `&str`.

#### B. The `&` and `&mut` in Loops

* `for item in &todos`: We are **borrowing** the items to read them. If we didn't use `&`, Rust would try to **move** the items out of the vector, destroying the vector in the process.
* `for item in &mut todos`: We are **borrowing mutably** because we need to change the `completed` status.

#### C. Ownership of the Struct

When you do `todos.push(Todo { ... })`, the `Todo` instance is **moved** into the vector. The vector now "owns" that data. When the vector goes out of scope at the end of `main()`, it automatically cleans up the memory for every Todo item inside it. **No Garbage Collector needed.**

---

### Documentation References

* **Common Collections (Vectors):** [The Rust Book - Ch 8.1](https://doc.rust-lang.org/book/ch08-01-vectors.html)
* **The String Type:** [The Rust Book - Ch 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
* **References and Borrowing:** [The Rust Book - Ch 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

**Run this lesson with:**
`cargo run --bin todo`