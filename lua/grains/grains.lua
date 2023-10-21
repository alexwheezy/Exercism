local grains = {}

function grains.square(n)
    return 1 << (n - 1)
end

function grains.total()
    return 1 << 64
end

return grains
