// benchmark.c
// Compile: gcc -O3 -march=native benchmark.c -o benchmark_c
// Run: ./benchmark_c

#include <stdio.h>
#include <time.h>

static unsigned long long config_threshold = 100;
static int debug_flag = 0;
static unsigned long long counter = 0;

unsigned long long get_nano() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1000000000ULL + ts.tv_nsec;
}

int main() {
    const long long iterations = 100000000LL;
    
    // بدون گرم کردن
    unsigned long long start = get_nano();
    for (long long i = 0; i < iterations; i++) {
        if (i > config_threshold && !debug_flag) counter++;
    }
    unsigned long long cold_time = get_nano() - start;
    
    // با Pre-Warming Ceremony
    counter = 0;
    volatile unsigned long long warm = config_threshold ^ debug_flag ^ counter; // گرم کردن
    
    start = get_nano();
    for (long long i = 0; i < iterations; i++) {
        if (i > config_threshold && !debug_flag) counter++;
    }
    unsigned long long warm_time = get_nano() - start;
    
    printf("Without Pre-Warming: %.2f ms\n", cold_time / 1e6);
    printf("With Pre-Warming   : %.2f ms\n", warm_time / 1e6);
    printf("Speedup: %.2fx\n", (double)cold_time / warm_time);
    
    return 0;
}