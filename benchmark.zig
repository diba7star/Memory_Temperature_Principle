<<<<<<< HEAD
// benchmark.zig
// Compile & run: zig run benchmark.zig -O ReleaseFast

const std = @import("std");

var config_threshold: u64 = 100;
var debug_flag: bool = false;
var counter: u64 = 0;

pub fn main() void {
    const iterations: u64 = 100_000_000;
    const timer = std.time.Timer;
    
    // بدون گرم کردن
    var t = timer.start() catch unreachable;
    var i: u64 = 0;
    while (i < iterations) : (i += 1) {
        if (i > config_threshold and !debug_flag) counter += 1;
    }
    const cold = t.read();
    
    // با Pre-Warming
    counter = 0;
    const _warm = config_threshold ^ @intFromBool(debug_flag) ^ counter; _ = _warm;
    
    t = timer.start() catch unreachable;
    i = 0;
    while (i < iterations) : (i += 1) {
        if (i > config_threshold and !debug_flag) counter += 1;
    }
    const warm = t.read();
    
    std.debug.print("Without Pre-Warming: {d:.2} ms\n", .{@intToFloat(f64, cold) / 1e6});
    std.debug.print("With Pre-Warming   : {d:.2} ms\n", .{@intToFloat(f64, warm) / 1e6});
    std.debug.print("Speedup: {d:.2}x\n", .{@intToFloat(f64, cold) / @intToFloat(f64, warm)});
=======
// benchmark.zig
// Compile & run: zig run benchmark.zig -O ReleaseFast

const std = @import("std");

var config_threshold: u64 = 100;
var debug_flag: bool = false;
var counter: u64 = 0;

pub fn main() void {
    const iterations: u64 = 100_000_000;
    const timer = std.time.Timer;
    
    // بدون گرم کردن
    var t = timer.start() catch unreachable;
    var i: u64 = 0;
    while (i < iterations) : (i += 1) {
        if (i > config_threshold and !debug_flag) counter += 1;
    }
    const cold = t.read();
    
    // با Pre-Warming
    counter = 0;
    const _warm = config_threshold ^ @intFromBool(debug_flag) ^ counter; _ = _warm;
    
    t = timer.start() catch unreachable;
    i = 0;
    while (i < iterations) : (i += 1) {
        if (i > config_threshold and !debug_flag) counter += 1;
    }
    const warm = t.read();
    
    std.debug.print("Without Pre-Warming: {d:.2} ms\n", .{@intToFloat(f64, cold) / 1e6});
    std.debug.print("With Pre-Warming   : {d:.2} ms\n", .{@intToFloat(f64, warm) / 1e6});
    std.debug.print("Speedup: {d:.2}x\n", .{@intToFloat(f64, cold) / @intToFloat(f64, warm)});
>>>>>>> ba78e598db2acd4a7ad03928b530de06dda4eafa
}