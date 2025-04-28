fn main() {
    let resultado: Option<f64> = divide(10.0, 0.0);
    match resultado {
        Some(resultado) => println!("El resultado es: {}", resultado),
        None => println!("Error: No se puede División por cero"),
    }

    let valor: Option<i8> = None;

    if valor.is_some() {
        println!("El valor es: {}", valor.unwrap());
    }
    
    if valor.is_none() {
        println!("El valor es: {}", valor.unwrap());
    }
}

fn divide(dividendo: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None
    } else {
        Some(dividendo / divisor)
    }
}
