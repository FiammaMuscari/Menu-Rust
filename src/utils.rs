use std::io::{self, Write};

pub fn get_input_u32() -> u32 {
    loop {
        let mut input = String::new();
        print!("Ingrese un número: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(choice) => return choice,
            Err(_) => println!("Ingrese un número válido."),
        }
    }
}

pub fn get_input_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_input_u8() -> u8 {
    loop {
        let input = get_input_string("¿Cuántos años tiene?: ");
        match input.parse::<u8>() {
            Ok(age) => return age,
            Err(_) => println!("Ingrese una edad válida, un número."),
        }
    }
}

pub fn elegir_opcion(prompt: &str, opciones: &[&str]) -> String {
    println!("{}:", prompt);
    for (index, opcion) in opciones.iter().enumerate() {
        println!("{}. {}", index + 1, opcion);
    }

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice: usize = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Ingrese un número válido.");
                continue;
            }
        };

        if choice >= 1 && choice <= opciones.len() {
            return opciones[choice - 1].to_string();
        } else {
            println!("Ingrese un número válido.");
        }
    }
}
