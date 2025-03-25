const std = @import("std");
const mem = std.mem;
const math = std.math;

pub const Triplet = struct {
    const T = usize;
    a: T,
    b: T,
    c: T,

    pub fn init(a: T, b: T, c: T) Triplet {
        return .{ .a = a, .b = b, .c = c };
    }
};

pub fn tripletsWithSum(allocator: mem.Allocator, n: usize) mem.Allocator.Error![]Triplet {
    var buf = std.ArrayList(Triplet).init(allocator);
    var a: usize = 0;
    var b: usize = 0;
    var c: usize = n / 2;
    while (c > 0) : (c -= 1) {
        while (b < c) : (b += 1) {
            while (a < b) : (a += 1) {
                const cond = a * a + b * b == c * c and a + b + c == n;
                if (cond) {
                    try buf.append(Triplet.init(a, b, c));
                }
            }
            a = 0;
        }
        b = 0;
    }
    return buf.toOwnedSlice();
}
