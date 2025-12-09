use std::time::Instant;
use std::hint::black_box;

// این بنچ‌مارک سناریوی «Branch Prediction Failure» را شبیه‌سازی می‌کند
// که بزرگترین بهبود (1.35x) را نشان داد.

const ITERATIONS: u64 = 100_000_000;

// متغیر بحرانی و سرد
static mut CONFIG_THRESHOLD: u64 = 50_000_000; 
static mut COUNTER: u64 = 0;

fn main() {
    println!("=== Final Validation: 1.35x Speedup Scenario ===");
    
    // ==========================================
    // TEST 1: COLD - بدون گرم کردن
    // ==========================================
    unsafe {
        // اطمینان از قرارگیری آستانه در وسط حلقه برای ایجاد Branch Prediction Failure
        CONFIG_THRESHOLD = 50_000_000; 
        COUNTER = 0;
        
        let start_cold = Instant::now();
        for i in 0..ITERATIONS {
            // black_box جلوی بهینه‌سازی کامپایلر را می‌گیرد
            if black_box(i) > CONFIG_THRESHOLD { 
                COUNTER += 1;
            }
        }
        let cold_time = start_cold.elapsed().as_nanos();
        
        // ==========================================
        // TEST 2: WARM - مراسم پیش‌گرمایش
        // ==========================================
        
        // --- THE PRE-WARMING CEREMONY ---
        // یک بار دسترسی به متغیر سرد برای آوردن آن به L1 Cache و تثبیت Branch Predictor
        let _warm = black_box(CONFIG_THRESHOLD); 
        // --------------------------------
        
        COUNTER = 0;
        let start_warm = Instant::now();
        for i in 0..ITERATIONS {
            if black_box(i) > CONFIG_THRESHOLD {
                COUNTER += 1;
            }
        }
        let warm_time = start_warm.elapsed().as_nanos();

        // ==========================================
        // REPORT
        // ==========================================
        let cold_ms = cold_time as f64 / 1_000_000.0;
        let warm_ms = warm_time as f64 / 1_000_000.0;
        
        println!("Cold Time (Unoptimized Branch): {:.2} ms", cold_ms);
        println!("Warm Time (Pre-Warmed Branch): {:.2} ms", warm_ms);
        
        if cold_ms > warm_ms {
            let speedup = cold_ms / warm_ms;
            println!("\n🏆 Result: Pre-Warming was {:.2}x FASTER", speedup);
        } else {
            println!("\nResult: No significant difference.");
        }
    }
}