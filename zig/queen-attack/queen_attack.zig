pub const QueenError = error{
    InitializationFailure,
};

pub const Queen = struct {
    row: i8,
    col: i8,

    pub fn init(row: i8, col: i8) QueenError!Queen {
        if (row < 0 or row >= 8 or col < 0 or col >= 8) {
            return error.InitializationFailure;
        }
        return .{ .row = row, .col = col };
    }

    pub fn canAttack(self: Queen, other: Queen) QueenError!bool {
        return self.row == other.row or
            self.col == other.col or
            self.row - other.row == self.col - other.col or
            self.row + self.col == other.row + other.col;
    }
};
