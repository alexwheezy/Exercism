const std = @import("std");
const fmt = std.fmt;
const ascii = std.ascii;
const mem = std.mem;

fn plural(condition: bool) []const u8 {
    return if (condition) "" else "s";
}

pub fn recite(buffer: []u8, start_bottles: u32, take_down: u32) []const u8 {
    if (start_bottles < 1 or take_down < 1) {
        @panic("Need at least 1 bottle and 1 take down");
    }

    const titles = [_][]const u8{ "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten" };

    var buf: [512]u8 = undefined;
    var str: [16]u8 = undefined;
    var len: usize = 0;

    var begin = start_bottles;
    var down = take_down;
    while (down > 0) : (down -= 1) {
        const first = titles[begin - 1];
        const lower = ascii.lowerString(&str, titles[begin -| 2]);
        const last = if (begin == 1) "no" else lower;
        const suffix = plural(begin == 1);
        const format_string =
            \\{s} green bottle{s} hanging on the wall,
            \\{s} green bottle{s} hanging on the wall,
            \\And if one green bottle should accidentally fall,
            \\There'll be {s} green bottle{s} hanging on the wall.
            \\
            \\
        ;
        const phrase = fmt.bufPrint(
            &buf,
            format_string,
            .{ first, suffix, first, suffix, last, plural(begin - 1 == 1) },
        ) catch unreachable;
        mem.copyBackwards(u8, buffer[len..], phrase);
        len += phrase.len;
        begin -= 1;
    }
    return buffer[0 .. len - 2];
}
