use std::thread::JoinHandle;
use std::{sync::Mutex, thread};
use std::time::{Instant, Duration};

fn main() {
    // let handler: Vec<_> = (0..10).map(|_| {
    //     let mut n = Mutex::new(0);
    //     thread::spawn(|| {
    //         for _ in 0..500000000 {
    //             n.lock().unwrap();
    //             *n += 1;
    //         }
    //         *n
    //     })
    // }).collect();

    let handler: Vec<JoinHandle<i32>> = (0..1).map(|_| {
        thread::spawn(move || {
            let n = Mutex::new(0);
            for _ in 0..50000000 {
                let mut n = n.lock().unwrap();
                *n += 1;
                // El bloqueo se libera automáticamente cuando n sale del ámbito.
            }
            n.into_inner().unwrap()  // Devuelve el valor interior del Mutex después de que todos los incrementos hayan terminado.
        })
    }).collect();

    for h in handler {
        match h.join() {
            Ok(n) => {println!("Thread finished with count: {}", n);},
            Err(e) => println!("Thread failed: {:?}", e),
        }
    }

}

fn count(count: u32, threads: u32) -> Vec<JoinHandle<u32>> {
    let handler: Vec<_> = (0..threads).map(|_| {
        thread::spawn(move || {
            let n = Mutex::new(0);
            for _ in 0..count {
                let mut n = n.lock().unwrap();
                *n += 1;
                // El bloqueo se libera automáticamente cuando n sale del ámbito.
            }
            n.into_inner().unwrap()  // Devuelve el valor interior del Mutex después de que todos los incrementos hayan terminado.
        })
    }).collect();
    handler
}

fn time_function(function: fn(u32, u32) -> Vec<JoinHandle<u32>>) -> Duration {
    let start = Instant::now();
    let handler = function(10, 50000000);
    let end = Instant::now();
    
    for h in handler {
        match h.join() {
            Ok(n) => {println!("Thread finished with count: {}", n);},
            Err(e) => println!("Thread failed: {:?}", e),
        }
    }

    end.duration_since(start)

}