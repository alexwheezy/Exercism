local function square_of_sum(n)
    return (n * 0.5 * (n + 1)) ^ 2
end

local function sum_of_squares(n)
    return (n * 0.5 * (n + 1)) * ((2 * n + 1) / 3)
end

local function difference_of_squares(n)
    return math.abs(square_of_sum(n) - sum_of_squares(n))
end

return {
  square_of_sum = square_of_sum,
  sum_of_squares = sum_of_squares,
  difference_of_squares = difference_of_squares
}
