const std = @import("std");
const ascii = std.ascii;
const mem = std.mem;

pub const SeriesError = error{
    InvalidCharacter,
    NegativeSpan,
    InsufficientDigits,
};

pub fn largestProduct(digits: []const u8, span: i32) SeriesError!u64 {
    if (span == 0) return 1;
    if (span > digits.len or digits.len == 0) return error.InsufficientDigits;
    if (span < 0) return error.NegativeSpan;

    var it = mem.window(u8, digits, @intCast(span), 1);
    var product: u64 = 0;
    while (it.next()) |slice| {
        var temp: u64 = 1;
        for (slice) |digit| {
            if (!ascii.isDigit(digit)) return error.InvalidCharacter;
            temp *= digit - '0';
        }
        product = @max(product, temp);
    }
    return product;
}
