pub fn squareRoot(radicand: usize) usize {
    const frad: f16 = @floatFromInt(radicand);
    const eps = 1e-9;
    var x: f16 = 1.0;
    while (@abs(x * x - frad) > eps) {
        x = (x + frad / x) * 0.5;
    }
    return @as(usize, @intFromFloat(x));
}
