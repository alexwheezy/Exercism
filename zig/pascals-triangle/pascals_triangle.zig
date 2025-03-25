const std = @import("std");
const mem = std.mem;

pub fn rows(allocator: mem.Allocator, count: usize) mem.Allocator.Error![][]u128 {
    const arrays = try allocator.alloc([]u128, count);
    for (arrays, 1..) |*array, i| {
        array.* = try allocator.alloc(u128, i);
        @memset(array.*, 1);
    }
    for (arrays, 0..) |array, i| {
        if (array.len > 1 and i < arrays.len - 1) {
            var it = mem.window(u128, array, 2, 1);
            var index: usize = 1;
            while (it.next()) |pair| {
                arrays[i + 1][index] = pair[0] + pair[1];
                index += 1;
            }
        }
    }
    return arrays;
}
