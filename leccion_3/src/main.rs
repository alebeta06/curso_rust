fn main() {
   let x: &str = "Hola";
   let y: &str = "Mundo";

   let z: &str = larga(x, y);
   println!("La cadena más larga es: {}", z);
}

fn larga<'amigo>(x: &'amigo str, y: &'amigo str) -> &'amigo str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} 
