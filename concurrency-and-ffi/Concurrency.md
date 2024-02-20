# Concurrency in Rust

## Threads

To create threads you can use the `std::thread` module. The `spawn` function is used to create a new thread. The `spawn` function takes a closure as an argument. The closure is the code that will be executed in the new thread.

```rust
for _ in 0..10 {
    std::thread::spawn(|| {
        println!("Hello from a thread!");
    });
}
```

To manage threads, we use handlers. We create a Vec of JoinHandles, which are the handlers for the threads. We then use the `join` method to wait for the threads to finish.

```rust
    use std::thread;
    use std::thread::JoinHandle;

    let count:u32 = 10;

    let handler: Vec<JoinHandle<u32>> = (0..threads).map(|count| {
        thread::spawn(move || {
            for n in 0..count {
                println!("Hello from a thread {} time!", n);
            }
        })
    }).collect();

    for h in handler {
        match handle.join() {
            Ok(_) => println!("Thread finished"),
            Err(_) => println!("Thread panicked"),
        }
    }
```

## Mutex and Arc

To share data between threads, we use the `std::sync` module. The `Mutex` type is used to lock data and the `Arc` type is used to share data between threads.

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    std::thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
}
```

