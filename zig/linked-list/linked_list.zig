const std = @import("std");

pub fn LinkedList(comptime T: type) type {
    return struct {
        pub const Node = struct {
            prev: ?*Node = null,
            next: ?*Node = null,
            data: T,
        };

        first: ?*Node = null,
        last: ?*Node = null,
        len: T = 0,

        const Self = @This();

        pub fn push(self: *Self, node: *Node) void {
            if (self.len == 0) {
                self.first = node;
                self.last = self.first;
            } else {
                self.last.?.next = node;
                self.last.?.next.?.prev = self.last;
                self.last = node;
            }
            self.len += 1;
        }

        pub fn pop(self: *Self) ?Node {
            if (self.len != 0) {
                const node = self.last.?;
                self.last = self.last.?.prev;
                self.len -= 1;
                return node.*;
            }
            return null;
        }

        pub fn shift(self: *Self) ?Node {
            if (self.len == 0) return null;
            const node = self.first.?;
            if (node.next) |next| {
                self.first = next;
                self.first.?.prev = null;
            }
            self.len -= 1;
            return node.*;
        }

        pub fn unshift(self: *Self, node: *Node) void {
            if (self.len == 0) {
                self.first = node;
                self.last = self.first;
            } else {
                self.first.?.prev = node;
                self.first.?.prev.?.next = self.first;
                self.first = node;
            }
            self.len += 1;
        }

        pub fn delete(self: *Self, node: *Node) void {
            var last = self.last;
            while (last) |curr| : (last = curr.prev) {
                if (curr.data == node.data) {
                    if (curr.prev) |prev| {
                        last.?.* = prev.*;
                    }
                    self.len -= 1;
                    break;
                }
            }
        }

        pub fn print(self: *Self) void {
            var node = self.last;
            while (node) |curr| : (node = curr.prev) {
                std.debug.print("value: {}\n", .{curr.data});
            }
        }
    };
}
