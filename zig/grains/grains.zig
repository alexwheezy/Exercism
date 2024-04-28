pub const ChessboardError = error{IndexOutOfBounds};

pub fn square(index: usize) ChessboardError!u64 {
    switch (index) {
        0, 65 => return ChessboardError.IndexOutOfBounds,
        else => return @as(u64, 1) << @intCast(index - 1),
    }
}

pub fn total() u64 {
    return 0xffff_ffff_ffff_ffff;
}
