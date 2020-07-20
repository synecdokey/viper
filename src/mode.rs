use termion::color;

pub enum Mode {
    Normal,
    Visual,
    VisualBlock,
    VisualLine,
    Command,
    Insert,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} {} {}",
            color::Fg(color::Black),
            color::Bg(color::LightBlue),
            match self {
                Mode::Normal => "NORMAL",
                Mode::Visual => "VISUAL",
                Mode::VisualBlock => "V-BLOCK",
                Mode::VisualLine => "V-LINE",
                Mode::Command => "COMMAND",
                Mode::Insert => "INSERT",
            },
            termion::style::Reset
        )
    }
}

impl Mode {
    pub fn len(&self) -> usize {
        match self {
            Mode::Normal | Mode::Visual | Mode::VisualLine | Mode::Insert => 8,
            Mode::VisualBlock | Mode::Command => 9,
        }
    }
}
