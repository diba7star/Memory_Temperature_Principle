# The Memory Temperature Principle | اصل هم‌دما شدن حافظه

**First Discovery and Formal Naming by Danial Diba (danidiba)**
[cite_start]**First Public Disclosure: 8 December 2025** [cite: 7, 51]

A novel mental model for explaining a major class of mysterious performance bugs in hot loops and its practical, zero-overhead solution.
[cite_start]یک مدل ذهنی جدید برای توضیح باگ‌های عجیب عملکردی در حلقه‌های داغ و راه‌حل عملی آن. [cite: 9, 53]

---

## 🧊 3. Collective Cache Frostbite (سرمازدگی جمعی حافظه)

### پدیدهٔ اصلی (The Core Phenomenon)

[cite_start]When a **Cold** variable ($\leq 1$ access/ms or accessed only once) [cite: 19, 63] [cite_start]is suddenly touched inside a **Hot Path** ($\ge 1000$ accesses/ms in L1d cache) [cite: 16, 17, 60, 61][cite_start], the CPU must evict multiple **Hot** cache lines (typically 4–16)[cite: 21, 65].
[cite_start]وقتی یک متغیر **سرد** (Cold) ناگهان داخل یک مسیر **داغ** (Hot Path) استفاده می‌شود، CPU مجبور است چندین خط کش داغ را از L1 بیرون بریزد. [cite: 21, 65]

[cite_start]This single access triggers a **cascading temperature collapse**—the entire hot path temporarily becomes Cold until re-warmed [cite: 22, 66]—formally named **Collective Cache Frostbite**. [cite_start]This results in up to **4.2× performance degradation**[cite: 11, 55].

### 
---

## 🔥 4. The Pre-Warming Ceremony (مراسم پیش‌گرمایش)

### راه‌حل پیشنهادی (The Proposed Solution)

[cite_start]The solution is a practical, **compiler-independent**, **zero-overhead** technique [cite: 41, 85] to stabilize the cache. [cite_start]**Deliberately touch (read or XOR with zero) every Cold variable** that will be used in the hot path **once, immediately before entry**[cite: 28, 72].

[cite_start]**مراسم پیش‌گرمایش:** قبل از ورود به حلقهٔ بحرانی، یک‌بار همهٔ متغیرهای مورد نیاز رو «لمس» کن (حتی با عمل XOR بی‌معنی)[cite: 28, 72]:

```rust
// Rust example – real measured 4.2x speedup
let _warm = config.threshold ^ flags.debug ^ metrics.counter ^ 0; // Pre-Warming Ceremony

for i in 0..100_000_000 {
    // This loop now runs on a "warm" cache.
    if value > config.threshold && !flags.debug {
        metrics.counter += 1;
    }
}
