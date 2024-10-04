const std = @import("std");
const ascii = std.ascii;

pub fn encode(buffer: []u8, string: []const u8) []u8 {
    var left: usize = 0;
    var right = left + 1;
    var count = left;
    while (right <= string.len) : (right += 1) {
        if (right == string.len or string[left] != string[right]) {
            var digit: u8 = @intCast(right - left);
            if (digit > 1) {
                count += @as(usize, @intFromFloat(@ceil(@log10(@as(f16, @floatFromInt(digit))))));
                var i: usize = 1;
                while (digit != 0) : (digit /= 10) {
                    buffer[count - i] = digit % 10 + '0';
                    i += 1;
                }
            }
            buffer[count] = string[left];
            count += 1;
            left = right;
        }
    }
    return buffer[0..count];
}

pub fn decode(buffer: []u8, string: []const u8) []u8 {
    var left: usize = 0;
    var right = left;
    var count = left;
    while (right < string.len) : (right += 1) {
        if (ascii.isDigit(string[right])) continue;
        var digit = std.fmt.parseInt(usize, string[left..right], 10) catch 1;
        while (digit > 0) : (digit -= 1) {
            buffer[count] = string[right];
            count += 1;
        }
        left = right + 1;
    }
    return buffer[0..count];
}
