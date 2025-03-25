const std = @import("std");
const mem = std.mem;

pub const Triplet = struct {
    a: usize,
    b: usize,
    c: usize,

    pub fn init(a: usize, b: usize, c: usize) Triplet {
        return .{ .a = a, .b = b, .c = c };
    }
};

pub fn tripletsWithSum(allocator: mem.Allocator, n: usize) mem.Allocator.Error![]Triplet {
    var buf = std.ArrayList(Triplet).init(allocator);
    for (2..n) |a| {
        const b = n / 2 - a * n / (2 * (n - a));
        if (a >= b) break;
        const c = n - (a + b);
        if (a * a + b * b == c * c) {
            try buf.append(Triplet.init(a, b, c));
        }
    }
    return buf.toOwnedSlice();
}
