const std = @import("std");
const mem = std.mem;

pub fn translate(allocator: mem.Allocator, phrase: []const u8) mem.Allocator.Error![]u8 {
    const capacity: usize = @intFromFloat(@as(f16, @floatFromInt(phrase.len)) * 1.5);
    var buffer = try std.ArrayList(u8).initCapacity(allocator, capacity);
    var it = mem.tokenizeScalar(u8, phrase, ' ');
    while (it.next()) |word| {
        if (mem.indexOfScalar(u8, "aeiou", word[0]) != null or mem.startsWith(u8, word, "xr") or mem.startsWith(u8, word, "yt")) {
            try buffer.appendSlice(word);
        } else {
            const pos: usize = blk: {
                if (mem.indexOf(u8, word, "qu")) |value| {
                    break :blk value * 1 + 2;
                } else {
                    const value = mem.indexOfAny(u8, word, "aeiouy").?;
                    break :blk if (value != 0) value else 1;
                }
            };
            try buffer.appendSlice(word[pos..]);
            try buffer.appendSlice(word[0..pos]);
        }
        try buffer.appendSlice("ay");
        if (it.peek() != null) try buffer.append(' ');
    }
    return buffer.toOwnedSlice();
}
