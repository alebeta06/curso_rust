fn main() {
    let palabra: String = String::from("Hola");
    
    let palabra2: &String = &palabra;
    let palabra3: &String = &palabra;
    let palabra4: &String = &palabra;

    println!("{}", palabra);
    println!("{}", palabra2);
    println!("{}", palabra3);
    println!("{}", palabra4);

    child(&palabra);
}

fn child(p: &String) {
    println!("{}", p);
}
