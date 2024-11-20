const std = @import("std");
const mem = std.mem;

pub const Item = struct {
    weight: usize,
    value: usize,

    pub fn init(weight: usize, value: usize) Item {
        return .{ .weight = weight, .value = value };
    }
};

pub fn maximumValue(allocator: mem.Allocator, maximumWeight: usize, items: []const Item) !usize {
    if (items.len == 0 or maximumWeight == 0) return 0;

    var m = try allocator.alloc([]usize, items.len + 1);
    defer allocator.free(m);

    for (0..items.len + 1) |i| {
        m[i] = try allocator.alloc(usize, maximumWeight + 1);
        for (0..maximumWeight + 1) |w| {
            m[i][w] = 0;
        }
    }

    defer for (0..items.len + 1) |i| {
        allocator.free(m[i]);
    };

    for (0..items.len) |x| {
        const i = items.len - x - 1;
        for (0..maximumWeight) |y| {
            const w = maximumWeight - y;
            m[i][w] = m[i + 1][w];
            if (items[i].weight <= w) {
                m[i][w] = @max(m[i][w], items[i].value + m[i + 1][w - items[i].weight]);
            }
        }
    }
    return m[0][maximumWeight];
}
