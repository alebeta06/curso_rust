// * Importa todo lo público de ese módulo
use leccion_4::ejercicios_de_repaso::*;
use leccion_4::biblioteca::*;


fn main() {
    println!("=== Ejecutando solo los ejercicios de repaso ===");

    println!("=== Ejercicio 1 ===");
    match dividir_si_puedes(10, 2) {
        Some(resultado) => println!("Resultado: {}", resultado),
        None => println!("No se pudo dividir."),
    }

    println!("=== Ejercicio 2 ===");
    match calcular(5, 0, '/') {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(error) => println!("Error: {}", error),
    }

    println!("=== Ejercicio 3 ===");
    match buscar_libro("Harry Potter") {
        Some(autor) => println!("Autor encontrado: {}", autor),
        None => println!("Libro no encontrado."),
    }

    println!("=== Ejercicio 4 ===");
    let mensaje_final = abrir_cofre().and_then(leer_mensaje);
    match mensaje_final {
        Ok(mensaje) => println!("{}", mensaje),
        Err(error) => println!("Error: {}", error),
    }

    println!("=== Ejercicio 5 ===");
    let palabra = obtener_palabra();
    let letras = palabra.map(|p| p.len());
    match letras {
        Some(cantidad) => println!("La palabra tiene {} letras.", cantidad),
        None => println!("No se encontró palabra."),
    }
}
