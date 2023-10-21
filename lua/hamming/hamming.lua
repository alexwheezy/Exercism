local Hamming = {}

function Hamming.compute(a, b)
    if #a ~= #b then return -1 end
    local ncount = 0
    for i = 1, #a, 1 do
       ncount = ncount + (a:byte(i) ~= b:byte(i) and 1 or 0)
    end
    return ncount
end

return Hamming
