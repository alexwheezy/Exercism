const std = @import("std");
const mem = std.mem;

pub fn abbreviate(allocator: mem.Allocator, words: []const u8) mem.Allocator.Error![]u8 {
    var buf = try allocator.alloc(u8, words.len / 2);
    var it = mem.tokenizeAny(u8, words, " \n\t:!&@$%^,._-\"");
    var step: usize = 0;
    while (it.next()) |word| : (step += 1) {
        buf[step] = std.ascii.toUpper(word[0]);
    }
    _ = allocator.resize(buf, step);
    buf.len = step;
    return buf;
}
