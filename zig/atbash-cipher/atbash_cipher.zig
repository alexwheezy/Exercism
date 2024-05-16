const std = @import("std");
const ascii = std.ascii;
const mem = std.mem;

/// Encodes `s` using the Atbash cipher. Caller owns the returned memory.
fn conversion(allocator: mem.Allocator, s: []const u8, comptime split: bool) mem.Allocator.Error![]u8 {
    var buf = try allocator.alloc(u8, s.len * 2);
    var step: usize = 0;
    for (s, 1..) |c, i| {
        const char = ascii.toLower(c);
        switch (char) {
            'a'...'z' => buf[step] = 'z' - (char - 'a'),
            '0'...'9' => buf[step] = char,
            else => {
                continue;
            },
        }
        step += 1;
        if (split) {
            // You need to separate words 5 letters long with a space and update the index of the next
            // letter in the buffer.
            if (step % 6 == 5 and i < s.len - 1) {
                buf[step] = ' ';
                step += 1;
            }
        }
    }
    _ = allocator.resize(buf, step);
    buf.len = step;
    return buf;
}
pub fn encode(allocator: mem.Allocator, s: []const u8) mem.Allocator.Error![]u8 {
    return try conversion(allocator, s, true);
}

/// Decodes `s` using the Atbash cipher. Caller owns the returned memory.
pub fn decode(allocator: mem.Allocator, s: []const u8) mem.Allocator.Error![]u8 {
    return try conversion(allocator, s, false);
}
