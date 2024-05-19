const std = @import("std");
const ascii = std.ascii;

fn isUpper(s: []const u8) bool {
    return for (s) |c| {
        if (ascii.isAlphabetic(c)) {
            if (ascii.isLower(c)) {
                break false;
            }
        }
    } else true;
}

fn isQuestion(s: []const u8) bool {
    if (s.len == 0) return false;
    var len = s.len - 1;
    return while (len > 0) : (len -= 1) {
        if (ascii.isWhitespace(s[len])) continue;
        if (ascii.isAlphabetic(s[len])) break false;
        if (s[len] == '?') break true;
    } else false;
}

fn isWhitespace(s: []const u8) bool {
    return for (s) |c| {
        if (!ascii.isWhitespace(c)) break false;
    } else true;
}

fn isAlphabetic(s: []const u8) bool {
    var is_alphabetic = false;
    for (s) |c| {
        if (!ascii.isAlphabetic(c)) continue;
        is_alphabetic = true;
    }
    return is_alphabetic;
}

pub fn response(s: []const u8) []const u8 {
    const is_alpha_upper = isUpper(s) and isAlphabetic(s);
    const is_whitespace = isWhitespace(s);
    const is_question = isQuestion(s);
    if (is_question) {
        if (is_alpha_upper) {
            return "Calm down, I know what I'm doing!";
        }
        return "Sure.";
    } else if (is_alpha_upper) {
        return "Whoa, chill out!";
    } else if (is_whitespace) {
        return "Fine. Be that way!";
    }
    return "Whatever.";
}
