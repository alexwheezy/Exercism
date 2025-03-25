const std = @import("std");
const mem = std.mem;

pub fn toRoman(allocator: mem.Allocator, arabicNumeral: i16) mem.Allocator.Error![]u8 {
    var buf = std.ArrayList(u8).init(allocator);
    defer buf.deinit();

    var count = std.math.pow(i16, 10, @intFromFloat(@log10(@as(f16, @floatFromInt(arabicNumeral)))));

    const arr = [_]i16{ 1000, 500, 100, 50, 10, 5, 1 };
    for (arr) |value| {
        std.debug.print("{d} {d}", .{ value, @mod(count, value) });
    }

    var numeral = arabicNumeral;
    while (count > 0) {
        var value = @divTrunc(numeral, count) * count;
        std.debug.print("{d}\n", .{value});
        while (0 < value) : (value -= count) {
            switch (value) {
                1000, 2000, 3000 => try buf.append('M'),
                900 => |x| {
                    try buf.appendSlice("CM");
                    value -= x;
                },
                800 => |x| {
                    try buf.appendSlice("DCCC");
                    value -= x;
                },
                700 => |x| {
                    try buf.appendSlice("DCC");
                    value -= x;
                },
                600 => |x| {
                    try buf.appendSlice("DC");
                    value -= x;
                },
                500 => |x| {
                    try buf.append('D');
                    value -= x;
                },
                400 => |x| {
                    try buf.appendSlice("CD");
                    value -= x;
                },
                100, 200, 300 => try buf.append('C'),
                90 => |x| {
                    try buf.appendSlice("XC");
                    value -= x;
                },
                80 => |x| {
                    try buf.appendSlice("LXXX");
                    value -= x;
                },
                70 => |x| {
                    try buf.appendSlice("LXX");
                    value -= x;
                },
                60 => |x| {
                    try buf.appendSlice("LX");
                    value -= x;
                },
                50 => |x| {
                    try buf.append('L');
                    value -= x;
                },
                40 => |x| {
                    try buf.appendSlice("XL");
                    value -= x;
                },
                10, 20, 30 => try buf.append('X'),
                9 => |x| {
                    try buf.appendSlice("IX");
                    value -= x;
                },
                8 => |x| {
                    try buf.appendSlice("VIII");
                    value -= x;
                },
                7 => |x| {
                    try buf.appendSlice("VII");
                    value -= x;
                },
                6 => |x| {
                    try buf.appendSlice("VI");
                    value -= x;
                },
                5 => |x| {
                    try buf.append('V');
                    value -= x;
                },
                4 => |x| {
                    try buf.appendSlice("IV");
                    value -= x;
                },
                1, 2, 3 => try buf.append('I'),
                else => {},
            }
        }
        numeral = @mod(numeral, count);
        count = @divTrunc(count, 10);
    }
    return buf.toOwnedSlice();
}
