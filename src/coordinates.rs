pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Coordinates {
    pub fn new() -> Self {
        Coordinates { x: 1, y: 1 }
    }

    pub fn from(x: u16, y: u16) -> Self {
        Coordinates { x, y }
    }

    pub fn goto_cursor(&self) -> termion::cursor::Goto {
        termion::cursor::Goto(self.x, self.y)
    }

    pub fn left(&mut self) {
        self.x -= 1;
    }

    pub fn down(&mut self) {
        self.y += 1;
    }

    pub fn up(&mut self) {
        if self.y > 1 {
            self.y -= 1;
        }
    }

    pub fn right(&mut self) {
        self.x += 1;
    }
}
