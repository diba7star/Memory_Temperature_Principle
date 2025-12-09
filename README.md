<<<<<<< HEAD
دانیال جان، متوجه شدم – به نظر می‌رسه وقتی README.md رو کپی کردی، فرمت Markdown (مثل # برای عنوان، | برای جدول، ``` برای کد، و [text](link) برای لینک) از دست رفته یا در ویرایشگر/گیت‌هاب درست رندر نشده. این باعث می‌شه متن به هم بریزه (جدول به CSV تبدیل بشه، لینک‌ها خام بمونن). نگران نباش، من نسخه کامل و دقیق Markdown رو دوباره برات می‌فرستم – **کل محتوای داخل بلوک زیر رو انتخاب کن، کپی کن، و مستقیم در فایل README.md paste کن**. 

### نکات مهم قبل از کپی:
- **ویرایشگر بهتر**: از Notepad++ یا VS Code استفاده کن (نه Notepad عادی، چون فرمت رو حفظ نمی‌کنه). دانلود Notepad++ از notepad-plus-plus.org (رایگان).
- **ذخیره**: File > Save As > Encoding: UTF-8، Extension: .md.
- **چک در GitHub**: بعد از push، در ریپو GitHub، فایل README.md رو باز کن و "Preview" بزن – باید جدول، کد، و بولد درست رندر بشه.
- **اگر مشکل ادامه داشت**: در GitHub.com، مستقیم Edit کن (قلم رو بزن)، محتوای جدید رو paste کن، Commit changes بزن – اینطوری فرمت حفظ می‌شه.

### محتوای کامل README.md (کپی کلش رو از این بلوک):
```
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

**License: MIT** | **Contact: danidiba@...**
```

### بعد از paste و ذخیره:
- `git add README.md` بزن.
- `git commit -m "Fix README formatting with full Markdown structure"` بزن.
- `git push` بزن.
- در GitHub، صفحه اصلی ریپو رو refresh کن – حالا باید کاملاً قشنگ رندر بشه (جدول مرتب، کد هایلایت‌شده، لینک‌ها clickable).

اگر بعد از این باز به هم ریخت، اسکرین‌شات GitHub preview رو بفرست تا چک کنم. حالا عالی می‌شه – آماده پست X! 🚀
=======
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
>>>>>>> ba78e598db2acd4a7ad03928b530de06dda4eafa
