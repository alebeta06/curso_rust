# Lección 3 - El mago Archibaldo y la Biblioteca Encantada 🧙‍♂📚

## 🧪 Descripción del problema:

### 🧪 EJERCICIO: EL MAGO ARCHIBALDO Y LA BIBLIOTECA ENCANTADA 🧙‍♂📚

El mago Archibaldo tiene una colección de libros mágicos, runas y hechizos. Quiere compartirlos con sus aprendices, pero debe aplicar las reglas de RUST para evitar errores como libros duplicados incorrectos, hechizos que desaparecen antes de tiempo o conflictos por edición simultánea.

📝 TU MISIÓN:
Corrige el código usando lo que has aprendido sobre: ownership, borrowing (inmutable y mutable), clone, copy, const, static y lifetime.

```
// HECHIZO CONSTANTE QUE NO CAMBIA
const HECHIZO_DEL_DIA: &str = "Expecto Patronum";

// HECHIZO GLOBAL DISPONIBLE SIEMPRE
static MENSAJE_GLOBAL: &str = "Bienvenido a la biblioteca mágica";

fn main() {
    println!("{}", HECHIZO_DEL_DIA);
    println!("{}", MENSAJE_GLOBAL);

    let libro = String::from("Libro de los Elementos");

    // ❌ EL MAGO PRESTA EL LIBRO A DOS APRENDICES Y UNO LO EDITA
    let referencia1 = &libro;
    let referencia2 = &libro;
    let referencia3 = &mut libro;

    println!("{}, {}, {}", referencia1, referencia2, referencia3);

    // ✅ RUNA QUE SE COPIA AUTOMÁTICAMENTE
    let runa = 7;
    usar_runa(runa);
    println!("La runa sigue con Archibaldo: {}", runa);

    // ❌ EL MAGO ENTREGA EL LIBRO PERO LUEGO QUIERE USARLO
    entregar(libro);
    println!("El libro aún está con Archibaldo: {}", libro);

    // ✅ SE HACE UNA COPIA DEL GRIMORIO ANTES DE ENTREGARLO
    let grimorio = String::from("Grimorio Oscuro");
    entregar(grimorio.clone());
    println!("Archibaldo aún tiene el grimorio: {}", grimorio);

    // ❌ ERROR POR FALTA DE LIFETIME EN LA FUNCIÓN
    let uno = String::from("Fuego");
    let dos = String::from("Fuego y Hielo");
    let resultado = mas_largo(&uno, &dos);
    println!("El libro más largo es: {}", resultado);
}

fn usar_runa(r: i32) {
    println!("La runa usada es: {}", r);
}

fn entregar(libro: String) {
    println!("Se ha entregado el libro: {}", libro);
}

fn mas_largo(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## 🧠 OBJETIVOS DEL EJERCICIO:

 ✅ 1. Arreglar errores de préstamo mutable e inmutable
 
 ✅ 2. Usar .clone() cuando quieras conservar ownership
 
 ✅ 3. Aprovechar Copy con tipos simples como i32
 
 ✅ 4. Corregir el error de lifetime en mas_largo
 
 ✅ 5. Entender cuándo usar const y static

# 🧠 Explicación paso a paso

| Problema original                        | Corrección aplicada                                       | Explicación                                                                 |
|:------------------------------------------|:----------------------------------------------------------|:----------------------------------------------------------------------------|
| Error de préstamo mut/inmut               | Usamos primero referencias inmutables, después la mutable | En Rust no puedes tener mutables e inmutables activas a la vez               |
| Ownership movido sin querer (libro)       | Hicimos una copia antes de pasar                           | `.clone()` permite que el original siga siendo válido                       |
| Error de lifetime en `mas_largo`           | Añadimos lifetime explícito `'a`                           | Indica que las referencias devueltas viven al menos tanto como las entradas |
| Tipos simples (`i32`)                     | Se copian automáticamente, no pasa nada                   | `i32`, `bool`, etc., implementan `Copy` por defecto                         |
| Constantes y static                       | Usadas correctamente                                       | Constantes (`const`) y datos globales (`static`) están bien utilizados       |

## 🧙‍♂📚 Resumen visual

✅ `&` → Presta sin mover la propiedad (borrow inmutable)  
✅ `&mut` → Presta permitiendo modificar (borrow mutable)  
✅ `.clone()` → Copia profunda (para tipos que no son `Copy`)  
✅ `const` → Valor fijo en tiempo de compilación  
✅ `static` → Valor global que vive toda la ejecución  
✅ Lifetimes `'a` → Controlan que las referencias sean válidas

## Salida por Consola
```
:~/curso_rust/leccion_3$ cargo run
Expecto Patronum
Bienvenido a la biblioteca mágica
Libro de los Elementos, Libro de los Elementos
Libro de los Elementos - Versión Mejorada
La runa usada es: 7
La runa sigue con Archibaldo: 7
Se ha entregado el libro: Libro de Alquimia
Se ha entregado el libro: Grimorio Oscuro
Archibaldo aún tiene el grimorio: Grimorio Oscuro
El libro más largo es: Fuego y Hielo
```

