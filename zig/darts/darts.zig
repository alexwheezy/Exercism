pub const Coordinate = struct {
    x: f32,
    y: f32,

    const Circle = struct {
        small: usize = 1,
        medium: usize = 5,
        large: usize = 10,
        out_of_bounds: usize = 0,
    };

    pub fn init(x_coord: f32, y_coord: f32) Coordinate {
        return .{ .x = x_coord, .y = y_coord };
    }

    pub fn score(self: Coordinate) usize {
        const dist = @sqrt((self.x * self.x) + (self.y * self.y));
        const circle = Circle{};

        if (dist > circle.large) {
            return circle.out_of_bounds;
        } else if (dist > circle.medium) {
            return circle.small;
        } else if (dist > circle.small) {
            return circle.medium;
        }
        return circle.large;
    }
};
