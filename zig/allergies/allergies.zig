const std = @import("std");
const EnumSet = std.EnumSet;

pub const Allergen = enum(u8) {
    eggs = 1,
    peanuts = 2,
    shellfish = 4,
    strawberries = 8,
    tomatoes = 16,
    chocolate = 32,
    pollen = 64,
    cats = 128,
};

pub fn isAllergicTo(score: u8, allergen: Allergen) bool {
    return (score & @intFromEnum(allergen)) != 0;
}

pub fn initAllergenSet(score: usize) EnumSet(Allergen) {
    var set = EnumSet(Allergen).initEmpty();
    for (std.enums.values(Allergen)) |allergen| {
        if (@intFromEnum(allergen) & score != 0) {
            set.insert(allergen);
        }
    }
    return set;
}
