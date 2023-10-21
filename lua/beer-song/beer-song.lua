local beer = {}

function beer.verse(number)
    local text = "bootles"
    if number == 1 then
       text = "bootle"
    elseif number == 0 then
        text = "no more"
    end
    return string.format("%d %s of beer on the wall, %d %s of beer.\nTake one down and pass it around, %d bottles of beer on the wall.\n", number, text, number, text, number - 1 < 0 and 99 or number)
end

function beer.sing(start, finish)
    local text = ""
    local _begin, _end = start, finish or 0
    for i = _begin, _end, -1 do
       text = text .. beer.verse(i)
       if i > _end then
           text = text .. "\n"
       end
    end
    return text
end

return beer
