const std = @import("std");
const fmt = std.fmt;
const mem = std.mem;

fn verse(end_verse: u32) []const u8 {
    const buffer_size = 512;
    var buffer: [buffer_size]u8 = undefined;
    const nums = [_][]const u8{
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    };
    const pharases = [_][]const u8{
        "a Partridge in a Pear Tree.\n",
        "two Turtle Doves, and ",
        "three French Hens, ",
        "four Calling Birds, ",
        "five Gold Rings, ",
        "six Geese-a-Laying, ",
        "seven Swans-a-Swimming, ",
        "eight Maids-a-Milking, ",
        "nine Ladies Dancing, ",
        "ten Lords-a-Leaping, ",
        "eleven Pipers Piping, ",
        "twelve Drummers Drumming, ",
    };
    var end: usize = end_verse;
    var len: usize = 0;

    const intro = fmt.bufPrint(
        buffer[0..],
        "On the {s} day of Christmas my true love gave to me: ",
        .{nums[end - 1]},
    ) catch unreachable;
    mem.copyBackwards(u8, &buffer, intro);
    len += intro.len;

    while (end >= 1) : (end -= 1) {
        const sentense = pharases[end - 1];
        mem.copyBackwards(u8, buffer[len..], sentense);
        len += sentense.len;
    }
    return buffer[0..len];
}

pub fn recite(buffer: []u8, start_verse: u32, end_verse: u32) []const u8 {
    var begin = start_verse;
    var len: usize = 0;
    while (begin <= end_verse) : (begin += 1) {
        const sentense = verse(begin);
        mem.copyBackwards(u8, buffer[len..], sentense);
        len += sentense.len;
    }
    return buffer[0 .. len - 1];
}
