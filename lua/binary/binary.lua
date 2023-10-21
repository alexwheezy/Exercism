local function to_decimal(input)
    local result = 0
    local length = #input
    for n = length, 1, -1 do
       local dec = tonumber(input:sub(length + 1 - n, length + 1 - n))
       if not dec then return 0 end
       result = result + (dec * 2 ^ (n - 1))
    end
    return result
end

return {
  to_decimal = to_decimal
}
