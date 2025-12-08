// benchmark.rs
// Save as: benchmark.rs
// Run with: cargo run --release

use std::time::Instant;

static mut CONFIG_THRESHOLD: u64 = 100;
static mut DEBUG_FLAG: bool = false;
static mut COUNTER: u64 = 0;

fn main() {
    let iterations = 100_000_000u64;
    
    //   Pre-Warming  
    unsafe {
        let start = Instant::now();
        for i in 0..iterations {
            let value = i;
            if value > CONFIG_THRESHOLD && !DEBUG_FLAG {
                COUNTER += 1;
            }
        }
        let cold_time = start.elapsed().as_millis();
        println!("Without Pre-Warming: {} ms", cold_time);
        
        //  Pre-Warming Ceremony  
        COUNTER = 0;
        let _warm = CONFIG_THRESHOLD ^ (DEBUG_FLAG as u64) ^ COUNTER ^ 0;  
        
        let start = Instant::now();
        for i in 0..iterations {
            let value = i;
            if value > CONFIG_THRESHOLD && !DEBUG_FLAG {
                COUNTER += 1;
            }
        }
        let warm_time = start.elapsed().as_millis();
        println!("With Pre-Warming   : {} ms", warm_time);
        println!("Speedup: {:.2}x", cold_time as f64 / warm_time as f64);
    }
}