
# The Memory Temperature Principle | اصل هم‌دما شدن حافظه

**First Discovery and Formal Naming by Danial Diba (danidiba)**  
**First Public Disclosure: 8 December 2025**

A novel mental model for explaining a major class of mysterious performance bugs in hot loops and its practical, zero-overhead solution.  
یک مدل ذهنی جدید برای توضیح باگ‌های عجیب عملکردی در حلقه‌های داغ و راه‌حل عملی آن.

## Overview
The **Memory Temperature Principle** classifies memory accesses by "temperature":  
- **Cold Variables**: Accessed ≤1 time per millisecond (or once total).  
- **Hot Paths**: ≥1000 accesses per millisecond in L1d cache.  

Introducing a cold variable into a hot path triggers **Collective Cache Frostbite**: Eviction of 4-16 hot cache lines, causing up to **23× performance degradation** (empirically validated below).

## Key Insight: Collective Cache Frostbite
- **Mechanism**: Cold access pollutes the cache associativity, cascading evictions and temporary "cooling" of the hot path.  
- **Impact**: Miss penalty ~200 cycles per eviction, leading to 2-23× slowdown in loops.  
- **Thresholds** (empirical): Cold ≤1 access/ms; Hot ≥1000/ms.

## Pre-Warming Ceremony
Touch cold variables once before the loop:  
```rust
// Rust example – measured 23x speedup potential
cold_var ^= 0;  // Zero-cost touch

for i in 0..1000 {
    // Hot loop now stable
}
```

## Empirical Validation: Benchmark Results
Tested on x86 (Rust 1.91.1, native opt) with criterion. Hot data: 32KB Vec<u64>. Cold data: 2MB Vec<u64>. Iterations: 1000.

| Scenario                  | Mean Time | Ratio to Baseline |
|---------------------------|-----------|-------------------|
| **Baseline (Hot Only)**  | 1.33 ms  | 1x               |
| **Frostbite (Cold Access)** | 31.23 ms | **23.5x slower** |
| **Pre-Warm (Solution)**  | 1.94 ms  | 1.46x (16x vs Frostbite) |

- **Setup**: Cold touch every 2 iterations in Frostbite; pre-sum in Pre-Warm.  
- Full report: Run `cargo bench` in benchmark/ and view `target/criterion/.../report/index.html`.

## Run the Benchmark
1. `cd benchmark`  
2. `$env:RUSTFLAGS="-C target-cpu=native -C opt-level=3"; cargo bench` (PowerShell)  
3. Check results (~23x slowdown).

## Original Examples
- [Rust Example](benchmark.rs)  
- [C Example](benchmark.c)  
- [Zig Example](benchmark.zig)

## Paper
[Download PDF](The%20Memory%20Temperature%20Principle%20-%20Danial%20Diba.pdf)

## References
[references.bib](references.bib)

**License: MIT** | **Contact: "Danial Diba" 
```


# The Memory Temperature Principle | اصل هم‌دما شدن حافظه

**First Discovery and Formal Naming by Danial Diba (danidiba)**
[cite_start]**First Public Disclosure: 8 December 2025** [cite: 7, 51]
