return function(input)
    local matrix = { rows = {}, columns = {} }
    local i = 1
    for lines in input:gmatch "[^\r\n]+" do
        matrix.rows[i] = {}
        local j = 1
        for value in lines:gmatch "%d+" do
            local element = tonumber(value)
            matrix.rows[i][j] = element
            matrix.columns[j] = matrix.columns[j] or {}
            matrix.columns[j][i] = element
            j = j + 1
        end
        i = i + 1
    end

    matrix.row = function (index)
       return matrix.rows[index]
    end

    matrix.column = function (index)
        return matrix.columns[index]
    end

    return matrix
end
