# Learning Rust

Little programs to learn the basic concepts of the Rust programming language

## Useful sites
* (https://doc.rust-lang.org/stable/book/)
* (https://tourofrust.com/TOC_en.html)
* (https://doc.rust-lang.org/rust-by-example/)
* (https://midu.dev/rust-para-desarrolladores-javascript/)

## Compilation

With `cargo init`  we create a proyect in which we can use `cargo run` to compile and run our program. The binaty is saved in `target/debug/<file_name>`.

To compile just one file we can use:
```bash
rustc <nombre_del_fichero>.rs
```

## Variables

Variables are declared using the **let** keyword
Types are infered by the compiler but they can be added to the variable declaration
Variables are inmutable by default. **mut** keyword can be used to make a variable mutable

```rust
fn main () {
	let x = 69;
	let y: f64 = 2.71;
	let mut z: i64 = 434243234545;
}
```

In Rust, variable names are written in **snake_case**

### Types
* bool -> true, false
* unsigned integer -> u8, u16, u32, u64, u128
* signed integer -> i8, i16, i32, i64, i128
* floating -> f32, f64


### Arrays

Fixed lenght collection of elements all of the same type. C indexation style.

```rust
fn main() {
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);
}
```

```bash
[1, 2, 3]
2
```

To make dynamic arrays use Vectors

## Functions

Always in snake_case. If you just want to return an expression, you can drop the `return` keyword and the semicolon at the end, as we did in the _subtract_ function.

```rust
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn main() {
    println!("42 + 13 = {}", add(42, 13));
    println!("42 - 13 = {}", subtract(42, 13));
}
```

If no return type is specified for a function, it returns an empty tuple, also known as a _unit_ like in ocaml.
An empty tuple is represented by `()`.
## Tuples

Tuple elements can be referenced by their index number. (like c structs)

```rust
fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn main() {
    // return a tuple of return values
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // destructure the tuple into two variables names
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
}

```

