fn main() {
    let mut palabra: String = String::from("Hola");
    let mut palabra2: String = String::from("Mundo");

    palabra = child(palabra);

    println!("La palabra es: {}", palabra);
    println!("La palabra es: {}", palabra2);
}

fn child(palabra: String) -> String {
    println!("La palabra es: {}", palabra);
    palabra
}