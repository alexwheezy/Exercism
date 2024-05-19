const std = @import("std");
const mem = std.mem;

pub const ConversionError = error{
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit,
};

/// Converts `digits` from `input_base` to `output_base`, returning a slice of digits.
/// Caller owns the returned memory.
pub fn convert(
    allocator: mem.Allocator,
    digits: []const u32,
    input_base: u32,
    output_base: u32,
) (mem.Allocator.Error || ConversionError)![]u32 {
    if (input_base < 2) return ConversionError.InvalidInputBase;
    if (output_base < 2) return ConversionError.InvalidOutputBase;

    var buf = std.ArrayList(u32).init(allocator);
    var value: u32 = 0;
    for (digits, 1..) |digit, i| {
        if (digit >= input_base) return ConversionError.InvalidDigit;
        value += digit * std.math.pow(u32, input_base, @intCast(digits.len - i));
    }
    if (value == 0) try buf.append(0);
    while (value != 0) {
        try buf.append(value % output_base);
        value /= output_base;
    }
    std.mem.reverse(u32, buf.items);
    return buf.toOwnedSlice();
}
