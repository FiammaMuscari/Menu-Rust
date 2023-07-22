mod animal;
mod utils;

use animal::{Animal, Gato, Perro};
use std::time::SystemTime;
use std::vec::Vec;

fn main() {
    let mut animales_registrados: Vec<Box<dyn Animal>> = Vec::new();

    loop {
        println!("Guardería de animalitos");
        println!("1. Registrar Perro");
        println!("2. Registrar Gato");
        println!("3. Mostrar animales en la vaterinaria");
        println!("4. Retirar animal");
        println!("5. Salir");

        let choice = utils::get_input_u32();

        match choice {
            1 => registrar_perro(&mut animales_registrados),
            2 => registrar_gato(&mut animales_registrados),
            3 => mostrar_animales_registrados(&animales_registrados),
            4 => eliminar_animal(&mut animales_registrados),
            5 => {
                println!("¡Hasta luego!");
                break;
            }
            _ => println!("Elige un número válido."),
        }
    }
}

fn registrar_perro(animales_registrados: &mut Vec<Box<dyn Animal>>) {
    let nombre = utils::get_input_string("Nombre del perro:");
    let ojos = utils::elegir_opcion(
        "Color",
        &["amarillo", "blanco", "gris", "marrón", "negro", "otro"],
    );
    let pelaje = utils::elegir_opcion(
        "Pelaje del perro:",
        &["rulo", "largo", "corto", "duro", "sin", "otro"],
    );
    let edad = utils::get_input_u8();

    // Obtener la fecha actual en formato dd/mm/yyyy
    let fecha_guardado = get_current_date();

    let perro = Box::new(Perro {
        color_ojos: ojos,
        pelaje,
        edad,
        fecha_guardado,
        nombre,
    });
    animales_registrados.push(perro);
    println!("Perro registrado correctamente.");
    println!("╔══════════════════╗");
    println!("║           .-._   ║");
    println!("║         ( ) ^ )o ║");
    println!("║   (/ ____/ /''   ║");
    println!("║   (         )    ║");
    println!("║   //||~~~~~||    ║");
    println!("╚══════════════════╝");
}

fn registrar_gato(animales_registrados: &mut Vec<Box<dyn Animal>>) {
    let nombre = utils::get_input_string("Nombre del gato:");
    let ojos = utils::elegir_opcion("Gato", &["naranja", "gris", "negro", "marmolado", "otro"]);
    let pelaje = utils::elegir_opcion(
        "Pelaje del gato:",
        &["naranja", "gris", "negro", "marmolado", "otro"],
    );
    let edad = utils::get_input_u8();

    // Obtener la fecha actual en formato dd/mm/yyyy
    let fecha_guardado = get_current_date();

    let gato = Box::new(Gato {
        color_ojos: ojos,
        pelaje,
        edad,
        fecha_guardado,
        nombre,
    });
    animales_registrados.push(gato);
    println!("Gato registrado correctamente.");
    println!("╔───▄▀▀▀▄▄▄▄▄▄▄▀▀▀▄───╗");
    println!("║───█▒▒░░░░░░░░░▒▒█───║");
    println!("║────█░░█░░░░░█░░█────║");
    println!("║─▄▄──█░░░▀█▀░░░█──▄▄─║");
    println!("║█░░█─▀▄░░░░░░░▄▀─█░░█║");
    println!("╚═════════════════════╝");
}

// Función para obtener la fecha actual en formato dd/mm/yyyy
fn get_current_date() -> String {
    let now = SystemTime::now();
    let local = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let seconds = local.as_secs();
    let days = seconds / (60 * 60 * 24);
    let years = days / 365;
    let remainder_days = days % 365;
    format!(
        "{:02}/{:02}/{:04}",
        remainder_days / 31 + 1,
        remainder_days % 31 + 1,
        1970 + years
    )
}
fn mostrar_animales_registrados(animales_registrados: &Vec<Box<dyn Animal>>) {
    if animales_registrados.is_empty() {
        println!("No se han registrado animales todavía.");
    } else {
        println!("Animales registrados:");
        for (index, animal) in animales_registrados.iter().enumerate() {
            println!("{}: {}", index + 1, animal.to_string());
        }
    }
}

fn eliminar_animal(animales_registrados: &mut Vec<Box<dyn Animal>>) {
    if animales_registrados.is_empty() {
        println!("No hay animales registrados para eliminar.");
    } else {
        println!("Selecciona el número del animal a eliminar:");
        mostrar_animales_registrados(&animales_registrados);

        let choice = utils::get_input_u32();

        if choice <= animales_registrados.len() as u32 {
            animales_registrados.remove((choice - 1) as usize);
            println!("Animal eliminado correctamente.");
        } else {
            println!("Opción inválida. Por favor, elige un número válido.");
        }
    }
}
