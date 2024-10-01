pub fn eggCount(number: usize) usize {
    var result: @TypeOf(number) = 0;
    const byte = 8;
    inline for (0..@sizeOf(@TypeOf(number)) * byte) |mask| {
        result += (number >> @intCast(mask)) & 0x1;
    }
    return result;
}
