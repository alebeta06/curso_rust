use std::rc::Rc;

fn main() {
    let palabra: Rc<String> = Rc::new(String::from("Hola"));

    let palabra2: Rc<String> = Rc::clone(&palabra);
    let palabra3: Rc<String> = Rc::clone(&palabra);

    println!("{}", palabra);
    println!("{}", palabra2);
    println!("{}", palabra3);
    println!("Contador de referencias: {}", Rc::strong_count(&palabra));

}
