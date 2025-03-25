const std = @import("std");
const mem = std.mem;

pub fn factors(allocator: mem.Allocator, value: u64) mem.Allocator.Error![]u64 {
    if (value < 2) return &[_]u64{};

    var buf = std.ArrayList(u64).init(allocator);
    defer buf.deinit();

    var divisor: u64 = 2;
    var num = value;
    while (divisor * divisor <= num) {
        if (num % divisor == 0) {
            try buf.append(divisor);
            num /= divisor;
        } else {
            divisor += 1;
        }
    }

    if (num > 1) try buf.append(num);
    return buf.toOwnedSlice();
}
