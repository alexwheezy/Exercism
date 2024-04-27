pub const ComputationError = error{IllegalArgument};

pub fn steps(number: usize) anyerror!usize {
    if (number < 1) {
        return ComputationError.IllegalArgument;
    }
    var n = number;
    var step: usize = 0;
    while (n > 1) : (step += 1) {
        if (n & 1 == 0) {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }
    }
    return step;
}
