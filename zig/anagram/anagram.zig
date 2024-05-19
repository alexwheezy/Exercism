const std = @import("std");
const mem = std.mem;

/// Returns the set of strings in `candidates` that are anagrams of `word`.
/// Caller owns the returned memory.
pub fn detectAnagrams(
    allocator: mem.Allocator,
    word: []const u8,
    candidates: []const []const u8,
) !std.BufSet {
    var buf_set = std.BufSet.init(allocator);
    const word_buf = try std.ascii.allocLowerString(allocator, word);
    defer allocator.free(word_buf);
    std.mem.sort(u8, word_buf, {}, std.sort.asc(u8));

    for (candidates) |candidate| {
        const candidate_buf = try std.ascii.allocLowerString(allocator, candidate);
        defer allocator.free(candidate_buf);
        if (std.ascii.eqlIgnoreCase(word, candidate_buf)) continue;
        std.mem.sort(u8, candidate_buf, {}, std.sort.asc(u8));
        if (std.mem.eql(u8, word_buf, candidate_buf)) {
            try buf_set.insert(candidate);
        }
    }
    return buf_set;
}
