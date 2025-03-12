fn main() {
    println!("Welcome to the Basic Calculator!");

    loop {
        let mut input = String::new();
        println!("Please enter a calculation (e.g., 2 + 2) or type 'exit' to quit:");

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            println!("Invalid input. Please enter a calculation in the format 'number operator number'.");
            continue;
        }

        let num1: f64 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number: {}", parts[0]);
                continue;
            }
        };

        let operator = parts[1];
        let num2: f64 = match parts[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number: {}", parts[2]);
                continue;
            }
        };

        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Error: Division by zero.");
                    continue;
                }
                num1 / num2
            }
            _ => {
                println!("Invalid operator: {}", operator);
                continue;
            }
        };

        println!("Result: {}", result);
    }

    println!("Thank you for using the calculator!");
}