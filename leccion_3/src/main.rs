fn main() {
    let palabra: String = String::from("Hola");
    let palabra2: String = String::from("Mundo");

    child(&palabra2);

    println!("{}", palabra);
    println!("{}", palabra2);
}

fn child(p: &String) {
    println!("{}", p);
}