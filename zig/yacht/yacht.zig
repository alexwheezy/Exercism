const std = @import("std");
const allEqual = std.mem.allEqual;

pub const Category = enum(u8) {
    ones = 1,
    twos,
    threes,
    fours,
    fives,
    sixes,
    full_house,
    four_of_a_kind,
    little_straight = 15,
    big_straight = 20,
    choice,
    yacht,
};

fn sum(dice: []const u3) u32 {
    var total: u32 = 0;
    for (dice) |d| total += d;
    return total;
}

fn sumEqualValue(dice: []const u3, value: u8) u32 {
    var total: u32 = 0;
    for (dice) |d| {
        if (d == value) total += value;
    }
    return total;
}

pub fn score(dice: [5]u3, category: Category) u32 {
    std.debug.assert(dice.len == 5);

    var arr = dice;
    std.mem.sort(u3, &arr, {}, comptime std.sort.asc(u3));

    const straight_score = 30;
    const yacht_score = 50;
    const is_all_equals = allEqual(u3, arr[0..], arr[0]);
    const all_sum = sum(&arr);

    return switch (category) {
        .ones, .twos, .threes, .fours, .fives, .sixes => |variant| sumEqualValue(&arr, @intFromEnum(variant)),
        .full_house => blk: {
            if ((!is_all_equals) and ((allEqual(u3, arr[0..2], arr[0]) and allEqual(u3, arr[2..], arr[4])) or
                ((allEqual(u3, arr[0..3], arr[0]) and allEqual(u3, arr[3..], arr[4])))))
            {
                break :blk all_sum;
            } else break :blk 0;
        },
        .four_of_a_kind => blk: {
            if (allEqual(u3, arr[0..4], arr[0])) {
                break :blk sum(arr[0..4]);
            } else if (allEqual(u3, arr[1..], arr[1])) {
                break :blk sum(arr[1..]);
            } else break :blk 0;
        },
        .little_straight, .big_straight => |variant| if (all_sum == @intFromEnum(variant)) straight_score else 0,
        .choice => all_sum,
        .yacht => if (is_all_equals) yacht_score else 0,
    };
}
