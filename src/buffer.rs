use std::fs::File;
use std::io::{BufReader, Stdout, Write};

use ropey::Rope;
use termion::color;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

use crate::cursor::Cursor;
use crate::mode::Mode;

pub struct Buffer<'a> {
    filename: &'a str,
    text: Rope,
    start_line: u16,
    pub cursor: Cursor,
    mode: Mode,
    limit: Cursor,
}

impl<'a> Buffer<'a> {
    pub fn new(filename: &'a str) -> Self {
        let termsize = termion::terminal_size().unwrap();
        Buffer {
            text: Rope::from_reader(BufReader::new(File::open(filename).unwrap())).unwrap(),
            filename,
            start_line: 0,
            cursor: Cursor::new(),
            mode: Mode::Normal,
            limit: Cursor::from(termsize.0, termsize.1 - 2),
        }
    }

    pub fn draw(&mut self, stdout: &mut RawTerminal<AlternateScreen<Stdout>>) {
        write!(*stdout, "{}{}", termion::cursor::Hide, termion::clear::All).unwrap();
        let termsize = termion::terminal_size().unwrap();
        let mut count = 1;

        for line in self.text.lines_at(self.start_line as usize) {
            write!(*stdout, "{}{}", termion::cursor::Goto(1, count), line).unwrap();
            if self.cursor.1 == count {
                self.limit.0 = line.len_chars() as u16;
                if self.cursor.0 > self.limit.0 {
                    self.cursor.0 = self.limit.0
                }
            }

            if count == termsize.1 - 1 || count as usize > self.text.len_lines() {
                break;
            }
            count += 1;
        }
        let percent = format!(
            "{:.0}",
            (self.line_position() as f32 / self.text.len_lines() as f32) * 100.0
        );

        let length = self.mode.len()
            + self.filename.chars().count()
            + 7
            + self.line_position().to_string().len()
            + self.cursor.0.to_string().len()
            + percent.len();
        write!(
            *stdout,
            "{}{}{} {} {} {}% {}:{} {}{}{}",
            termion::cursor::Goto(1, termsize.1 - 1),
            self.mode,
            color::Bg(color::LightBlack),
            self.filename,
            String::from_utf8(vec![' ' as u8; termsize.0 as usize - length],).unwrap(),
            percent,
            self.line_position(),
            self.cursor.0,
            self.cursor.goto_cursor(),
            termion::style::Reset,
            termion::cursor::Show
        )
        .unwrap();
    }

    fn line_position(&self) -> u16 {
        self.cursor.1 + self.start_line
    }

    pub fn right(&mut self) {
        if self.cursor.0 < self.limit.0 {
            self.cursor.right()
        }
    }

    pub fn up(&mut self) {
        if self.cursor.1 > 1 {
            self.cursor.up()
        } else if self.line_position() > 1 {
            self.start_line -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.cursor.1 < self.limit.1 {
            self.cursor.down();
        } else if (self.line_position() as usize) < self.text.len_lines() {
            self.start_line += 1;
        }
    }
}
