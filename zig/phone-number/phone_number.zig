const std = @import("std");
const ascii = std.ascii;

pub fn clean(phrase: []const u8) ?[10]u8 {
    var buf: ?[10]u8 = [_]u8{0} ** 10;
    var index: usize = 0;
    for (phrase) |c| {
        if (ascii.isDigit(c)) {
            buf.?[index] = c;
            index += 1;
        }
    }
    return buf;
}
