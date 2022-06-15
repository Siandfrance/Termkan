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


extern crate termios;


#[macro_use]
pub mod math;
pub mod img;

pub mod rds;




#[cfg(test)]
mod tests {

    use crate::rds::Renderer;
    use std::time::Duration;
    use std::thread::sleep;
    use crate::math::Vec2;
    use crate::img::*;


    #[test]
    fn it_works() {
        let rdr = Renderer::get();

        rdr.begin_draw();
        rdr.draw_line(vec2!(2, 7), vec2!(28, 6), Color::WHITE);
        rdr.draw_rect(vec2!(40, 15), vec2!(15, -10), Color::RED);
        rdr.draw_rect_boundary(vec2!(40, 15), vec2!(15, -10), Color::CHOCOLATE);
        rdr.draw_ellipse_boundary(vec2!(45, 25), vec2!(25, 8), Color::AQUAMARINE);

        rdr.end_draw();
        sleep(Duration::from_secs(2));

        Renderer::exit();
    }
}
