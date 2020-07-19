use std::fs::File;
use std::io::{BufReader, Stdout, Write};

use ropey::Rope;
use termion::color;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

use crate::cursor::Cursor;

pub struct Buffer<'a> {
    filename: &'a str,
    text: Rope,
    pub cursor: Cursor,
    limit: Cursor,
}

impl<'a> Buffer<'a> {
    pub fn new(filename: &'a str) -> Self {
        let termsize = termion::terminal_size().unwrap();
        Buffer {
            text: Rope::from_reader(BufReader::new(File::open(filename).unwrap())).unwrap(),
            filename,
            cursor: Cursor::new(),
            limit: Cursor::from(termsize.0, termsize.1 - 1),
        }
    }

    pub fn draw(&mut self, stdout: &mut RawTerminal<AlternateScreen<Stdout>>) {
        write!(*stdout, "{}", termion::cursor::Hide).unwrap();
        let termsize = termion::terminal_size().unwrap();
        let mut count = 1;
        for line in self.text.lines() {
            write!(*stdout, "{}{}", termion::cursor::Goto(1, count), line).unwrap();
            count += 1;
            if self.cursor.1 == count - 1 {
                self.limit.0 = line.len_chars() as u16;
                if self.cursor.0 > self.limit.0 {
                    self.cursor.0 = self.limit.0
                }
            }
            if count == termsize.1 - 1 {
                break;
            }
        }

        write!(
            *stdout,
            "{}{}{} {} {}{}{}{}",
            termion::cursor::Goto(1, termsize.1 - 1),
            color::Fg(color::Black),
            color::Bg(color::LightCyan),
            self.filename,
            String::from_utf8(vec![
                ' ' as u8;
                termsize.0 as usize - self.filename.chars().count() - 2
            ],)
            .unwrap(),
            self.cursor.goto_cursor(),
            termion::style::Reset,
            termion::cursor::Show
        )
        .unwrap();
    }

    pub fn right(&mut self) {
        if self.cursor.0 < self.limit.0 {
            self.cursor.right()
        }
    }
}
