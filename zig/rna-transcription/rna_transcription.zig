const mem = @import("std").mem;

pub fn toRna(allocator: mem.Allocator, dna: []const u8) mem.Allocator.Error![]const u8 {
    var buf = try allocator.alloc(u8, dna.len);
    for (dna, 0..) |c, i| {
        const transription: u8 = switch (c) {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            else => unreachable,
        };
        buf[i] = transription;
    }
    return buf;
}
