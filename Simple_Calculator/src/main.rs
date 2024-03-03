use std::io;

fn main() {
    println!("Simple Calculator!");

    let first_value = get_user_input("Enter the first number:");

    println!("Please choose the operation:");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");

    let operation = get_user_input("Enter the operation (1-4):");

    let second_value = get_user_input("Enter the second number:");

    // Parse input values
    let first_value: f64 = first_value.trim().parse().expect("Please enter a valid floating-point number");
    let operation: i32 = operation.trim().parse().expect("Please enter a valid integer (1-4)");
    let second_value: f64 = second_value.trim().parse().expect("Please enter a valid floating-point number");

    // Create an Operation enum instance
    let op = match operation {
        1 => Operation::Add(first_value, second_value),
        2 => Operation::Subtract(first_value, second_value),
        3 => Operation::Multiply(first_value, second_value),
        4 => {
            // Handle division by zero
            if second_value == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            Operation::Divide(first_value, second_value)
        }
        _ => {
            println!("Error: Invalid operation. Please choose a number between 1 and 4.");
            return;
        }
    };

    // Call the calculate function with the created Operation enum instance
    println!("Result: {}", op.calculate());
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

impl Operation {
    fn calculate(&self) -> f64 {
        match self {
            Operation::Add(x, y) => x + y,
            Operation::Subtract(x, y) => x - y,
            Operation::Multiply(x, y) => x * y,
            Operation::Divide(x, y) => x / y,
        }
    }
}
