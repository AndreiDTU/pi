use std::{sync::atomic::Ordering::Relaxed, thread, time::{Duration, Instant}};

use atomic_float::AtomicF64;

static INTERVALS: u32 = 10000000;
static NUM_THREADS: u32 = 7;
static PI25DT: f64 = 3.141592653589793238462643;

fn main() {
    let iterations = 100;
    let mut total: Duration = Duration::from_millis(0);
    for i in 0..iterations {
        let now = Instant::now();
        let sum = &AtomicF64::new(0.0);
        let step = INTERVALS / NUM_THREADS;
        let dx = 1.0 / INTERVALS as f64;
        
        thread::scope(|s| {
            for i in 0..NUM_THREADS {
                s.spawn(move || sum.fetch_add(par_sum(i*step + 1, (i+1)*step + 1, dx), Relaxed));
            }
        });
        let pi = dx * sum.load(Relaxed) + par_sum(NUM_THREADS*step + 1, NUM_THREADS*step + 1 + INTERVALS % NUM_THREADS, dx);
    
        println!("Iteration {i}");
        println!("NUM_THREADS = {:?}", NUM_THREADS);
        println!("pi_approx = {:.24?}", pi);
        println!("pi_real = {:.24?}", PI25DT);
        println!("error = {:.24?}", PI25DT-pi);
        let end = now.elapsed();
        total += end;
        println!("elapsed = {:?}", end);
        println!();
    }
    println!("Average time: {:?}", total / iterations)
}

fn par_sum(start_interval: u32, end_interval: u32, dx: f64) -> f64 {
    let mut sum = 0.0;
    for i in start_interval..end_interval  {
        let x = dx * (i as f64 - 0.5);
        let f = 4.0 / (1.0 + x*x);
        sum += f;
    }
    sum
}
