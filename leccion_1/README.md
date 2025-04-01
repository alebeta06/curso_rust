# Ejercicios de Rust 🦀

Este repositorio contiene cinco ejercicios básicos en Rust para aprender los conceptos fundamentales del lenguaje.

---

## Ejercicio 1: Hola Mundo 🌍

**Objetivo:** Familiarizarse con la estructura básica de un programa en Rust.

### 📌 Instrucciones:

1. Crea un nuevo proyecto en Rust usando `cargo`.
2. Modifica el archivo `main.rs` para imprimir `"Hola, Rust!"`.
3. Compila y ejecuta el programa.

### 🔹 Preguntas de reflexión:

- ¿Qué pasa si olvidas el punto y coma (`;`) después de `println!`?

### ✅ Explicación:

El programa usa la macro `println!` para imprimir texto en la consola. Si olvidamos el `;`, Rust espera que haya más código en la misma línea y arrojará un error de sintaxis.

---

## Ejercicio 2: Variables y Mutabilidad 🔀

**Objetivo:** Comprender la diferencia entre variables inmutables y mutables.

### 📌 Instrucciones:

1. Declara una variable inmutable `x` con el valor `10`.
2. Intenta cambiar su valor a `20` y observa el error del compilador.
3. Declara otra variable `y` pero usando `mut` y cambia su valor.
4. Declara una constante `PI` con el valor `3.1416`.

### 🔹 Preguntas de reflexión:

- ¿Por qué `x` no se puede modificar?
- ¿Qué ventajas tiene usar constantes?

### ✅ Explicación:

- En Rust, las variables son inmutables por defecto. Si intentamos modificar `x`, obtenemos un error de compilación.
- Usamos `mut` para declarar una variable mutable.
- Las constantes (`const`) deben tener un tipo explícito y no pueden cambiar su valor en tiempo de ejecución. Son útiles para mejorar la seguridad y optimizar el rendimiento.

---

## Ejercicio 3: Tipos de Datos y Tuplas 📦

**Objetivo:** Practicar diferentes tipos de datos en Rust.

### 📌 Instrucciones:

1. Crea una tupla que contenga un número entero, un número flotante y un carácter.
2. Accede a cada uno de los valores de la tupla e imprímelos.

### 🔹 Preguntas de reflexión:

- ¿Cómo accederías al tercer elemento de la tupla?

### ✅ Explicación:

Las tuplas permiten almacenar valores de distintos tipos en una misma estructura. Podemos acceder a cada elemento usando `tupla.índice`, donde los índices comienzan desde `0`.

---

## Ejercicio 4: Conversión de Tipos (Casting) 🔄

**Objetivo:** Practicar la conversión de tipos en Rust.

### 📌 Instrucciones:

1. Declara un número entero de tipo `i32`.
2. Convierte ese número a tipo `f64` y guárdalo en una nueva variable.
3. Imprime ambos valores.

### ✅ Explicación:

Rust no realiza conversiones implícitas entre tipos numéricos para evitar errores. Usamos `as` para hacer conversiones explícitas, por ejemplo:

```rust
let numero: i32 = 25;
let numero_flotante: f64 = numero as f64;
```
