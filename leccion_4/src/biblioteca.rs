// Lista interna privada de libros y autores
const LIBROS: [(&str, &str); 3] = [
    ("El Señor de los Anillos", "J.R.R. Tolkien"),
    ("Harry Potter", "J.K. Rowling"),
    ("Cien Años de Soledad", "Gabriel García Márquez"),
];

// Función pública para buscar un libro
pub fn buscar_libro(titulo: &str) -> Option<String> {
    for (libro, autor) in LIBROS.iter() {
        if *libro == titulo {
            return Some(autor.to_string());
        }
    }
    None
}
