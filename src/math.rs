/*

    MIT License
    
    Copyright (c) 2022 Siandfrance
    
    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:
    
    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.
    
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

*/


use std::ops::{Add, Sub, AddAssign, SubAssign};


macro_rules! vec2 {
    ($x:expr, $y:expr) => {Vec2::new($x, $y)};
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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