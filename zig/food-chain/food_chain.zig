const std = @import("std");

fn insert(
    buffer: []u8,
    phrase: []const u8,
    animal: []const u8,
    end: []const u8,
) []u8 {
    return std.fmt.bufPrint(buffer, "\n{s}{s}{s}\n", .{ phrase, animal, end }) catch unreachable;
}

fn repetition(
    buffer: []u8,
    comptime stroke: []const u8,
    animal_prev: []const u8,
    animal_next: []const u8,
    phrase: []const u8,
    end: []const u8,
) []u8 {
    return std.fmt.bufPrint(buffer, stroke, .{ animal_prev, animal_next, phrase, end }) catch unreachable;
}

fn verse(index: usize) []const u8 {
    const buffer_size = 4000;
    var buffer: [buffer_size]u8 = undefined;
    var total_len: usize = 0;

    const animals = [_][]const u8{ "fly", "spider", "bird", "cat", "dog", "goat", "cow", "horse" };
    const phrases = [_][]const u8{
        "I know an old lady who swallowed a ",
        "It wriggled and jiggled and tickled inside her",
        "How absurd to swallow a ",
        "Imagine that, to swallow a ",
        "What a hog, to swallow a ",
        "Just opened her throat and swallowed a ",
        "I don't know how she swallowed a ",
        "I don't know why she swallowed the fly. Perhaps she'll die.",
        "She's dead, of course!",
    };

    const animal = animals[index - 1];
    const start = std.fmt.bufPrint(&buffer, "{s}{s}.\n", .{ phrases[0], animal }) catch unreachable;
    total_len += start.len - 1;

    if (index < animals.len) {
        const phrase = phrases[index - 1];
        if (index != 1) {
            const tmp = blk: {
                if (index == 2) {
                    break :blk insert(buffer[total_len..], phrase, "", ".");
                } else {
                    break :blk insert(buffer[total_len..], phrase, animal, "!");
                }
            };
            total_len += tmp.len;
        }

        var rep: usize = 1;
        while (rep < index) : (rep += 1) {
            const tmp = blk: {
                if (!std.mem.eql(u8, animals[index - rep - 1], "spider")) {
                    const stroke = "She swallowed the {s} to catch the {s}{s}{s}\n";
                    break :blk repetition(buffer[total_len..], stroke, animals[index - rep], animals[index - rep - 1], "", ".");
                } else {
                    const stroke = "She swallowed the {s} to catch the {s} that {s}{s}\n";
                    break :blk repetition(buffer[total_len..], stroke, animals[index - rep], animals[index - rep - 1], phrases[1][3..], ".");
                }
            };
            total_len += tmp.len;
        }
        if (index > 1) total_len -= 1;
    }

    const end = if (index == animals.len) phrases[phrases.len - 1] else phrases[phrases.len - 2];
    const line = std.fmt.bufPrint(buffer[total_len + 1 ..], "{s}\n\n", .{end}) catch unreachable;
    total_len += line.len + 1;
    return buffer[0..total_len];
}

pub fn recite(buffer: []u8, start_verse: u32, end_verse: u32) []const u8 {
    var fbs = std.io.fixedBufferStream(buffer);
    const stream = fbs.writer();
    const offset_newline = 2;
    var len: usize = 0;
    for (start_verse..end_verse + 1) |index| {
        const stroke = verse(index);
        _ = stream.writeAll(stroke) catch undefined;
        len += stroke.len;
    }
    return fbs.buffer[0 .. len - offset_newline];
}
