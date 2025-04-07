use std::io; // Para leer entrada del usuario

// -------------------------
// EJERCICIO 1
// Programa que acepta un número del 1 al 7 y muestra el día de la semana
// -------------------------
fn mostrar_dia(numero: u8) {
    match numero {
        1 => println!("Lunes"),
        2 => println!("Martes"),
        3 => println!("Miércoles"),
        4 => println!("Jueves"),
        5 => println!("Viernes"),
        6 => println!("Sábado"),
        7 => println!("Domingo"),
        _ => println!("Número inválido. Debe ser del 1 al 7."),
    }
}

// -------------------------
// FUNCIÓN PRINCIPAL
// -------------------------
fn main() {
    // -------------------------
    // Ejecutamos Ejercicio 1
    println!("--- EJERCICIO 1 ---");
    mostrar_dia(3); // Puedes cambiar el número del 1 al 7
}