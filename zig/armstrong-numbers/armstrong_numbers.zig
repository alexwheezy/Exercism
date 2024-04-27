const std = @import("std");

pub fn isArmstrongNumber(num: u128) bool {
    if (num <= 0) return true;
    var value = num;
    const len = std.math.log10(value) + 1;
    var total: u128 = 0;
    while (value > 0) : (value /= 10) {
        total += std.math.pow(u128, value % 10, len);
    }
    return total == num;
}
