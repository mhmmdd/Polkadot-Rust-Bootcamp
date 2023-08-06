use std::io;
use std::io::Write;
use std::str::FromStr;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b != 0.0 {
                a / b
            } else {
                panic!("Cannot divide by zero")
            }
        }
    }
}

fn main() {
    let mut input = String::new();

    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let num1: f64 = f64::from_str(input.trim()).unwrap();

    input.clear();
    print!("Enter the operation (Add, Subtract, Multiply, Divide): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let operation = input.trim().to_string();

    input.clear();
    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let num2: f64 = f64::from_str(input.trim()).unwrap();

    let result = match operation.to_lowercase().as_str() {
        "add" => calculate(Operation::Add(num1, num2)),
        "subtract" => calculate(Operation::Subtract(num1, num2)),
        "multiply" => calculate(Operation::Multiply(num1, num2)),
        "divide" => calculate(Operation::Divide(num1, num2)),
        _ => panic!("Invalid operation"),
    };

    println!("Result: {}", result);
}
