pub fn binarySearch(
    comptime T: type,
    comptime target: T,
    items: []const T,
) ?usize {
    if (items.len == 0) return null;
    var left: usize = 0;
    var right = items.len;
    while (left < right) {
        const mid = (right + left) / 2;
        if (items[mid] < target) {
            left = mid + 1;
        } else if (items[mid] > target) {
            right = mid;
        } else if (items[mid] == target) {
            return mid;
        }
    }
    return null;
}
