const std = @import("std");
const mem = std.mem;
const ascii = std.ascii;

pub fn clean(phrase: []const u8) ?[10]u8 {
    const max_elems = 10;
    var buf: ?[max_elems]u8 = undefined;
    var index: usize = 0;
    for (phrase) |c| {
        if (ascii.isDigit(c)) {
            // ignore leading 1
            if (index == 0 and c == '1') continue;
            // invalid area code
            if ((index == 0 or index == 3) and c < '2') return null;
            if (index == max_elems) return null;
            buf.?[index] = c;
            index += 1;
        }
    }
    if (index < max_elems) return null;
    return buf;
}
