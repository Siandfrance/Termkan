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


/* 
    Credit to `redox-os` for the termion crate that helped with the design of this library
    termion gitlab: https://gitlab.redox-os.org/redox-os/termion
*/


extern crate termios;


#[macro_use]
pub mod math;
pub mod img;

pub mod rds;
pub mod input;




#[cfg(test)]
mod tests {

    use crate::rds::Renderer;
    use std::time::Duration;
    use std::thread::sleep;

    use crate::math::Vec2;
    use crate::img::*;
    use crate::input::{Input, InputEvent, KeyEvent, MouseEvent};


    #[test]
    fn renderer() {
        let rdr = Renderer::get();

        rdr.begin_draw();
        rdr.draw_line(vec2!(2, 7), vec2!(28, 6), Color::WHITE);
        rdr.draw_rect(vec2!(40, 15), vec2!(15, -10), Color::RED);
        rdr.draw_rect_boundary(vec2!(40, 15), vec2!(15, -10), Color::CHOCOLATE);
        rdr.draw_ellipse_boundary(vec2!(45, 25), vec2!(25, 8), Color::AQUAMARINE);

        rdr.draw_ellipse_boundary(vec2!(60, 30), vec2!(4, 4), Color::DEEP_PINK);

        rdr.end_draw();
        sleep(Duration::from_secs(2));

        Renderer::exit();
    }


    #[test]
    fn input() {
        let rdr = Renderer::get();
        let inp = Input::get();
        Input::enable_mouse();

        let mut pos = Renderer::get_size() / 2;

        loop {
            let size = Renderer::get_size();

            // manage input
            match inp.get_event() {
                Some(event) => {
                    match event {
                    InputEvent::Key(event) => match event {
                        KeyEvent::Ctrl('c') => Renderer::exit(),
                        KeyEvent::Up        => if pos.y >  1            {pos.y -= 1},
                        KeyEvent::Down      => if pos.y <= size.y - 2   {pos.y += 1},
                        KeyEvent::Left      => if pos.x >  1            {pos.x -= 1},
                        KeyEvent::Right     => if pos.x <= size.x - 2   {pos.x += 1},
                        _ => ()
                    }
                    InputEvent::Mouse(event) => match event {
                        MouseEvent::ButtonPressed(_, mpos) | MouseEvent::Hold(_, mpos)
                            => pos = mpos,
                        _ => ()
                    }
                    _ => ()
                }
                }
                None => ()
            };

            // draw on screen
            rdr.begin_draw();
            rdr.clear_screen(Color::BLACK);
            rdr.draw_rect_boundary(Vec2::ZERO, size - vec2!(1, 1), Color::BROWN);
            rdr.draw_point(pos, Color::WHITE);
            rdr.end_draw();
        }
    }
}
