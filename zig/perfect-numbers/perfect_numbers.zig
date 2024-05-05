const std = @import("std");
const math = std.math;

pub const Classification = enum {
    deficient,
    perfect,
    abundant,
};

/// Asserts that `n` is nonzero.
pub fn classify(n: u64) Classification {
    std.debug.assert(n != 0);

    var sum: u64 = 0;
    var i: u64 = 1;

    while (n > i) : (i += 1) {
        if (n % i == 0) sum += i;
    }

    switch (math.order(sum, n)) {
        .eq => return .perfect,
        .gt => return .abundant,
        .lt => return .deficient,
    }
}
