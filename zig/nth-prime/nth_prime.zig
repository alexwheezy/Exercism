const std = @import("std");
const mem = std.mem;

pub fn prime(allocator: mem.Allocator, number: usize) !usize {
    var alloc = std.ArrayList(usize).init(allocator);
    defer alloc.deinit();
    var count: usize = 2;
    while (count > 0) : (count += 1) {
        var is_prime = true;
        for (2..(count / 2 + 1)) |i| {
            if (count % i == 0) {
                is_prime = false;
                break;
            }
        }
        if (is_prime) try alloc.append(count);
        if (alloc.items.len == number) break;
    }
    return alloc.items[number - 1];
}
