pub fn isValidIsbn10(s: []const u8) bool {
    const max_len = 10;
    const count_dash = 3;

    if (s.len < max_len) return false;

    var idx: u8 = @intCast(if (s.len > max_len) s.len - count_dash else s.len);
    var digit: u16 = 0;
    for (s) |c| {
        switch (c) {
            '-' => continue,
            'A'...'Z' => |v| {
                if (v == 'X' and idx == 1) {
                    digit += max_len;
                } else {
                    return false;
                }
            },
            else => {
                digit += (c - '0') * idx;
                idx = @max(1, idx) - 1;
            },
        }
    }
    return @mod(digit, 11) == 0;
}
