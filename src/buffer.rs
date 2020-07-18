use std::fs::File;
use std::io::{BufReader, Stdout, Write};

use ropey::Rope;
use termion::color;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

pub struct Buffer<'a> {
    filename: &'a str,
    text: Rope,
}

impl<'a> Buffer<'a> {
    pub fn new(filename: &'a str) -> Self {
        Buffer {
            text: Rope::from_reader(BufReader::new(File::open(filename).unwrap())).unwrap(),
            filename,
        }
    }

    pub fn draw(self, stdout: &mut RawTerminal<AlternateScreen<Stdout>>) {
        let termsize = termion::terminal_size().unwrap();
        let mut count = 1;
        for line in self.text.lines() {
            write!(*stdout, "{}{}", termion::cursor::Goto(1, count), line).unwrap();
            count += 1;
            if count == termsize.1 - 2 {
                break;
            }
        }

        write!(
            *stdout,
            "{}{}{}{}{}{}",
            termion::cursor::Goto(1, termsize.1 - 1),
            color::Fg(color::Black),
            color::Bg(color::LightCyan),
            self.filename,
            String::from_utf8(vec![
                ' ' as u8;
                termsize.0 as usize - self.filename.chars().count()
            ],)
            .unwrap(),
            termion::cursor::Goto(1, 1)
        )
        .unwrap();
        // Flush stdout (i.e. make the output appear).
        stdout.flush().unwrap();
    }
}
