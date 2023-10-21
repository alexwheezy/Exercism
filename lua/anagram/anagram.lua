local Anagram = {}

function Anagram:new(str)
    Anagram.str = str
    return self
end

function Anagram:match(strings)
    function sort_chars(str)
        local t = { str:byte(1, #str) }
        table.sort(t, function (lhs, rhs) return lhs < rhs end)
        return table.concat(t, "")
    end
    local result = {}
    local input_str = sort_chars(self.str:lower())
    for _, value in pairs(strings) do
        if input_str == sort_chars(value:lower()) then
            table.insert(result, value)
        end
    end
    return result
end

return Anagram
