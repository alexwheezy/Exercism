local function word_count(s)
    local hash_map = {}
    for word in s:lower():gmatch "%f[%w]%w+'?%w*%f[%W]" do
        hash_map[word] = (hash_map[word] or 0) + 1
    end
    return hash_map
end

return {
  word_count = word_count,
}

