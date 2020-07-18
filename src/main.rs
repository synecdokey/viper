extern crate ropey;
extern crate termion;

use std::env::args;
use std::io::{stdin, stdout, Write};

mod buffer;
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
        let buffer = Buffer::new(&filename);
        write!(stdout, "{}", termion::clear::All).unwrap();

        buffer.draw(&mut stdout);

        for c in stdin.keys() {
            // Clear the current line.
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(1, 1),
                termion::clear::CurrentLine
            )
            .unwrap();

            // Print the key we type...
            match c.unwrap() {
                // Exit.
                Key::Char('q') => break,
                _ => println!("Other"),
            }

            // Flush again.
            stdout.flush().unwrap();
        }
    }
}
