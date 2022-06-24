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


use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, MulAssign, Div, DivAssign};

#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {Vec2::new($x, $y)};
}



/// Two dimentional vector. Has math operations implemented.
/// Can be instanced with the macro `vec2!`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32
}


impl Vec2 {

    pub const ZERO: Vec2 = vec2!(0, 0);
    pub const UNIX: Vec2 = vec2!(1, 0);
    pub const UNIY: Vec2 = vec2!(0, 1);


    pub const fn point(v: (i32, i32)) -> Self {
        Vec2 {
            x: v.0,
            y: v.1
        }
    }


    pub const fn new(x: i32, y: i32) -> Self {
        Vec2 {
            x: x,
            y: y
        }
    }

}



impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        vec2!(self.x + rhs.x, self.y + rhs.y)
    }
}


impl AddAssign for Vec2 {

    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}


impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        vec2!(self.x - rhs.x, self.y - rhs.y)
    }
}


impl SubAssign for Vec2 {

    fn sub_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}


impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        vec2!(self.x * rhs, self.y * rhs)
    }
}


impl MulAssign<i32> for Vec2 {

    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}


impl Div<i32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        vec2!(self.x / rhs, self.y / rhs)
    }
}


impl DivAssign<i32> for Vec2 {

    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs;
    }
}


impl AsRef<Vec2> for (u32, u32) {


    fn as_ref(&self) -> &Vec2 {
        if self.0 > i32::MAX as u32 || self.1 > i32::MAX as u32 {
            panic!("Cannot convert {:?} to Vec2, integeroverflow", self);
        }
        unsafe {
            let ptr: *const (u32, u32) = self;
            &*(ptr as *const Vec2)
        }
    }
}


impl AsRef<Vec2> for (i32, i32) {


    fn as_ref(&self) -> &Vec2 {
        unsafe {
            let ptr: *const (i32, i32) = self;
            &*(ptr as *const Vec2)
        }
    }
}


impl AsMut<Vec2> for (u32, u32) {


    fn as_mut(&mut self) -> &mut Vec2 {
        if self.0 > i32::MAX as u32 || self.1 > i32::MAX as u32 {
            panic!("Cannot convert {:?} to Vec2, integeroverflow", self);
        }
        unsafe {
            let ptr: *mut (u32, u32) = self;
            &mut *(ptr as *mut Vec2)
        }
    }
}


impl AsMut<Vec2> for (i32, i32) {


    fn as_mut(&mut self) -> &mut Vec2 {
        unsafe {
            let ptr: *mut (i32, i32) = self;
            &mut *(ptr as *mut Vec2)
        }
    }
}


impl Into<Vec2> for (i32, i32) {


    fn into(self) -> Vec2 {
        Vec2::new(self.0, self.1)
    }
}


impl Into<Vec2> for (u32, u32) {


    fn into(self) -> Vec2 {
        Vec2::new(self.0 as i32, self.1 as i32)
    }
}


impl AsRef<Vec2> for (usize, usize) {


    fn as_ref(&self) -> &Vec2 {
        if self.0 > i32::MAX as usize || self.1 > i32::MAX as usize {
            panic!("Cannot convert {:?} to Vec2, integeroverflow", self);
        }
        unsafe {
            let ptr: *const (usize, usize) = self;
            &*(ptr as *const Vec2)
        }
    }
}


impl AsRef<Vec2> for (isize, isize) {


    fn as_ref(&self) -> &Vec2 {
        unsafe {
            let ptr: *const (isize, isize) = self;
            &*(ptr as *const Vec2)
        }
    }
}


impl AsMut<Vec2> for (usize, usize) {


    fn as_mut(&mut self) -> &mut Vec2 {
        if self.0 > i32::MAX as usize || self.1 > i32::MAX as usize {
            panic!("Cannot convert {:?} to Vec2, integeroverflow", self);
        }
        unsafe {
            let ptr: *mut (usize, usize) = self;
            &mut *(ptr as *mut Vec2)
        }
    }
}


impl AsMut<Vec2> for (isize, isize) {


    fn as_mut(&mut self) -> &mut Vec2 {
        unsafe {
            let ptr: *mut (isize, isize) = self;
            &mut *(ptr as *mut Vec2)
        }
    }
}


impl Into<Vec2> for (isize, isize) {


    fn into(self) -> Vec2 {
        Vec2::new(self.0 as i32, self.1 as i32)
    }
}


impl Into<Vec2> for (usize, usize) {


    fn into(self) -> Vec2 {
        Vec2::new(self.0 as i32, self.1 as i32)
    }
}


impl AsRef<Vec2> for Vec2 {

    fn as_ref(&self) -> &Vec2 {
        self
    }
}


impl AsMut<Vec2> for Vec2 {

    fn as_mut(&mut self) -> &mut Vec2 {
        self
    }
}