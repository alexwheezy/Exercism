const std = @import("std");
const mem = std.mem;
const ascii = std.ascii;

pub fn transform(allocator: mem.Allocator, legacy: std.AutoHashMap(i5, []const u8)) mem.Allocator.Error!std.AutoHashMap(u8, i5) {
    var map = std.AutoHashMap(u8, i5).init(allocator);
    errdefer map.deinit();
    var it = legacy.iterator();
    while (it.next()) |entry| {
        const key = entry.key_ptr.*;
        const value = entry.value_ptr.*;
        for (value) |char| {
            try map.put(ascii.toLower(char), key);
        }
    }
    return map;
}
