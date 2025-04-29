// Ejercicio 1: La división misteriosa
pub fn dividir_si_puedes(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        println!("¡No se puede dividir entre cero! 🧙‍♂️");
        None
    } else if a % b == 0 {
        Some(a / b)
    } else {
        println!("¡La división no es exacta! 🧹");
        None
    }
}

// Ejercicio 2: Calculadora segura
pub fn calcular(a: i32, b: i32, operacion: char) -> Result<i32, String> {
    match operacion {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0 {
                Err(String::from("¡No se puede dividir entre cero!"))
            } else {
                Ok(a / b)
            }
        }
        _ => Err(String::from("Operación no soportada")),
    }
}

// Ejercicio 4: Encadenando magias
pub fn abrir_cofre() -> Result<String, String> {
    Ok(String::from("Tesoro encontrado!"))
    // Puedes probar con Err("Cofre vacío".to_string()) si quieres ver error
}

pub fn leer_mensaje(mensaje: String) -> Result<String, String> {
    Ok(format!("El mensaje es: {}", mensaje))
    // También podrías simular errores: Err("Mensaje ilegible".to_string())
}

// Ejercicio 5: El adivinador de palabras
pub fn obtener_palabra() -> Option<String> {
    Some(String::from("magia"))
    // Puedes probar con None también
}
