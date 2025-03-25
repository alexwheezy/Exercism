const std = @import("std");
const assert = std.debug.assert;
const mem = std.mem;

pub fn toRoman(allocator: mem.Allocator, arabicNumeral: i16) mem.Allocator.Error![]u8 {
    assert(arabicNumeral > 0);

    var buf = std.ArrayList(u8).init(allocator);
    defer buf.deinit();

    const M = [_][]const u8{ "", "M", "MM", "MMM" };
    const C = [_][]const u8{ "", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM" };
    const X = [_][]const u8{ "", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC" };
    const I = [_][]const u8{ "", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX" };

    const numeral: usize = @intCast(arabicNumeral);
    const thousands = M[numeral / 1000];
    const hundreds = C[numeral % 1000 / 100];
    const tens = X[numeral % 100 / 10];
    const ones = I[numeral % 10];

    inline for (&[4][]const u8{ thousands, hundreds, tens, ones }) |value| {
        try buf.appendSlice(value);
    }
    return buf.toOwnedSlice();
}
