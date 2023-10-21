local triangle = {}

function triangle.kind(a, b, c)
    assert(a > 0 and b > 0 and c > 0, "Input Error")
    assert(a + b >= c and a + c >= b and b + c >= a, "Input Error")
    if a == b and b == c then
       return "equilateral"
    elseif a ~= b and b ~= c and a ~= c then
       return "scalene"
    else
       return "isosceles"
    end
end

return triangle
