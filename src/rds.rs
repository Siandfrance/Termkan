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


extern crate libc;

use crate::math::Vec2;
use crate::img::{Image, Color};

use termios::*;

use std::mem;

use std::io::{stdout, Write};

use std::thread;
use std::sync::{mpsc, Barrier, Arc};

use std::io::stdin;
use std::os::unix::io::AsRawFd;

const NCCS: usize = 32;


/// csi macro rule
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1b[", $( $l ),*) };
}



enum RenderingDirective {
    DrawLine(Vec2, Vec2, Color),
    DrawRect(Vec2, Vec2, Color),
    DrawRectBoudary(Vec2, Vec2, Color),
    DrawEllipseBoudary(Vec2, Vec2, Color),
    DrawPoint(Vec2, Color),

    ClearScreen(Color),

    UpdateScreenSize(Vec2),
    BeginFrame,
    PushFrame
}


pub struct Renderer {
    termios: Termios,
    default_c_lflags: u32,
    default_c_cc: [u8; NCCS],

    building_frame: bool,
    prev_screen_size: Vec2,

    _server_handle: Option<thread::JoinHandle<()>>,
    sender: mpsc::Sender<RenderingDirective>,

    frame_barrier: Arc<Barrier>
}


static mut RENDERER: Option<Renderer> = None;
static mut EXIT: bool = false;


impl Renderer {

    fn init() -> Renderer {
        let stdinfd = stdin().as_raw_fd();

        let mut termios = match Termios::from_fd(stdinfd) {
            Ok(t)  => t,
            Err(_) => panic!("Could not read stdin fd")
        };

        // save and update settings
        let default_c_lflags = termios.c_lflag;
        let default_c_cc = termios.c_cc;

        termios.c_lflag &= !(ECHO | ICANON | ISIG);
        termios.c_cc[VMIN] = 1;
        termios.c_cc[VTIME] = 0;

        tcsetattr(stdinfd, TCSANOW, &mut termios).expect("could not set stdin attributes");
        
        print!("{}{}{}", 
            csi!("?25l"),                                   // hide cursor
            csi!("?1049h"),                                 // use alternate screen buffer
            "\x1b[?1000h\x1b[?1002h\x1b[?1015h\x1b[?1006h"  // mouse support
        );
        stdout().flush().expect("Could not write to stdout"); 

        // setup and start server
        let (rx, tx) = mpsc::channel();
        let barrier = Arc::new(Barrier::new(2));
        let frame_barrier = Arc::clone(&barrier);

        let handle = thread::spawn(move || {
            let mut screen_size = Renderer::get_size();
            let mut screen: Image = Image::new(0, 0);

            let mut back: Color = Color::BLACK;
            let mut fore: Color = Color::BLACK;
            print!("{:-}{:+}", back, fore);


            loop {
                match tx.recv().expect("RenderingServer channel was destroyed") {
                    RenderingDirective::DrawLine(p1, p2, c) => screen.line(p1, p2, c),
                    RenderingDirective::DrawRect(p, s, c) => screen.rect(p, s, c),
                    RenderingDirective::DrawRectBoudary(p, s, c) => screen.rect_boudary(p, s, c),
                    RenderingDirective::DrawEllipseBoudary(center, s, c) => screen.ellipse_boundary(center, s, c),
                    RenderingDirective::DrawPoint(p, c) => screen.point(p, c),

                    RenderingDirective::ClearScreen(c) => screen.clear(c),

                    RenderingDirective::UpdateScreenSize(size) => {
                        screen_size = size;
                        screen.resize(size.x as usize, size.y as usize);
                    }

                    RenderingDirective::BeginFrame => {frame_barrier.wait(); ()},
                    RenderingDirective::PushFrame => {
                        // position cursor
                        print!("\x1b[H");
                        for j in (0..screen_size.y).step_by(2) {
                            for i in 0..screen_size.x {
                                let pos1 = Vec2::new(i, j);
                                let pos2 = Vec2::new(i, j + 1);
                                
                                // update color
                                if screen[pos1] != back && screen[pos1] != fore && screen[pos2] == back {
                                    fore = screen[pos1];
                                    print!("{:+}", fore);
                                } else if screen[pos1] != back && screen[pos1] != fore && screen[pos2] == fore {
                                    back = screen[pos1];
                                    print!("{:-}", back);
                                } else if screen[pos2] != back && screen[pos2] != fore && screen[pos1] == back {
                                    fore = screen[pos2];
                                    print!("{:+}", fore);
                                } else if screen[pos2] != back && screen[pos2] != fore && screen[pos1] == fore {
                                    back = screen[pos2];
                                    print!("{:-}", back);
                                } else if screen[pos1] != back && screen[pos1] != fore && screen[pos2] != back && screen[pos2] != fore {
                                    fore = screen[pos1];
                                    back = screen[pos2];
                                    print!("{:+}", fore);
                                    print!("{:-}", back);
                                }

                                // print pixel
                                if screen[pos1] == back && screen[pos2] == back {
                                    print!(" ");
                                } else if screen[pos1] == back && screen[pos2] == fore {
                                    print!("▄");
                                } else if screen[pos1] == fore && screen[pos2] == back {
                                    print!("▀");
                                } else if screen[pos1] == fore && screen[pos2] == fore {
                                    print!("█");
                                }
                            }
                        }
                        stdout().flush().expect("Could not write to stdout");
                    }
                }
            }
        });

        Renderer {
            termios: termios,
            default_c_lflags: default_c_lflags,
            default_c_cc: default_c_cc,

            building_frame: false,
            prev_screen_size: Vec2::ZERO,

            _server_handle: Some(handle),
            sender: rx,

            frame_barrier: barrier
        }
    }


    pub fn exit() {
        unsafe {
            EXIT = true;
            RENDERER = None;
        }
    }


    pub fn get() -> &'static mut Renderer {
        unsafe {
            if EXIT {
                panic!("Ty to get Renderer after exit");
            }
            match &mut RENDERER {
                None => { // construct the renderer, and initialize
                    RENDERER = Some(Renderer::init());
                    Renderer::get()
                }
                Some(r) => r
            }
        }
    }


    pub fn get_size() -> Vec2 {
        unsafe {
            let mut size: TermSize = mem::zeroed();
            libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut size as *mut _);
            Vec2::new(size.col as i32, 2 * size.row as i32)
        }
    }

    
    /// panics if we are not in a draw loop
    fn can_draw(&self) {
        if !self.building_frame { panic!("drawing outside of a frame build (call begin_draw)"); }
    }


    pub fn begin_draw(&mut self) {
        if self.building_frame {
            panic!("begin_draw called when already building a frame");
        }
        self.building_frame = true;
        let new_size = Renderer::get_size();
        if self.prev_screen_size != new_size {
            self.sender.send(RenderingDirective::UpdateScreenSize(new_size)).expect("Rendering thread stoped");
            self.prev_screen_size = new_size;
        }

        self.sender.send(RenderingDirective::BeginFrame).expect("Rendering thread stoped");
        self.frame_barrier.wait();
    }


    pub fn end_draw(&mut self) {
        if !self.building_frame {
            panic!("end_draw called when already building a frame");
        }
        self.building_frame = false;
        self.sender.send(RenderingDirective::PushFrame).expect("Rendering thread stoped");
    }


    pub fn clear_screen(&mut self, c: Color) {
        self.can_draw();
        self.sender.send(RenderingDirective::ClearScreen(c)).expect("Rendering thread stoped");
    }


    pub fn draw_line(&mut self, p1: Vec2, p2: Vec2, c: Color) {
        self.can_draw();
        self.sender.send(RenderingDirective::DrawLine(p1, p2, c)).expect("Rendering thread stoped");
    }


    pub fn draw_rect(&mut self, p: Vec2, s: Vec2, c: Color) {
        self.can_draw();
        self.sender.send(RenderingDirective::DrawRect(p, s, c)).expect("Rendering thread stoped");
    }


    pub fn draw_rect_boundary(&mut self, p: Vec2, s: Vec2, c: Color) {
        self.can_draw();
        self.sender.send(RenderingDirective::DrawRectBoudary(p, s, c)).expect("Rendering thread stoped");
    }


    pub fn draw_ellipse_boundary(&mut self, c: Vec2, s: Vec2, col: Color) {
        self.can_draw();
        self.sender.send(RenderingDirective::DrawEllipseBoudary(c, s, col)).expect("Rendering thread stoped");
    }


    pub fn draw_point(&mut self, p: Vec2, c: Color) {
        self.can_draw();
        self.sender.send(RenderingDirective::DrawPoint(p, c)).expect("Rendering thread stoped");
    }
}


impl Drop for Renderer {

    fn drop(&mut self) {
        // return settings to default
        self.termios.c_cc = self.default_c_cc;
        self.termios.c_lflag = self.default_c_lflags;

        print!("{}{}{}",
            csi!("?25h"),                                   // show cursor
            csi!("?1049l"),                                 // use main screen buffer
            "\x1b[?1006l\x1b[?1015l\x1b[?1002l\x1b[?1000l"  // mouse support
        );
        stdout().flush().expect("Could not write to stdout");

        std::process::exit(0);
    }
}


struct TermSize {
    // should be c_short
    row: libc::c_ushort,
    col: libc::c_ushort,
    _x : libc::c_ushort,
    _y : libc::c_ushort
}