local PhoneNumber = {}
PhoneNumber.__index = PhoneNumber

function PhoneNumber:new(number)
    local phone = { number = "0000000000",
                    region_code = 1,
                    area_code = 123,
                    main_code = 456,
                    other_code = 789}

    local full_number = number:gsub("[%p ]+", "")
    local len, region_code = full_number:len(), tonumber(full_number:sub(1, 1))

    if len == 10 then
        phone.number = full_number
    elseif len == 11 and region_code == 1 then
        phone.number = full_number:sub(2, #full_number)
    end

    phone.region_code = region_code
    phone.area_code = phone.number:sub(1, 3)
    phone.main_code = phone.number:sub(4, 6)
    phone.other_code = phone.number:sub(7, #phone.number)

    return setmetatable(phone, PhoneNumber)
end

function PhoneNumber:areaCode()
    return self.area_code
end

function PhoneNumber:__tostring()
    return string.format("(%s) %s-%s", self.area_code,
                            self.main_code, self.other_code)
end

return PhoneNumber
