const std = @import("std");
const mem = std.mem;
const fmt = std.fmt;

pub fn recite(
    allocator: mem.Allocator,
    words: []const []const u8,
) (fmt.AllocPrintError || mem.Allocator.Error)![][]u8 {
    var buffer = try allocator.alloc([]u8, words.len);
    if (words.len == 0) return buffer;

    var index: usize = 1;
    while (index < words.len) : (index += 1) {
        const str = try fmt.allocPrint(allocator, "For want of a {s} the {s} was lost.\n", .{
            words[index - 1],
            words[index],
        });
        buffer[index - 1] = str;
    }
    buffer[index - 1] = try fmt.allocPrint(allocator, "And all for the want of a {s}.\n", .{words[0]});
    return buffer;
}
