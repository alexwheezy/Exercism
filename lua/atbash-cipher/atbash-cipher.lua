local letters = {
    ['a'] = 'z', ['b'] = 'y', ['c'] = 'x',
    ['d'] = 'w', ['e'] = 'v', ['f'] = 'u',
    ['g'] = 't', ['h'] = 's', ['i'] = 'r',
    ['j'] = 'q', ['k'] = 'p', ['l'] = 'o',
    ['m'] = 'n', ['n'] = 'm', ['o'] = 'l',
    ['p'] = 'k', ['q'] = 'j', ['r'] = 'i',
    ['s'] = 'h', ['t'] = 'g', ['u'] = 'f',
    ['v'] = 'e', ['w'] = 'd', ['x'] = 'c',
    ['y'] = 'b', ['z'] = 'a',
}

return {
  encode = function(plaintext)
      local text = ""
      local chars = plaintext:lower():gsub("[ ,.]", "")
      for i = 1, #chars, 1 do
          local char = chars:sub(i, i)
          text = text .. (letters[char] or char)
          if i % 5 == 0 and i < #chars - 1 then
              text = text .. " "
          end
      end
      return text
  end
}
