use std::thread::JoinHandle;
use std::thread;
use std::time::{Instant, Duration};

fn main() {
    let one = study_times(100, 10_000, 1);
    let twelve = study_times(100, 100_000, 12);
    println!("one: {:?} twelve: {:?}", one, twelve);

    println!("==================================="); 
    
    let one = study_times(100, 100_000, 1);
    let twelve = study_times(100, 1_000_000, 12);
    println!("one: {:?} twelve: {:?}", one, twelve);

}

fn counter(_count: u32, threads: u32) -> Vec<JoinHandle<u32>> {
    let handler: Vec<_> = (0..threads).map(|_count| {
        thread::spawn(move || {
            let mut n = 0;
            for _ in 0.._count {
                n += 1;
            }
            n
        })
    }).collect();
    handler
}

fn timer(function: fn(u32, u32) -> Vec<JoinHandle<u32>>, count: u32, threads: u32) -> Duration {
    let start = Instant::now();
    let handler = function(count, threads);
    
    for h in handler {
        match h.join() {
            Ok(_) => (),
            Err(e) => println!("Thread failed: {:?}", e),
        }
    }
    let end = Instant::now();
    
    end.duration_since(start)

}

fn study_times (iterations:u32, count: u32, threads: u32) -> Duration {
    let mut total_time = Duration::from_secs(0);

    for i in 0..iterations {
        let counting_time = timer(counter, count, threads);

        let printing_time = if i % 50 == 0 {
            let start_printing = Instant::now();
            println!("====================");
            println!("Iteration: {} of 100", i);
            println!("Time: {:?} with {} threads counting to {}", counting_time.as_micros(), threads, count);
            let end_printing = Instant::now();
            Some(end_printing.duration_since(start_printing))
        }
        else {
            None
        };

        match printing_time {
            Some(t) =>  t + counting_time,
            None => counting_time
        };
        total_time += counting_time;
    }

    let promedy = total_time / iterations;
    println!("====================");
    println!("Promedy time: {:?} with {} threads counting to {}", promedy.as_micros(), threads, count);
    promedy
}
