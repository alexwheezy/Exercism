use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Copy + PartialEq + PartialOrd + Add<Output = T> + From<i32>,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if !sides.iter().all(|s| s > &T::from(0)) {
            return None;
        }
        let mut sides = sides;
        sides.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        if sides[0] + sides[1] > sides[2] {
            Some(Self { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1] && self.sides[1] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] || self.sides[1] == self.sides[2]
    }
}
