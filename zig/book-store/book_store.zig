const std = @import("std");
const mem = std.mem;

fn sum(items: []const u32) u32 {
    var result: u32 = 0;
    for (items) |item| result += item;
    return result;
}

fn countNonZero(items: []const u32) u32 {
    var count: u32 = 0;
    for (items) |item| {
        if (item != 0) count += 1;
    }
    return count;
}

fn makeDiscount(count_of_books: u32) u32 {
    const cost = 8;
    const discount: u32 = switch (count_of_books) {
        1 => 100,
        2 => 95,
        3 => 90,
        4 => 80,
        5 => 75,
        else => unreachable,
    };
    return count_of_books * discount * cost;
}

fn calculateCost(books: []u32) u32 {
    var cost: u32 = 0;
    const last_index = mem.lastIndexOfScalar(u32, books, 0) orelse books.len;

    while (countNonZero(books) != 0) {
        cost += makeDiscount(countNonZero(books));
        for (0..last_index) |item| {
            if (books[item] == 0) continue;
            books[item] -= 1;
        }
    }
    return cost;
}

fn totalCost(items: []const u32) u32 {
    var books: [5]u32 = undefined;
    var costs: [3]u32 = undefined;

    // We only need to calculate 3 grouping options (for 3, 4, 5 items), the others will be suboptimal
    for (2..items.len, 0..) |i, idx| {
        @memcpy(&books, items);
        books[i] = sum(books[i..]);
        for (i + 1..items.len) |item| books[item] = 0;
        costs[idx] = calculateCost(&books);
    }
    return mem.min(u32, &costs);
}

pub fn total(basket: []const u32) u32 {
    if (basket.len == 0) return 0;

    const max_different_books = 5;
    var books = [_]u32{0} ** max_different_books;

    for (basket) |item| books[item - 1] += 1;

    // We need to sort the array of books in descending order so that when calculating the grouping options we always start at the beginning of the array
    std.mem.sort(u32, &books, {}, std.sort.desc(u32));

    return totalCost(&books);
}
