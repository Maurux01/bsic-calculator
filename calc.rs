use std::io;

fn main() {
    println!("Calculadora Básica en Rust");
    println!("---------------------------");

    loop {
        println!("\nOperaciones disponibles:");
        println!("1. Suma (+)");
        println!("2. Resta (-)");
        println!("3. Multiplicación (*)");
        println!("4. División (/)");
        println!("5. Salir");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Error al leer la entrada");

        let choice = choice.trim();

        if choice == "5" || choice.eq_ignore_ascii_case("salir") {
            println!("¡Hasta luego!");
            break;
        }

        println!("Ingrese el primer número:");
        let num1: f64 = read_number();
        
        println!("Ingrese el segundo número:");
        let num2: f64 = read_number();

        match choice {
            "1" | "+" => println!("Resultado: {}", num1 + num2),
            "2" | "-" => println!("Resultado: {}", num1 - num2),
            "3" | "*" => println!("Resultado: {}", num1 * num2),
            "4" | "/" => {
                if num2 == 0.0 {
                    println!("Error: No se puede dividir por cero!");
                } else {
                    println!("Resultado: {}", num1 / num2);
                }
            },
            _ => println!("Opción no válida!"),
        }
    }
}

fn read_number() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Por favor ingrese un número válido:"),
        }
    }
}
