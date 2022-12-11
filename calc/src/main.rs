use std::io;

fn main() {
    // Print a welcome message
    println!("Welcome to the Rust Calculator!");

    loop {
        // Read the user's input
        print!("Enter an expression: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read input");

        // Evaluate the expression and print the result
        let result = evaluate(&input);
        println!("Result: {}", result);
    }
}

fn evaluate(input: &str) -> f64 {
    // TODO: Parse and evaluate the expression
    let mut result = 0.0;

    // Split the input into tokens
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Parse and evaluate each token
    let mut operator = '+';
    for token in tokens {
        match token {
            "+" | "-" | "*" | "/" => {
                operator = token.chars().next().unwrap();
            },
            _ => {
                let operand: f64 = token.parse().unwrap();
                match operator {
                    '+' => result += operand,
                    '-' => result -= operand,
                    '*' => result *= operand,
                    '/' => result /= operand,
                    _ => (),
                }
            }
        }
    }

    result
}
