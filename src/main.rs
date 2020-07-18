extern crate ropey;
extern crate termion;

use ropey::Rope;
use std::env::args;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    {
        let mut stdout = termion::screen::AlternateScreen::from(stdout()).into_raw_mode().unwrap();
        let mut arguments = args();

        let filename = arguments.nth(1).unwrap();
        let text = Rope::from_reader(BufReader::new(File::open(&filename).unwrap()));
        write!(stdout, "{}", termion::clear::All).unwrap();

        let termsize = termion::terminal_size().ok().unwrap();
        let mut count = 1;
        for line in text.unwrap().lines() {
            write!(stdout, "{}{}", termion::cursor::Goto(1, count), line).unwrap();
            count += 1;
            if count == termsize.1 - 2 {
                break;
            }
        }

        write!(
            stdout,
            "{}{}{}{}{}",
            termion::cursor::Goto(1, termsize.1 - 1),
            color::Fg(color::Black),
            color::Bg(color::LightCyan),
            filename,
            String::from_utf8(vec![
                ' ' as u8;
                termsize.0 as usize - filename.chars().count()
            ])
            .unwrap(),
        ).unwrap();
        // Flush stdout (i.e. make the output appear).
        stdout.flush().unwrap();

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

        // Show the cursor again before we exit.
        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Show).unwrap();
    }
}
