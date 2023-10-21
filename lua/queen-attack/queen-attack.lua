return function(pos)
    assert(pos.row >= 0 and pos.row < 8 and pos.column >= 0 and pos.column < 8)
    function pos.can_attack(queen)
        if pos.row == queen.row or pos.column == queen.column then
            return true
        end
        return false
    end
    return pos
end
