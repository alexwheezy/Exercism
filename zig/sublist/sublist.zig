const std = @import("std");

pub const Relation = enum {
    equal,
    sublist,
    superlist,
    unequal,
};

fn compareList(list_one: []const i32, list_two: []const i32) bool {
    var it = std.mem.window(i32, list_two, list_one.len, 1);
    while (it.next()) |window| {
        if (std.mem.eql(i32, list_one, window)) return true;
    }
    return false;
}

pub fn compare(list_one: []const i32, list_two: []const i32) Relation {
    if (std.mem.eql(i32, list_one, list_two)) {
        return .equal;
    } else if (list_one.len < list_two.len) {
        if (list_one.len == 0 or compareList(list_one, list_two)) return .sublist;
    } else if (list_one.len > list_two.len) {
        if (list_two.len == 0 or compareList(list_two, list_one)) return .superlist;
    }
    return .unequal;
}
