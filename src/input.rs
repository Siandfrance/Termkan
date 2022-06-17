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

// HEAVELY inspired by the termion library


use std::io::{Error, ErrorKind};
use std::str;

// use std::{fs, io};
use std::io::{Read, Write, stdin, stdout};

use std::thread;
use std::sync::mpsc;

use crate::math::Vec2;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Unsupported(Vec<u8>)
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum KeyEvent {
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    BackTab, // Shift + Tab
    Delete,
    Insert,
    F(u8),   // Only some function keys are reachable
    Char(char),
    Alt(char),
    Ctrl(char),
    Null,
    Esc
}


// TODO: add modifiers (shift, ctrl, meta) to MouseEvent
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseEvent {
    ButtonPressed(MouseButton, Vec2),
    ButtonReleased(MouseButton, Vec2),
    Hold(MouseButton, Vec2)
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    WheelUp,
    WheelDown
}



fn get_real_mouse_pos(cx: u16, cy: u16) -> Vec2 {
    vec2!(cx as i32 - 1, 2 * (cy as i32) - 2)
}


/// Parse an Event from `item` and possibly subsequent bytes through `iter`.
fn parse_event<I>(item: u8, iter: &mut I) -> Result<InputEvent, Error>
    where I: Iterator<Item = Result<u8, Error>>
{
    let error = Error::new(ErrorKind::Other, "Could not parse an event");
    match item {
        b'\x1B' => {
            // This is an escape character, leading a control sequence.
            Ok(match iter.next() {
                Some(Ok(b'O')) => {
                    match iter.next() {
                        // F1-F4
                        Some(Ok(val @ b'P'..=b'S')) => InputEvent::Key(KeyEvent::F(1 + val - b'P')),
                        _ => return Err(error),
                    }
                }
                Some(Ok(b'[')) => {
                    // This is a CSI sequence.
                    parse_csi(iter).ok_or(error)?
                }
                Some(Ok(c)) => {
                    let ch = parse_utf8_char(c, iter)?;
                    InputEvent::Key(KeyEvent::Alt(ch))
                }
                Some(Err(_)) | None => return Err(error),
            })
        }
        b'\n' | b'\r' => Ok(InputEvent::Key(KeyEvent::Char('\n'))),
        b'\t' => Ok(InputEvent::Key(KeyEvent::Char('\t'))),
        b'\x7F' => Ok(InputEvent::Key(KeyEvent::Backspace)),
        c @ b'\x01'..=b'\x1A' => Ok(InputEvent::Key(KeyEvent::Ctrl((c as u8 - 0x1 + b'a') as char))),
        c @ b'\x1C'..=b'\x1F' => Ok(InputEvent::Key(KeyEvent::Ctrl((c as u8 - 0x1C + b'4') as char))),
        b'\0' => Ok(InputEvent::Key(KeyEvent::Null)),
        c => {
            Ok({
                let ch = parse_utf8_char(c, iter)?;
                InputEvent::Key(KeyEvent::Char(ch))
            })
        }
    }
}


/// Parses a CSI sequence, just after reading ^[
///
/// Returns None if an unrecognized sequence is found.
fn parse_csi<I>(iter: &mut I) -> Option<InputEvent>
    where I: Iterator<Item = Result<u8, Error>>
{
    Some(match iter.next() {
        Some(Ok(b'[')) => match iter.next() {
            Some(Ok(val @ b'A'..=b'E')) => InputEvent::Key(KeyEvent::F(1 + val - b'A')),
            _ => return None,
        },
        Some(Ok(b'D')) => InputEvent::Key(KeyEvent::Left),
        Some(Ok(b'C')) => InputEvent::Key(KeyEvent::Right),
        Some(Ok(b'A')) => InputEvent::Key(KeyEvent::Up),
        Some(Ok(b'B')) => InputEvent::Key(KeyEvent::Down),
        Some(Ok(b'H')) => InputEvent::Key(KeyEvent::Home),
        Some(Ok(b'F')) => InputEvent::Key(KeyEvent::End),
        Some(Ok(b'Z')) => InputEvent::Key(KeyEvent::BackTab),
        Some(Ok(b'M')) => {
            // X10 emulation mouse encoding: ESC [ CB Cx Cy (6 characters only).
            let mut next = || iter.next().unwrap().unwrap();

            let cb = next() as i8 - 32;
            // (0, 0) are the coords for upper left.
            let cx = next().saturating_sub(32) as u16;
            let cy = next().saturating_sub(32) as u16;
            InputEvent::Mouse(match cb & 0b11 {
                0 => {
                    if cb & 0x40 != 0 {
                        MouseEvent::ButtonPressed(MouseButton::WheelUp, get_real_mouse_pos(cx, cy))
                    } else {
                        MouseEvent::ButtonPressed(MouseButton::Left, get_real_mouse_pos(cx, cy))
                    }
                }
                1 => {
                    if cb & 0x40 != 0 {
                        MouseEvent::ButtonPressed(MouseButton::WheelDown, get_real_mouse_pos(cx, cy))
                    } else {
                        MouseEvent::ButtonPressed(MouseButton::Middle, get_real_mouse_pos(cx, cy))
                    }
                }
                2 => MouseEvent::ButtonPressed(MouseButton::Right, get_real_mouse_pos(cx, cy)),
                // default to Left button, will be modified down the line
                3 => MouseEvent::ButtonReleased(MouseButton::Left, get_real_mouse_pos(cx, cy)),
                _ => return None,
            })
        }
        Some(Ok(b'<')) => {
            // xterm mouse encoding:
            // ESC [ < Cb ; Cx ; Cy (;) (M or m)
            let mut buf = Vec::new();
            let mut c = iter.next().unwrap().unwrap();
            while match c {
                      b'm' | b'M' => false,
                      _ => true,
                  } {
                buf.push(c);
                c = iter.next().unwrap().unwrap();
            }
            let str_buf = String::from_utf8(buf).unwrap();
            let nums = &mut str_buf.split(';');

            let cb = nums.next()
                .unwrap()
                .parse::<u16>()
                .unwrap();
            let cx = nums.next()
                .unwrap()
                .parse::<u16>()
                .unwrap();
            let cy = nums.next()
                .unwrap()
                .parse::<u16>()
                .unwrap();

            let event = match cb {
                0..=2 | 64..=65 => {
                    let button = match cb {
                        0 => MouseButton::Left,
                        1 => MouseButton::Middle,
                        2 => MouseButton::Right,
                        64 => MouseButton::WheelUp,
                        65 => MouseButton::WheelDown,
                        _ => unreachable!(),
                    };
                    match c {
                        b'M' => MouseEvent::ButtonPressed(button, get_real_mouse_pos(cx, cy)),
                        // default to Left button, will be modified down the line
                        b'm' => MouseEvent::ButtonReleased(MouseButton::Left, get_real_mouse_pos(cx, cy)),
                        _ => return None,
                    }
                }
                // default to Left button, will be modified down the line
                32 => MouseEvent::Hold(MouseButton::Left, get_real_mouse_pos(cx, cy)),
                // default to Left button, will be modified down the line
                3 => MouseEvent::ButtonReleased(MouseButton::Left, get_real_mouse_pos(cx, cy)),
                _ => return None,
            };

            InputEvent::Mouse(event)
        }
        Some(Ok(c @ b'0'..=b'9')) => {
            // Numbered escape code.
            let mut buf = Vec::new();
            buf.push(c);
            let mut c = iter.next().unwrap().unwrap();
            // The final byte of a CSI sequence can be in the range 64-126, so
            // let's keep reading anything else.
            while c < 64 || c > 126 {
                buf.push(c);
                c = iter.next().unwrap().unwrap();
            }

            match c {
                // rxvt mouse encoding:
                // ESC [ Cb ; Cx ; Cy ; M
                b'M' => {
                    let str_buf = String::from_utf8(buf).unwrap();

                    let nums: Vec<u16> = str_buf.split(';').map(|n| n.parse().unwrap()).collect();

                    let cb = nums[0];
                    let cx = nums[1];
                    let cy = nums[2];

                    let event = match cb {
                        32 => MouseEvent::ButtonPressed(MouseButton::Left, get_real_mouse_pos(cx, cy)),
                        33 => MouseEvent::ButtonPressed(MouseButton::Middle, get_real_mouse_pos(cx, cy)),
                        34 => MouseEvent::ButtonPressed(MouseButton::Right, get_real_mouse_pos(cx, cy)),
                        // default to Left button, will be modified down the line
                        35 => MouseEvent::ButtonReleased(MouseButton::Left, get_real_mouse_pos(cx, cy)),
                        // default to Left button, will be modified down the line
                        64 => MouseEvent::Hold(MouseButton::Left, get_real_mouse_pos(cx, cy)),
                        96 | 97 => MouseEvent::ButtonPressed(MouseButton::WheelUp, get_real_mouse_pos(cx, cy)),
                        _ => return None,
                    };

                    InputEvent::Mouse(event)
                }
                // Special key code.
                b'~' => {
                    let str_buf = String::from_utf8(buf).unwrap();

                    // This CSI sequence can be a list of semicolon-separated
                    // numbers.
                    let nums: Vec<u8> = str_buf.split(';').map(|n| n.parse().unwrap()).collect();

                    if nums.is_empty() {
                        return None;
                    }

                    // TODO: handle multiple values for key modififiers (ex: values
                    // [3, 2] means Shift+Delete)
                    if nums.len() > 1 {
                        return None;
                    }

                    match nums[0] {
                        1 | 7 => InputEvent::Key(KeyEvent::Home),
                        2 => InputEvent::Key(KeyEvent::Insert),
                        3 => InputEvent::Key(KeyEvent::Delete),
                        4 | 8 => InputEvent::Key(KeyEvent::End),
                        5 => InputEvent::Key(KeyEvent::PageUp),
                        6 => InputEvent::Key(KeyEvent::PageDown),
                        v @ 11..=15 => InputEvent::Key(KeyEvent::F(v - 10)),
                        v @ 17..=21 => InputEvent::Key(KeyEvent::F(v - 11)),
                        v @ 23..=24 => InputEvent::Key(KeyEvent::F(v - 12)),
                        _ => return None,
                    }
                }
                _ => return None,
            }
        }
        _ => return None,
    })
}


/// Parse `c` as either a single byte ASCII char or a variable size UTF-8 char.
fn parse_utf8_char<I>(c: u8, iter: &mut I) -> Result<char, Error>
    where I: Iterator<Item = Result<u8, Error>>
{
    let error = Err(Error::new(ErrorKind::Other, "Input character is not valid UTF-8"));
    if c.is_ascii() {
        Ok(c as char)
    } else {
        let bytes = &mut Vec::new();
        bytes.push(c);

        loop {
            match iter.next() {
                Some(Ok(next)) => {
                    bytes.push(next);
                    if let Ok(st) = str::from_utf8(bytes) {
                        return Ok(st.chars().next().unwrap());
                    }
                    if bytes.len() >= 4 {
                        return error;
                    }
                }
                _ => return error,
            }
        }
    }
}


/// Input Server Singleton instance
static mut INPUT_SERVER: Option<Input> = None;


/// The Input is a singleton that handles async io operations
/// 
/// # Usage
/// 
/// To get events from keyboard and mouse, there are two functions:
/// 
/// get_event - returns Some(InputEvent) or None depending on weather there was an input
/// 
/// get_event_blocking - waits for an event and returns it
/// 
/// # Mouse
/// 
/// To have mouse input or not use enable_mouse or disable_mouse
/// by default, there is no mouse input
pub struct Input {
    _server_handle: Option<thread::JoinHandle<()>>,
    input_recv: mpsc::Receiver<InputEvent>
}


impl Input {

    /// Creates the Input singleton, will only be called once
    fn init() -> Self {
        let (input_send, input_recv) = mpsc::channel();

        let handle = thread::spawn(move || {
            let mut mb = MouseButton::Left;
            loop {
                let mut stdin = stdin().bytes();

                if let Some(Ok(item)) = stdin.next() {
                    match parse_event(item, &mut stdin) {
                        Ok(evt) => {
                            let event = match evt {
                                InputEvent::Mouse(MouseEvent::ButtonPressed(button, _)) => {
                                    mb = button;
                                    evt
                                }
                                InputEvent::Mouse(MouseEvent::ButtonReleased(_, pos)) =>
                                    InputEvent::Mouse(MouseEvent::ButtonReleased(mb, pos)),
                                InputEvent::Mouse(MouseEvent::Hold(_, pos)) =>
                                    InputEvent::Mouse(MouseEvent::Hold(mb, pos)),
                                _ => evt
                            };
                            input_send.send(event).expect("input recv dropped")
                        }
                        Err(_) => {}
                    }
                };
            }
        });

        Self {
            _server_handle: Some(handle),
            input_recv: input_recv
        }
    }


    /// Returns the Input singleton.
    /// If no call to Input::get() is made, the server never starts;
    /// this can be usefull when custom input handling is needed.
    pub fn get() -> &'static mut Input {
        unsafe {
            match &mut INPUT_SERVER {
                None => {
                    INPUT_SERVER = Some(Input::init());
                    Input::get()
                },
                Some(i) => i
            }
        }
    }


    /// If there was an event, return it.
    /// Never blocks the current thread.
    pub fn get_event(&mut self) -> Option<InputEvent> {
        self.input_recv.try_recv().ok()
    }


    /// Wait for an InputEvent to occur and return it.
    pub fn get_event_blocking(&mut self) -> InputEvent {
        self.input_recv.recv().ok().expect("Input thread was killed")
    }


    /// Enable MouseEvent.
    pub fn enable_mouse() {
        print!("\x1b[?1000h\x1b[?1002h\x1b[?1015h\x1b[?1006h");
        stdout().flush().expect("Could not write to stdout");
    }


    /// Disable MouseEvent.
    pub fn disable_mouse() {
        print!("\x1b[?1006l\x1b[?1015l\x1b[?1002l\x1b[?1000l");
        stdout().flush().expect("Could not write to stdout");
    }
}
