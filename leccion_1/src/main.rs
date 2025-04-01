//Ejercicio 1: Hola Mundo
fn main() {
    println!("Hello, Rust!");

    //Ejercicio 2: Variables y Mutabilidad
    let x = 10;
    // x = 20; // Esto causará un error porque x es inmutable

    let mut y = 5;
    y = 15; // Esto es válido porque y es mutable

    const PI: f64 = 3.1416;
    println!("x: {}, y: {}, PI: {}", x, y, PI);

    // Ejercicio 3: Tipos de Datos y Tuplas
    let datos: (i32, f64, char) = (42, 3.14, 'R');
    println!("Entero: {}, Flotante: {}, Carácter: {}", datos.0, datos.1, datos.2);

    // Ejercicio 4: Conversión de Tipos (Casting)
    let numero: i32 = 25;
    let numero_flotante: f64 = numero as f64;
    println!("Número entero: {}, Número flotante: {}", numero, numero_flotante);
}
