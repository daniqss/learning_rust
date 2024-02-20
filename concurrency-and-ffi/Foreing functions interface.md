# Foreing functions interface

## Create them in Rust
To create a foreign function in Rust, you need to use the `extern` keyword. The `extern` keyword is used to define a foreign function. The `extern` keyword is followed by the `C` calling convention. The `C` calling convention is used to define the function signature and the function body.

You hace to use the `no_mangle` attribute to prevent the Rust compiler from mangling the function name. The `no_mangle` attribute is used to define the function name.

```rust
#[no_mangle]
pub extern test_function() {
    println!("Hello from a foreign function!");
}
```
The proyect must be a library, so you have to use the `crate-type` attribute in the `Cargo.toml` file.

```toml
[lib]
name = "concurrency_and_ffi"
crate-type = ["dylib"]
```

To compile then you have to use
```bash
cargo build --release
```
The library binary will be in the `target/release` directory.

## Use them in Python

To use the Rust library in Python, you have to use the `ctypes` module. The `ctypes` module is used to load the C libraries and call the foreign function.

```python
from ctypes import cdll
# In Unix dynamic libraries are compiled to .so files
# In Windows nonetheless, they are compiled to .dll files
lib = cdll.LoadLibrary('./target/release/your_lib.so')

lib.test_function()
```

The result will be
```bash
Hello from a foreign function!
```
