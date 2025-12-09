use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn hot_loop_baseline(iterations: usize) -> u64 {
    let mut sum = 0u64;
    let hot_data: Vec<u64> = (0..4096).map(|x| (x * 2) as u64).collect(); // ~32KB hot data (fits in L1)
    for _ in 0..iterations {
        for &val in &hot_data {
            sum = sum.wrapping_add(black_box(val));
        }
    }
    sum
}

fn hot_loop_cold_access(iterations: usize) -> u64 {
    let mut sum = 0u64;
    let hot_data: Vec<u64> = (0..4096).map(|x| (x * 2) as u64).collect(); // Hot
    let cold_data: Vec<u64> = (0..262144).map(|x| (x * 100) as u64).collect(); // ~2MB cold to cause stronger eviction
    for i in 0..iterations {
        for &val in &hot_data {
            sum = sum.wrapping_add(black_box(val));
        }
        if i % 2 == 0 { // More frequent cold touch every 2 iterations for stronger frostbite
            black_box(cold_data.iter().sum::<u64>());
        }
    }
    sum
}

fn hot_loop_prewarm(iterations: usize) -> u64 {
    let mut sum = 0u64;
    let hot_data: Vec<u64> = (0..4096).map(|x| (x * 2) as u64).collect();
    let cold_data: Vec<u64> = (0..262144).map(|x| (x * 100) as u64).collect(); // No mut
    // Pre-Warming Ceremony: Touch without cost
    black_box(cold_data.iter().sum::<u64>());
    for _ in 0..iterations {
        for &val in &hot_data {
            sum = sum.wrapping_add(black_box(val));
        }
    }
    sum
}

fn bench(c: &mut Criterion) {
    let iterations = 1000; // Increased for more pressure
    let mut group = c.benchmark_group("Cache Frostbite Test");
    group.sample_size(30); // More samples for better accuracy
    group.bench_function("Baseline (Hot Only)", |b| b.iter(|| hot_loop_baseline(iterations)));
    group.bench_function("With Cold Access (Frostbite)", |b| b.iter(|| hot_loop_cold_access(iterations)));
    group.bench_function("With Pre-Warm (Solution)", |b| b.iter(|| hot_loop_prewarm(iterations)));
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);