use std::io; // Para leer entrada del usuario

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

// EJERCICIO 2
// Tabla de multiplicar de un número
// -------------------------
// Función que recibe un número y muestra su tabla de multiplicar
fn tabla_multiplicar(numero: u32) {
    println!("Tabla de multiplicar del {}:", numero);

    // Usamos un bucle for para multiplicar de 1 a 10
    for i in 1..=10 {
        println!("{} x {} = {}", numero, i, numero * i);
    }
}

// EJERCICIO 3
// Función que determina si un número es par o impar
fn par_o_impar(numero: i32) -> &'static str {
    // Si el resto de dividir el número entre 2 es 0 => par
    if numero % 2 == 0 {
        "par"
    } else {
        "impar"
    }
}

// EJERCICIO 4
// Estructura Persona con Enum Color
// -------------------------

// Definimos un enum con colores posibles
enum Color {
    Rojo,
    Verde,
    Amarillo,
}

// Estructura que almacena datos de una persona
struct Persona {
    nombre: String,
    edad: u32,
    color_favorito: Color,
}

fn imprimir_persona(persona: &Persona) {
    println!("Nombre: {}", persona.nombre);
    println!("Edad: {}", persona.edad);

    // Para imprimir el enum, usamos match
    match persona.color_favorito {
        Color::Rojo => println!("Color favorito: Rojo"),
        Color::Verde => println!("Color favorito: Verde"),
        Color::Amarillo => println!("Color favorito: Amarillo"),
    }
}

// FUNCIÓN PRINCIPAL
fn main() {
    // -------------------------
    // Ejecutamos Ejercicio 1
    println!("--- EJERCICIO 1 ---");
    mostrar_dia(3); // Puedes cambiar el número del 1 al 7

    // -------------------------
    // Ejecutamos Ejercicio 2
    println!("\n--- EJERCICIO 2 ---");
    tabla_multiplicar(5); // Puedes cambiar el número

    // -------------------------
    // Ejecutamos Ejercicio 3
    println!("\n--- EJERCICIO 3 ---");
    let numero = 8;
    println!("El número {} es {}", numero, par_o_impar(numero));

    // Ejecutamos Ejercicio 4
    println!("\n--- EJERCICIO 4 ---");
    let persona = Persona {
        nombre: String::from("Alejandro"),
        edad: 37,
        color_favorito: Color::Verde,
    };
    imprimir_persona(&persona);
}