extern crate ropey;
extern crate termion;

use std::env::args;
use std::io::{stdin, stdout, Write};

mod buffer;
mod coordinates;
mod mode;
use crate::buffer::Buffer;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    {
        let mut stdout = termion::screen::AlternateScreen::from(stdout())
            .into_raw_mode()
            .unwrap();
        let mut arguments = args();

        let filename = arguments.nth(1).unwrap();
        let mut buffer = Buffer::new(&filename);
        write!(stdout, "{}", termion::clear::All).unwrap();

        buffer.draw(&mut stdout);
        buffer.draw_modeline(&mut stdout);
        stdout.flush().unwrap();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('h') => {
                    buffer.left();
                }
                Key::Char('j') => {
                    buffer.down();
                }
                Key::Char('k') => {
                    buffer.up();
                }
                Key::Char('l') => {
                    buffer.right();
                }
                // Exit.
                Key::Char('q') => break,
                _ => {}
            }

            buffer.draw(&mut stdout);
            buffer.draw_modeline(&mut stdout);
            // Flush again.
            stdout.flush().unwrap();
        }
    }
}
