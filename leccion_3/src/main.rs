// HECHIZO CONSTANTE QUE NO CAMBIA
const HECHIZO_DEL_DIA: &str = "Expecto Patronum";

// HECHIZO GLOBAL DISPONIBLE SIEMPRE
static MENSAJE_GLOBAL: &str = "Bienvenido a la biblioteca mágica";

fn main() {
    // Mostramos constantes
    println!("{}", HECHIZO_DEL_DIA);
    println!("{}", MENSAJE_GLOBAL);

    let mut libro = String::from("Libro de los Elementos");
    // Declaramos 'libro' como mut porque lo vamos a modificar

    // ✅ EL MAGO PRESTA EL LIBRO A DOS APRENDICES (referencias inmutables)
    let referencia1 = &libro;
    let referencia2 = &libro;
    println!("{}, {}", referencia1, referencia2);

    // Ahora terminamos de usar referencias inmutables antes de pedir una mutable

    // ✅ Ahora el mago permite una edición (préstamo mutable)
    let referencia3 = &mut libro;
    referencia3.push_str (" - Versión Mejorada");
    println!("{}", referencia3);

    // ✅ RUNA QUE SE COPIA AUTOMÁTICAMENTE (i32 implementa Copy)
    let runa = 7;
    usar_runa(runa); // Pasamos runa por copia
    println!("La runa sigue con Archibaldo: {}", runa);

    // ✅ EL MAGO ENTREGA EL LIBRO (ownership se transfiere)
    let libro_entregado = String::from("Libro de Alquimia");
    entregar(libro_entregado);
    // println!("El libro aún está con Archibaldo: {}", libro_entregado); // No se puede usar libro_entregado después

    // ✅ SE HACE UNA COPIA DEL GRIMORIO ANTES DE ENTREGARLO
    let grimorio = String::from("Grimorio Oscuro");
    entregar(grimorio.clone()); // Se entrega la copia
    println!("Archibaldo aún tiene el grimorio: {}", grimorio);

    // ✅ CORREGIMOS EL ERROR DE LIFETIME EN la función mas_largo
    let uno = String::from("Fuego");
    let dos = String::from("Fuego y Hielo");
    let resultado = mas_largo(&uno, &dos);
    println!("El libro más largo es: {}", resultado);
}

// Función que usa una runa (i32 implementa Copy, no mueve ownership)
fn usar_runa(r: i32) {
    println!("La runa usada es: {}", r);
}

// Función que recibe un libro y toma su propiedad (mueve ownership)
fn entregar(libro: String) {
    println!("Se ha entregado el libro: {}", libro);
}

// ✅ Corregimos el lifetime explícito para evitar errores
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//Salida por consola

//Expecto Patronum
//Bienvenido a la biblioteca mágica
//Libro de los Elementos, Libro de los Elementos
//Libro de los Elementos - Versión Mejorada
//La runa usada es: 7
//La runa sigue con Archibaldo: 7
//Se ha entregado el libro: Libro de Alquimia
//Se ha entregado el libro: Grimorio Oscuro
//Archibaldo aún tiene el grimorio: Grimorio Oscuro
//El libro más largo es: Fuego y Hielo