const std = @import("std");

pub fn isIsogram(str: []const u8) bool {
    var flags: u32 = 0;
    for (str) |c| {
        const upper = std.ascii.toUpper(c);
        if ('A' <= upper and upper <= 'Z') {
            const bit = @as(u32, 1) << @as(u5, @intCast(upper - 'A'));
            if (bit & flags != 0) {
                return false;
            }
            flags |= bit;
        }
    }
    return true;
}
