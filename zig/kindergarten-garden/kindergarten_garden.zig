const std = @import("std");

pub const Plant = enum { clover, grass, radishes, violets };
pub const Student = enum { Alice, Bob, Charlie, David, Eve, Fred, Ginny, Harriet, Ileana, Joseph, Kincaid, Larry };

pub fn plants(diagram: []const u8, student: []const u8) [4]Plant {
    var result: [4]Plant = undefined;
    const index: usize = @intFromEnum(std.meta.stringToEnum(Student, student).?);
    const offset = 1;
    const first = index * 2;
    const second = first + (diagram.len / 2) + offset;
    const values: []const u8 = &[4]u8{ diagram[first], diagram[first + offset], diagram[second], diagram[second + offset] };
    for (values, 0..) |value, i| {
        switch (value) {
            'C' => result[i] = .clover,
            'G' => result[i] = .grass,
            'R' => result[i] = .radishes,
            'V' => result[i] = .violets,
            else => unreachable,
        }
    }
    return result;
}
