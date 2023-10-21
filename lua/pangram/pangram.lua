return function(s)
    local letters = {}
    local alphabet_size = 26
    local ncount = 0
    for char in s:lower():gmatch "%a" do
       if not letters[char] then
           ncount = ncount + 1
       end
       letters[char] = (letters[char] or 0) + 1
    end
    return ncount == alphabet_size
end
