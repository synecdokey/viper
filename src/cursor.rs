pub struct Cursor(pub u16, pub u16);

impl Cursor {
    pub fn new() -> Self {
        Cursor(1, 1)
    }

    pub fn from(x: u16, y: u16) -> Self {
        Cursor(x, y)
    }

    pub fn goto_cursor(&self) -> termion::cursor::Goto {
        termion::cursor::Goto(self.0, self.1)
    }

    pub fn left(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    pub fn down(&mut self) {
        self.1 += 1;
    }

    pub fn up(&mut self) {
        if self.1 > 0 {
            self.1 -= 1;
        }
    }

    pub fn right(&mut self) {
        self.0 += 1;
    }
}
