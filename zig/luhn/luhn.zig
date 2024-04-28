const std = @import("std");
const ascii = std.ascii;

pub fn isValid(s: []const u8) bool {
    std.debug.assert(s.len < 64);
    var values: [64]u8 = undefined;
    var len: u8 = 0;
    var offset: usize = 0;

    for (s, 0..) |char, i| {
        switch (char) {
            ' ' => {
                offset += 1;
                continue;
            },
            '0'...'9' => {
                values[i - offset] = char - '0';
                len += 1;
            },
            else => return false,
        }
    }

    if (len < 2) return false;

    const max = 9;
    var sum: u32 = 0;
    var i: u8 = 0;
    while (i < len) : (i += 1) {
        const digit = values[len - i - 1];
        if (i % 2 != 0) {
            const double = digit * 2;
            sum += if (double > max) double - max else double;
        } else {
            sum += digit;
        }
    }
    return sum % 10 == 0;
}
