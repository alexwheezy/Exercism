const std = @import("std");
const ascii = std.ascii;
const IntBitSet = std.bit_set.IntegerBitSet;

const TOTAL_CHARS = 26;

pub fn isPangram(str: []const u8) bool {
    var chars = IntBitSet(TOTAL_CHARS).initEmpty();
    for (str) |c| {
        if (ascii.isUpper(c)) {
            chars.set(c - 'A');
        } else if (ascii.isLower(c)) {
            chars.set(c - 'a');
        }
    }
    return chars.count() == TOTAL_CHARS;
}
