local s = string.gsub("duplicates", "[^%w+]", "")
local b = {s:byte(1, #s)}
table.sort(b, function (n1, n2)
   return n1 < n2
end)

print(string.char(b, 1, #b))
