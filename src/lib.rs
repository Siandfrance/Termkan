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
