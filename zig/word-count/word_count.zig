const std = @import("std");
const ascii = std.ascii;
const mem = std.mem;

/// Returns the counts of the words in `s`.
/// Caller owns the returned memory.
pub fn countWords(allocator: mem.Allocator, s: []const u8) !std.StringHashMap(u32) {
    var hash_map = std.StringHashMap(u32).init(allocator);
    var it = mem.tokenizeAny(u8, s, " \n\t:!&@$%^,.\"");

    while (it.next()) |key| {
        var word = key;
        const start_quote = ascii.startsWithIgnoreCase(word, "'");
        const end_quote = ascii.endsWithIgnoreCase(word, "'");
        if (start_quote and end_quote) {
            if (word.len == 1) continue;
            word = word[1 .. word.len - 1];
        } else if (start_quote) {
            word = word[1..];
        } else if (end_quote) {
            word = word[0 .. word.len - 1];
        }
        const word_lower = try ascii.allocLowerString(allocator, word);
        if (hash_map.getPtr(word_lower)) |value| {
            value.* += 1;
            defer allocator.free(word_lower);
        } else {
            try hash_map.put(word_lower, 1);
        }
    }
    return hash_map;
}
