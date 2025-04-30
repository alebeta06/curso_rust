![L4](https://github.com/user-attachments/assets/ddcacb94-56fb-4831-bf2d-214dcc61b648)

# Lección 4 – Gestión de Errores en Rust 🦀

Proyecto del curso de Rust - Lección 4: Trabajando con `Option`, `Result`, manejo de errores y organización de módulos.

---

## 📁 Estructura del proyecto

```bash
~/curso_rust/leccion_4$ tree
├── Cargo.lock
├── Cargo.toml
├── datos.txt
├── src
│   ├── biblioteca.rs
│   ├── bin
│   │   └── ejercicios.rs
│   ├── ejercicios_de_repaso.rs
│   ├── lib.rs
│   └── main.rs
└── target
```
- `src/main.rs` → Código principal de la lección 4 (ejemplos de clase).

- `src/ejercicios_de_repaso.rs` → Archivo donde se resolvieron los primeros ejercicios de repaso.

- `src/biblioteca.rs` → Archivo que contiene el ejercicio 3 (Biblioteca secreta).

- `src/bin/ejercicios.rs` → Binario separado para ejecutar solo los ejercicios de repaso.

- `src/lib.rs` → Archivo que declara los módulos biblioteca y ejercicios_de_repaso.


## ⚙️ Comandos importantes
Debido a que se agregó en el `Cargo.toml`
```
default-run = "leccion_4"
```
- Los comandos son: 

- Para correr el código de la clase `main.rs`
```
cargo run
```
- Para correr solo los ejercicios de repaso `bin/ejercicios.rs`
```
cargo run --bin ejercicios
```

## 📚 ¿Qué contiene main.rs?
- En `src/main.rs` se encuentran todos los ejemplos prácticos de la Lección 4 sobre:

- Manejo de errores con `Option`
- Manejo de errores con `Result`
- Métodos útiles como `.unwrap(),` `.unwrap_or(),` `.is_some(),` `.map(),` `.and_then()`
- Lectura de archivos con `fs::read_to_string`
- Uso de `match` para gestionar resultados
- Principios básicos de errores seguros en Rust


# 🧪 Ejercicios de Repaso – Lección 4: Rust
## 📝 Ejercicio 1: La división misteriosa
### Enunciado:
Crea una función `dividir_si_puedes(a: i32, b: i32) -> Option<i32>`
Debe devolver `Some(resultado)` si la división es exacta, y `None` si no lo es o si `b` es 0.

- Respuesta / Implementación:
```rust
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
```
### Explicación:

- Si el divisor es 0, se devuelve `None`

- Si la división es exacta (`a % b == 0`), devuelve `Some(resultado)`

- Si no es exacta, también devuelve `None` y muestra un mensaje divertido.

# 📝 Ejercicio 2: Calculadora segura
### Enunciado:

Crea una función `calcular(a: i32, b: i32, operacion: char) -> Result<i32, String>`
Soporta: `'+'`, `'-'`, `'*'`, `'/'`. Error si división entre 0 o operación inválida.

- Respuesta / Implementación:
```rust
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
```
### Explicación:

- Para operaciones conocidas devuelve `Ok(resultado)`

- Si se intenta dividir por 0 o pasa un operador inválido (`_`), devuelve `Err(...)`

# 📝 Ejercicio 3: Biblioteca secreta
### Enunciado:

Crear un archivo `biblioteca.rs` con una función pública `buscar_libro(titulo: &str) -> Option<String>` Si el libro está en una lista interna constante privada, devuelve el autor.

- Respuesta / Implementación:
```rust
const LIBROS: [(&str, &str); 3] = [
    ("El Señor de los Anillos", "J.R.R. Tolkien"),
    ("Harry Potter", "J.K. Rowling"),
    ("Cien Años de Soledad", "Gabriel García Márquez"),
];

pub fn buscar_libro(titulo: &str) -> Option<String> {
    for (libro, autor) in LIBROS.iter() {
        if *libro == titulo {
            return Some(autor.to_string());
        }
    }
    None
}
```
### Explicación:

- Se compara el título buscado con la constante `LIBROS`

- Si existe, se devuelve el autor; si no, `None`

# 📝 Ejercicio 4: Encadenando magias
### Enunciado:

Tienes dos funciones `abrir_cofre()` y `leer_mensaje(mensaje)`, usa `.and_then()` para obtener el mensaje solo si todo va bien.

Respuesta / Implementación:
```rust
pub fn abrir_cofre() -> Result<String, String> {
    Ok(String::from("Tesoro encontrado!"))
}

pub fn leer_mensaje(mensaje: String) -> Result<String, String> {
    Ok(format!("El mensaje es: {}", mensaje))
}
```
Uso con `.and_then()`
```rust
let mensaje_final = abrir_cofre().and_then(leer_mensaje);
match mensaje_final {
    Ok(mensaje) => println!("{}", mensaje),
    Err(error) => println!("Error: {}", error),
}
```
### Explicación:

- `.and_then()` solo ejecuta `leer_mensaje` si `abrir_cofre` fue exitoso (`Ok`).

# 📝 Ejercicio 5: El adivinador de palabras
### Enunciado:

Crear una función que devuelva una palabra opcional `(Option<String>)` y luego usar `.map()` para contar las letras.

Respuesta / Implementación:
```rust
pub fn obtener_palabra() -> Option<String> {
    Some(String::from("magia"))
}
```
Uso con `.map():`
```rust
let palabra = obtener_palabra();
let letras = palabra.map(|p| p.len());
match letras {
    Some(cantidad) => println!("La palabra tiene {} letras.", cantidad),
    None => println!("No se encontró palabra."),
}
```
### Explicación:

- `.map()` transforma el contenido del `Option` si existe (`Some`).

- Si hay palabra, cuenta las letras; si no, da un mensaje.

## Salisa con Consola de `main.rs`
```bash
~/curso_rust/leccion_4$ cargo run

Error: No se puede División por cero
El valor es: 0
El doble es: 20
El resultado es: 5
El elemento no se encuentra en la lista
hubo un Error: no hay dato
El resultado es: 5
El contenido del archivo es: Esto es un contenido de prueba
```
## Salisa con Consola de `ejercicios`
```bash
~/curso_rust/leccion_4$ cargo run --bin ejercicios

=== Ejecutando solo los ejercicios de repaso ===
=== Ejercicio 1 ===
Resultado: 5
=== Ejercicio 2 ===
Error: ¡No se puede dividir entre cero!
=== Ejercicio 3 ===
Autor encontrado: J.K. Rowling
=== Ejercicio 4 ===
El mensaje es: Tesoro encontrado!
=== Ejercicio 5 ===
La palabra tiene 5 letras.
```

## 🚀 Conclusión
- Aprendimos a trabajar correctamente con `Option` y `Result`

- Organizamos el proyecto usando **lib.rs** y **múltiples binarios** (`main.rs` y `bin/ejercicios.rs`).

- Dominamos `.map()`, `.and_then()`, `unwrap_or()`, y el manejo seguro de errores.

¡Seguimos avanzando en Rust! 🦀✨
