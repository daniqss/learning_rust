# Ownership (Propiedad):
## Reglas de ownership:
Cada valor en Rust tiene una única variable que es su propietaria.
Cuando la variable propietaria sale de alcance, el valor se libera automáticamente.


```rust
fn main() {
    let s = String::from("¡Hola!");  // s es propietario de la cadena

    // La variable s es dueña de la cadena "¡Hola!"
}  // Aquí, s sale de alcance y la cadena se libera automáticamente
```

# Borrowing (Préstamo):
## Referencias & préstamos:
Permite que variables retengan prestada la propiedad de un valor.
Puede haber múltiples referencias prestadas a un recurso, pero solo puede existir una referencia mutable a la vez.

```rust
fn main() {
    let s1 = String::from("Hola");
    let len = calculate_length(&s1);  // Se pasa una referencia a la función

    println!("La longitud de '{}' es {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {  // Se recibe una referencia
    s.len()
}
```