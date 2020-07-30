use std::fs::File;
use std::io::{BufReader, Stdout, Write};

use ropey::Rope;
use termion::color;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

use crate::coordinates::Coordinates;
use crate::mode::Mode;

pub struct Buffer<'a> {
    filename: &'a str,
    text: Rope,
    start_line: u16,
    start_char: usize,
    pub cursor: Coordinates,
    mode: Mode,
    lower_limit: Coordinates,
    upper_limit: Coordinates,
}



impl<'a> Buffer<'a> {
    pub fn new(filename: &'a str) -> Self {
        let termsize = termion::terminal_size().unwrap();
        let text = Rope::from_reader(BufReader::new(File::open(filename).unwrap())).unwrap();
        let len = &text.len_lines().to_string().len() + 2;
        Buffer {
            start_char: len,
            text,
            filename,
            start_line: 0,
            cursor: Coordinates::from(len as u16, 1),
            mode: Mode::Normal,
            lower_limit: Coordinates::from(len as u16, 1),
            upper_limit: Coordinates::from(termsize.0, termsize.1 - 2),
        }
    }

    pub fn draw(&mut self, stdout: &mut RawTerminal<AlternateScreen<Stdout>>) {
        write!(*stdout, "{}{}", termion::cursor::Hide, termion::clear::All).unwrap();
        let termsize = termion::terminal_size().unwrap();
        let mut count = 1;

        for line in self.text.lines_at(self.start_line as usize) {
            let line_number_diff =
                self.start_char - (self.start_line + count).to_string().len() - 2;
            let line_number_str = match line_number_diff {
                0 => vec![],
                _ => vec![' ' as u8; line_number_diff],
            };

            write!(
                *stdout,
                "{}{}{} {}",
                termion::cursor::Goto(1, count),
                String::from_utf8(line_number_str).unwrap(),
                self.start_line + count,
                line
            )
            .unwrap();

            if self.cursor.y == count {
                self.upper_limit.x = line.len_chars() as u16;
                if self.cursor.x > (self.upper_limit.x + self.start_char as u16 - 1) {
                    self.cursor.x = self.upper_limit.x + self.start_char as u16 - 1
                }
                if self.cursor.x < self.lower_limit.x {
                    self.cursor.x = self.lower_limit.x
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
            + self.cursor.x.to_string().len()
            + percent.len();

        write!(
            *stdout,
            "{}{}{} {} {} {}% {}:{} {}{}{}",
            termion::cursor::Goto(1, termsize.1 - 1),
            self.mode,
            color::Bg(color::LightBlack),
            self.filename,
            String::from_utf8(vec![' ' as u8; termsize.0 as usize - length]).unwrap(),
            percent,
            self.line_position(),
            self.cursor.x,
            self.cursor.goto_cursor(),
            termion::style::Reset,
            termion::cursor::Show
        )
        .unwrap();
    }

    fn line_position(&self) -> u16 {
        self.cursor.y + self.start_line
    }

    pub fn left(&mut self) {
        if self.cursor.x as usize > self.start_char {
            self.cursor.left()
        }
    }

    pub fn right(&mut self) {
        if self.cursor.x < self.upper_limit.x + self.start_char as u16 - 2 {
            self.cursor.right()
        }
    }

    pub fn up(&mut self) {
        if self.cursor.y > self.lower_limit.y {
            self.cursor.up()
        } else if self.line_position() > self.lower_limit.y {
            self.start_line -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.cursor.y < self.upper_limit.y
            && (self.line_position() as usize) < self.text.len_lines()
        {
            self.cursor.down();
        } else if (self.line_position() as usize) < self.text.len_lines() {
            self.start_line += 1;
        }
    }
}
