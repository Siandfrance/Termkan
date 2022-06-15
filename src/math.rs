use std::ops::{Add, Sub, AddAssign, SubAssign};


macro_rules! vec2 {
    ($x:expr, $y:expr) => {Vec2::new($x, $y)};
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32
}


impl Vec2 {

    pub const ZERO: Vec2 = Vec2{x: 0, y: 0};

    pub fn point(v: (i32, i32)) -> Self {
        Vec2 {
            x: v.0,
            y: v.1
        }
    }


    pub fn new(x: i32, y: i32) -> Self {
        Self::point((x, y))
    }

}



impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        vec2!(self.x + rhs.x, self.y + rhs.y)
    }
}


impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        vec2!(self.x - rhs.x, self.y - rhs.y)
    }
}


impl AddAssign for Vec2 {

    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}


impl SubAssign for Vec2 {

    fn sub_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}