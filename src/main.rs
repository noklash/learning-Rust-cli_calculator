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