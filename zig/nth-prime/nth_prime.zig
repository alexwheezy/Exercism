const std = @import("std");
const mem = std.mem;
const math = std.math;

pub fn prime(allocator: mem.Allocator, number: usize) !usize {
    _ = allocator; // autofix
    var step: usize = 0;
    var n: usize = 2;
    while (step < number) : (n += 1) {
        var is_prime: bool = true;
        for (2..math.sqrt(n) + 1) |p| {
            if (n % p == 0) {
                is_prime = false;
                break;
            }
        }
        if (is_prime) step += 1;
    }
    return n - 1;
}
