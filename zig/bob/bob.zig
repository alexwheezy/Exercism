const std = @import("std");
const mem = std.mem;
const ascii = std.ascii;

pub fn response(s: []const u8) []const u8 {
    _ = s;
    @compileError("not implemented");
}
