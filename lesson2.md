## Project 2: Guessing Game (Memory & Ownership Fundamentals)

**Concepts Covered:** Ownership, Borrowing, References, Result/Option Handling, External Crates (Packages).

In JS, the Garbage Collector (GC) manages memory. In Rust, **Ownership** governs memory. This project demonstrates how Rust manages data without a GC.

---

### 1. Initialize Project

```bash
cargo new guessing_game
cd guessing_game

```

### 2. Adding Dependencies

In Node.js, you use `npm install`. In Rust, you edit `Cargo.toml` or use `cargo add`. We need the `rand` crate to generate a secret number.

**Run:**

```bash
cargo add rand

```

*Note: This updates your `Cargo.toml` file under `[dependencies]`.*

---

### 3. Implementation Step-by-Step

#### Step A: Generating a Random Number

We use the `rand` crate. In Rust, range syntax is `start..=end` (inclusive).

```rust
use std::io;
use rand::Rng; // "Trait" that allows us to use random number methods

fn main() {
    println!("Guess the number!");

    // thread_rng() gives a random generator local to the current thread
    // gen_range() takes a range expression
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}"); // Interpolation like JS `${}`
}

```

#### Step B: The Infinite Loop and Memory Ownership

We want the user to keep guessing until they win. We use the `loop` keyword.

```rust
loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    // Ownership check: 'guess' is a String (heap allocated).
    // '&mut guess' borrows the string, allowing read_line to fill it
    // without taking ownership of the variable.
}

```

#### Step C: Handling Errors Gracefully

Instead of `.expect()` (which crashes), we use `match` to handle the `Result` from `parse()`.

```rust
// Shadowing guess from String to u32 (unsigned 32-bit integer)
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,     // If successful, return the number
    Err(_) => continue, // If error (e.g., user typed "hi"), restart the loop
};

```

#### Step D: Comparing Values

To compare the guess to the secret number, we use `std::cmp::Ordering`.

```rust
use std::cmp::Ordering;

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break; // Exit the loop
    }
}

```

---

### 4. Technical Breakdown: Ownership vs. JS References

* **JS/TS:** If you pass an object to a function, both the caller and the function share it. If the function modifies it, the caller sees it.
* **Rust (Ownership):** 1. Each value has a **Variable Owner**.
2. When the owner goes out of scope, the memory is dropped.
3. You can **Borrow** (`&`) a value, but you can only have **one** mutable borrow (`&mut`) at a time. This prevents "Data Races" that are common in Node.js asynchronous code.

---

### 5. Documentation References

* **What is Ownership?:** [The Rust Book - Ch 4.1](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
* **References and Borrowing:** [The Rust Book - Ch 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
* **Using External Crates:** [Cargo Guide](https://doc.rust-lang.org/cargo/guide/dependencies.html)

### Suggestions for Practice

1. **Restrict the range:** Change the code to only accept numbers between 1 and 100.
2. **Count attempts:** Initialize a counter outside the loop and increment it every time the user guesses. Print the total attempts when they win.### Refined Guessing Game Logic

The logic in your snippet was fragmented. Below is the consolidated, syntactically correct version followed by a breakdown of the mechanics.

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Replace: let secret_number = rand::thread_rng().gen_range(1..=100);
// With the new 0.9 syntax:
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert String to Number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        // Compare guess to secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; 
            }
        }
    }
}

```

---

### Key Concept Breakdown

#### 1. References & Borrowing (`&mut guess`)

In JS, passing an object to a function usually passes a reference. In Rust, you must be explicit.

* `guess` is the **Owner** of the string.
* `&mut guess` says: "I am lending this string to `read_line` so it can write to it, but I want it back when it's done."

#### 2. Type Shadowing

```rust
let mut guess = String::new(); // Initially a String
// ... 
let guess: u32 = ...           // Now it's an Integer

```

In TypeScript, you cannot change a variable's type after declaration. In Rust, **Shadowing** allows you to reuse the same name for a transformation. This is cleaner than creating `guess_str` and `guess_int`.

#### 3. Match vs. If/Else

The `match` expression is your primary tool. It forces you to handle every possibility.

* `parse()` returns a `Result`. A `Result` is an Enum that is either `Ok(value)` or `Err(error)`.
* If you don't handle the `Err` case, Rust will not compile. This eliminates runtime "undefined is not a function" errors.

#### 4. The `Ordering` Enum

The `.cmp()` method returns one of three variants of the `Ordering` enum: `Less`, `Greater`, or `Equal`. This is more readable than the JS pattern of returning `-1`, `1`, or `0`.

---

### Implementation Comparison

| Feature | JavaScript / Node.js | Rust |
| --- | --- | --- |
| **Random** | `Math.random()` | `rand::thread_rng().gen_range()` |
| **Looping** | `while(true)` | `loop` |
| **Breaking** | `break` | `break` |
| **Input** | `readline` module (Async) | `io::stdin().read_line` (Sync) |
| **Types** | Dynamic / Optional (TS) | Static & Mandatory |

---

### Deep Reading

* **The Match Control Flow:** [Rust Book 6.2](https://doc.rust-lang.org/book/ch06-02-match.html)
* **Result Type handling:** [Rust Book 9.2](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)






