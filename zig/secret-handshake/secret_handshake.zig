const std = @import("std");
const mem = std.mem;

pub const Signal = enum(u5) {
    wink = 0b00001,
    double_blink = 0b00010,
    close_your_eyes = 0b00100,
    jump = 0b01000,
};

pub fn calculateHandshake(allocator: mem.Allocator, number: u5) mem.Allocator.Error![]const Signal {
    var signals = std.ArrayList(Signal).init(allocator);
    for (0..4) |shift| {
        const mask = number & @as(u5, 1) << @intCast(shift);
        if (mask == 0) continue;
        try signals.append(@enumFromInt(mask));
    }
    if (number & 0b10000 != 0) std.mem.reverse(Signal, signals.items);
    return signals.toOwnedSlice();
}
