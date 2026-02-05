## Project 1: Command-Line Calculator

### 1. Setup & Environment

Initialize a new Rust project using Cargo (Rust's package manager, equivalent to `npm`).

```bash
cargo new cli_calculator
cd cli_calculator

```

Open `src/main.rs`. This is your entry point.

---

### 2. Core Concepts for JS Developers

Before writing code, understand the architectural differences:

* **Mutability:** In JS, `let` allows reassignment. In Rust, `let` is immutable by default (like JS `const`). You must explicitly use `let mut` to change a value.
* **Input/Output:** Rust is systems-level. It does not implicitly handle Input/Output (I/O) streams. You must import the standard library `std::io`.
* **Error Handling:** Rust does not use `try/catch` for standard errors. It uses the `Result` type (Success or Failure).

---

### 3. Step-by-Step Implementation

#### Step A: Importing Modules and Reading Input

Rust standard library modules must be brought into scope.

```rust
use std::io; // Import the input/output library

fn main() {
    println!("Enter the first number:");
    
    // Create a mutable String buffer to store input
    // String::new() creates an empty string on the heap
    let mut num1_str = String::new(); 

    // Read from standard input
    io::stdin()
        .read_line(&mut num1_str) // Pass a mutable reference to the string
        .expect("Failed to read line"); // Handle potential failure immediately
}

```

**Breakdown:**

1. `let mut`: Explicitly mutable. If you used `let`, `read_line` could not modify the variable.
2. `&mut num1_str`: **References**. In JS, objects are passed by reference automatically. In Rust, `&` creates a reference. `mut` states this reference allows modification of the underlying data.
3. `.expect()`: `read_line` returns a `Result`. If the result is an error (IO failure), `expect` crashes the program with the message.

#### Step B: Parsing (String to Number)

Input comes as a string (e.g., "5\n"). It must be trimmed and parsed.

```rust
// Shadowing: We re-declare 'num1' to convert types. 
// Valid in Rust, avoids creating 'num1_parsed', 'num1_str', etc.
let num1: f64 = num1_str.trim().parse().expect("Please type a number!");

```

**Breakdown:**

1. `f64`: Explicit type annotation. Double-precision float (JS `Number`).
2. `.trim()`: Removes whitespace and the newline character (`\n`) captured by Enter.
3. `.parse()`: Analyzes the string to find the type specified (`f64`).
4. `Shadowing`: Reusing the variable name `num1` for a new type is idiomatic in Rust.

#### Step C: Full Logic with Match

Replicate logic for the second number and the operator, then compute.

```rust
use std::io;

fn main() {
    // 1. Get First Number
    println!("Enter first number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Read error");
    let num1: f64 = input1.trim().parse().expect("Invalid number");

    // 2. Get Operator
    println!("Enter operator (+, -, *, /):");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Read error");
    let operator = operator.trim(); // Shadow to remove newline

    // 3. Get Second Number
    println!("Enter second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Read error");
    let num2: f64 = input2.trim().parse().expect("Invalid number");

    // 4. Operation Logic using Match
    // 'match' is like JS 'switch' but enforces exhaustiveness (must handle all cases)
    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Cannot divide by zero");
                return; // Exit function early
            }
            num1 / num2
        },
        _ => { // The 'default' case (wildcard)
            println!("Unknown operator");
            return;
        }
    };

    println!("Result: {}", result);
}

```

---

### 4. Code Breakdown & Analysis

#### The `match` Expression

Unlike JS `switch`, `match` is an expression, meaning it returns a value.

* **JS:** You set a variable inside the switch case.
* **Rust:** The `match` block *evaluates* to the result, which is assigned to `result`.

#### Strict Typing

If you try to add `num1` (f64) to an integer `5` (i32), Rust will fail to compile. JS implicitly converts types (`"5" + 5 = "55"`). Rust prohibits this safety hazard.

#### Error Handling (The `.expect` method)

In production Rust, we do not use `.expect()`. We use `match` on the Result type to handle errors gracefully without crashing.

* **Current:** `parse().expect(...)` -> Crashes if input is "abc".
* **Better:** Check if `parse()` returns `Ok(val)` or `Err(e)`. (Covered in later lessons).

---

### 5. Documentation References

Read these specific sections to solidify understanding:

* **Variables & Mutability:** [The Rust Programming Language - Ch 3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
* **Data Types (Scalar vs Compound):** [The Rust Programming Language - Ch 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html)
* **Control Flow (if/else/loops):** [The Rust Programming Language - Ch 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
* **Match Control Flow:** [The Rust Programming Language - Ch 6.2](https://doc.rust-lang.org/book/ch06-02-match.html)