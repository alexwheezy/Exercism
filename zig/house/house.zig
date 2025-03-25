const std = @import("std");

fn verse(buffer: []u8, begin: u32) []const u8 {
    var size: usize = 7; // Start with "This is"
    std.mem.copyBackwards(u8, buffer, "This is");

    const phrases = [_][]const u8{
        " the house that Jack built.\n",
        " the malt that lay in",
        " the rat that ate",
        " the cat that killed",
        " the dog that worried",
        " the cow with the crumpled horn that tossed",
        " the maiden all forlorn that milked",
        " the man all tattered and torn that kissed",
        " the priest all shaven and shorn that married",
        " the rooster that crowed in the morn that woke",
        " the farmer sowing his corn that kept",
        " the horse and the hound and the horn that belonged to",
    };

    var index = begin;
    while (index >= 1) : (index -= 1) {
        const phrase = phrases[index - 1];
        std.mem.copyBackwards(u8, buffer[size..], phrase);
        size += phrase.len;
    }
    return buffer[0..size];
}

pub fn recite(buffer: []u8, start_verse: u32, end_verse: u32) []const u8 {
    if (start_verse > end_verse) {
        @panic("start_verse must be less than or equal to end_verse");
    }
    var size: usize = 0;
    var index = start_verse;
    var buf: [512]u8 = undefined;
    while (index <= end_verse) : (index += 1) {
        const sentense = verse(&buf, index);
        std.mem.copyBackwards(u8, buffer[size..], sentense);
        size += sentense.len;
    }
    return buffer[0 .. size - 1];
}
