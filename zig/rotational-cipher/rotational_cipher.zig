const std = @import("std");
const ascii = std.ascii;
const mem = std.mem;

pub fn rotate(allocator: mem.Allocator, text: []const u8, shiftKey: u5) mem.Allocator.Error![]u8 {
    const ascii_len = 26;
    var buf = try allocator.alloc(u8, text.len);
    for (text, 0..) |c, i| {
        if (ascii.isAlphabetic(c)) {
            const start: u8 = if (ascii.isUpper(c)) 'A' else 'a';
            buf[i] = (c - start + shiftKey) % ascii_len + start;
        } else {
            buf[i] = c;
        }
    }
    return buf;
}
