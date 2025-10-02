const std = @import("std");

pub fn truncate(phrase: []const u8) []const u8 {
    if(phrase.len == 0) return phrase;
    const max_len = 5;
    var count: usize = 0;
    var len: usize = 0;
    var utf8 = (std.unicode.Utf8View.init(phrase) catch return phrase).iterator();
    while(utf8.nextCodepointSlice()) |codepoint| : (count += 1){
        if(count >= max_len) return phrase[0..len];
        len += codepoint.len;
    }
    return phrase;
}
