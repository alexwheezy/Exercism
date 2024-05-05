const std = @import("std");
const mem = std.mem;

pub fn sum(allocator: mem.Allocator, factors: []const u32, limit: u32) !u64 {
    if (factors.len == 0) return 0;

    var buffer = std.ArrayList(u64).init(allocator);
    defer buffer.deinit();

    for (factors) |factor| {
        if (factor < limit and (factor % 3 == 0 or factor % 5 == 0)) {
            try buffer.append(factor);
        }
    }
    var total: u64 = 0;
    for (buffer.items) |item| total += item;
    return total;
}
