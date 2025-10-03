const std = @import("std");
const mem = std.mem;

pub const ColorBand = enum {
    black,
    brown,
    red,
    orange,
    yellow,
    green,
    blue,
    violet,
    grey,
    white,
};

const Unit = enum {
    ohms,
    kiloohms,
    megaohms,
    gigaohms,
};

pub fn label(allocator: mem.Allocator, colors: []const ColorBand) mem.Allocator.Error![]u8 {
    if (colors.len < 3) return try std.fmt.allocPrint(allocator, "{} {s}", .{ 0, @tagName(.ohms) });

    const first = @intFromEnum(colors[0]);
    const second = @intFromEnum(colors[1]);
    const zeroes = @intFromEnum(colors[2]);

    const number_of_signs = 2 + zeroes;
    const multiplier = std.math.powi(u32, 10, zeroes) catch 0;
    const value = (@as(u64, first) * 10 + second) * multiplier;
    const resistance: f64 = @floatFromInt(value);

    const result = switch (number_of_signs) {
        0...3 => .{ resistance, @tagName(.ohms) },
        4...6 => .{ resistance / 1000.0, @tagName(.kiloohms) },
        7...9 => .{ resistance / 1_000_000.0, @tagName(.megaohms) },
        else => .{ resistance / 1_000_000_000.0, @tagName(.gigaohms) },
    };

    return try std.fmt.allocPrint(allocator, "{} {s}", result);
}
