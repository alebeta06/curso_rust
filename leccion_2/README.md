# Lección 2 - Ejercicios de Rust 🦀

Este proyecto contiene los ejercicios resueltos de la Lección 2 del curso de Rust. Cada ejercicio está implementado en `src/main.rs` y se puede ejecutar todo junto usando `cargo run`.

## Ejercicios y soluciones

### 1. Programa que acepta un número del 1 al 7 y muestra el día de la semana
**Descripción:**  
Se crea una función `mostrar_dia(numero: u8)` que utiliza `match` para mostrar el nombre del día correspondiente al número ingresado.

**Notas:**  
- Si el número está fuera del rango 1-7, muestra un mensaje de error.  
- **Ejemplo de salida:** Si ingresamos `3`, imprime `Miércoles`.

### 2. Programa que muestra la tabla de multiplicar de un número
**Descripción:**  
La función `tabla_multiplicar(numero: u32)` imprime la tabla de multiplicar del número que se le pase, del 1 al 10.

**Notas:**  
- Usa un bucle `for` para recorrer los multiplicadores del 1 al 10.  
- **Ejemplo de salida para 5:**  
```plaintext
5 x 1 = 5
5 x 2 = 10
...
5 x 10 = 50
```

### 3. Función que determina si un número es par o impar
**Descripción:**  
Se implementa la función `par_o_impar(numero: i32) -> &'static str`.

**Notas:**  
- Usa el operador `%` (módulo) para verificar si el número es divisible por 2.  
- Retorna `"par"` si es par, `"impar"` si es impar.  
- **Ejemplo de salida:** `El número 8 es par`

### 4. Programa que define una estructura `Persona` y un enumerado `Color`
**Descripción:**  
Se define:  
- Un `enum Color` con tres variantes: `Rojo`, `Verde` y `Amarillo`.  
- Una estructura `Persona` con los campos `nombre`, `edad` y `color_favorito`.

**Notas:**  
- Se crea un dato de prueba: Alejandro, 37 años, color favorito `Verde`.  
- Se imprime cada campo usando `println!`.  
- El `Color` se imprime usando un `match`.  
- **Advertencia de compilación:** Rust emite un warning porque algunas variantes del `enum` (`Rojo`, `Amarillo`) no se están utilizando todavía.

### 5. Función que convierte una palabra a mayúsculas
**Descripción:**  
Se crea la función `uppercase(palabra: &str) -> String`.

**Notas:**  
- Utiliza el método `to_uppercase()` de Rust para transformar una palabra a mayúsculas.  
- **Ejemplo de salida:** `'hola' en mayúsculas es 'HOLA'`

## Código principal (`main.rs`)
En `src/main.rs`, cada ejercicio se ejecuta en orden dentro de la función `main`, separados claramente con un título.

## Cómo ejecutar el proyecto 🚀
1. Abre una terminal.  
2. Asegúrate de estar dentro de la carpeta del proyecto.  
3. Ejecuta:  
```bash
cargo run
```

