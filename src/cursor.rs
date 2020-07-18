pub struct Cursor(u16, u16);

impl Cursor {
    pub fn new() -> Self {
        Cursor(1, 1)
    }

    pub fn goto_cursor(&self) -> termion::cursor::Goto {
        termion::cursor::Goto(self.0, self.1)
    }

    pub fn down(&mut self, max: u16) {
        self.1 += 1;
    }
}
