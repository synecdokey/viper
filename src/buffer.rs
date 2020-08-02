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
    multiline_count: u16,
    x_memory: u16,
    pub cursor: Coordinates,
    mode: Mode,
    line_len: u16,
    lower_limit: Coordinates,
    upper_limit: Coordinates,
}

static MIN_CURSOR: u16 = 1;

impl<'a> Buffer<'a> {
    pub fn new(filename: &'a str) -> Self {
        let termsize = termion::terminal_size().unwrap();
        let text = Rope::from_reader(BufReader::new(File::open(filename).unwrap())).unwrap();
        let len = &text.len_lines().to_string().len() + 1 + MIN_CURSOR as usize;
        Buffer {
            text,
            filename,
            start_line: 0,
            line_len: 0,
            x_memory: len as u16,
            multiline_count: 0,
            cursor: Coordinates::from(len as u16, 1),
            mode: Mode::Normal,
            lower_limit: Coordinates::from(len as u16, 1),
            upper_limit: Coordinates::from(termsize.0, termsize.1 - 2),
        }
    }

    pub fn draw(&mut self, stdout: &mut RawTerminal<AlternateScreen<Stdout>>) {
        // First, clear everything
        write!(*stdout, "{}{}", termion::cursor::Hide, termion::clear::All).unwrap();
        self.resize(termion::terminal_size().unwrap());

        let max_line_size = (self.upper_limit.x - self.lower_limit.x) as usize;
        let mut count = 1;
        self.multiline_count = 0;

        for line in self.text.lines_at(self.start_line as usize) {
            let line_number_diff = self.lower_limit.x as usize
                - (self.start_line + count).to_string().len()
                - 1
                - MIN_CURSOR as usize;
            let line_number_str = String::from_utf8(vec![' ' as u8; line_number_diff]).unwrap();

            let mut start = 0;
            while line.len_chars() - start >= max_line_size {
                write!(
                    *stdout,
                    "{}{}{} {}",
                    termion::cursor::Goto(1, count + self.multiline_count),
                    &line_number_str,
                    self.start_line + count,
                    line.slice(start..=max_line_size)
                )
                .unwrap();
                start += max_line_size as usize + 1;
                self.multiline_count += 1;
            }
            if start > 0 && line.chars_at(start).next().unwrap_or('\n') == '\n' {
                self.multiline_count -= 1;
            } else {
                write!(
                    *stdout,
                    "{}{}{} {}",
                    termion::cursor::Goto(1, count + self.multiline_count),
                    &line_number_str,
                    self.start_line + count,
                    line.slice(start..)
                )
                .unwrap();
            }

            // Normalise cursor position
            if self.cursor.y == count {
                // TODO: understand why I need to substract
                // Is it counting the \n?
                self.line_len = line.len_chars() as u16 + self.lower_limit.x - 1;

                if self.x_memory < self.line_len {
                    self.cursor.x = self.x_memory
                } else {
                    self.cursor.x = self.line_len
                }
            }

            if count + self.multiline_count == self.upper_limit.y
                || count as usize > self.text.len_lines()
            {
                break;
            }

            count += 1;
        }
    }

    pub fn draw_modeline(&self, stdout: &mut RawTerminal<AlternateScreen<Stdout>>) {
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
            termion::cursor::Goto(1, self.upper_limit.y + 1),
            self.mode,
            color::Bg(color::LightBlack),
            self.filename,
            String::from_utf8(vec![
                ' ' as u8;
                termion::terminal_size().unwrap().0 as usize - length
            ])
            .unwrap(),
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

    pub fn resize(&mut self, termsize: (u16, u16)) {
        self.upper_limit = Coordinates::from(termsize.0, termsize.1 - 2)
    }

    pub fn left(&mut self) {
        if self.cursor.x > self.lower_limit.x {
            self.cursor.left();
            self.x_memory = self.cursor.x;
        }
    }

    pub fn right(&mut self) {
        if self.cursor.x < self.line_len {
            self.cursor.right();
            self.x_memory = self.cursor.x;
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
