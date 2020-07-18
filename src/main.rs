extern crate ropey;
extern crate termion;

use std::env::args;
use std::io::{stdin, stdout, Write};

mod buffer;
mod cursor;
use crate::buffer::Buffer;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    {
        let mut stdout = termion::screen::AlternateScreen::from(stdout())
            .into_raw_mode()
            .unwrap();
        let mut arguments = args();

        let filename = arguments.nth(1).unwrap();
        let mut buffer = Buffer::new(&filename);
        write!(stdout, "{}", termion::clear::All).unwrap();

        buffer.draw(&mut stdout);
        stdout.flush().unwrap();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('j') => {
                    buffer.cursor.down(2);
                }
                // Exit.
                Key::Char('q') => break,
                _ => {}
            }

            buffer.draw(&mut stdout);
            // Flush again.
            stdout.flush().unwrap();
        }
    }
}
