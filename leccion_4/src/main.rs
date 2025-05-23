use std::fs; // Importamos para leer archivos

fn main() {
    // Primera división: usamos Option porque dividir por 0 da None
    let resultado: Option<f64> = divide(10.0, 0.0);
    match resultado {
        Some(resultado) => println!("El resultado es: {}", resultado),
        None => println!("Error: No se puede División por cero"),
    }

    // Trabajamos con Option
    let valor: Option<i8> = None;

    // Verificamos si hay un valor
    if valor.is_some() {
        println!("El valor es: {}", valor.unwrap());
    }
    
    // Verificamos si es None y ponemos valor por defecto
    if valor.is_none() {
        println!("El valor es: {}", valor.unwrap_or(0));
    }

    // Usamos .map() para transformar el valor
    let numero: Option<i8> = Some(10);
    let doble: Option<i8> = numero.map(|n| n * 2);
    println!("El doble es: {}", doble.unwrap_or(0));

    // Usamos .and_then() para encadenar operaciones
    let resultado: Option<i8> = numero.and_then(dividir_entre_dos);
    println!("El resultado es: {}", resultado.unwrap_or(0));

    // Buscamos un elemento en una lista
    let lista_numero: Vec<i8> = vec![1, 2, 3, 4, 5];
    match buscar_elemento(lista_numero, 10) {
        Some(elemento) => println!("El elemento se encuentra en la posición: {}", elemento),
        None => println!("El elemento no se encuentra en la lista"),
    }

    // Trabajamos con Result
    let resultado: Result<i32, &str> = Err("no hay dato");

    if resultado.is_ok() {
        println!("El resultado es: {}", resultado.unwrap());
    }

    if resultado.is_err() {
        println!("hubo un Error: {}", resultado.unwrap_err());
    }

    // División segura usando Result
    match dividir(10.0, 2.0) {
        Ok(resultado) => println!("El resultado es: {}", resultado),
        Err(error) => println!("hubo un error: {}", error),
    }

    // Lectura segura de archivo
    let contenido = fs::read_to_string("datos.txt");
    match contenido {
        Ok(contenido) => println!("El contenido del archivo es: {}", contenido),
        Err(error) => println!("hubo un Error: {}", error),
    }
}

// División segura que devuelve Result
fn dividir(dividendo: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("no se puede Dividir por cero"))
    } else {
        Ok(dividendo / divisor)
    }
}

// Buscar un elemento en una lista y devolverlo si existe
fn buscar_elemento(lista: Vec<i8>, elemento: usize) -> Option<i8> {
    if elemento < lista.len() {
       Some(lista[elemento])
    } else {
        None
    }
}

// División que devuelve Option
fn divide(dividendo: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None
    } else {
        Some(dividendo / divisor)
    }
}

// Dividir un número entero si es par
fn dividir_entre_dos(numero: i8) -> Option<i8> {
    if numero % 2 == 0 {
        Some(numero / 2)
    } else {
        None
    }
}
