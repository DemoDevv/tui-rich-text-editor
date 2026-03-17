use crate::graphics::Drawable;

#[derive(Clone, Copy, PartialEq)]
pub struct Char {
    char: char,
}

impl Char {
    pub fn is_newline(&self) -> bool {
        // Inn rawmode, we only have '\r' for newline
        self.char == '\r'
    }

    pub fn is_delete(&self) -> bool {
        self.char == '\x7f' || self.char == '\x08'
    }
}

impl From<char> for Char {
    fn from(char: char) -> Self {
        Char { char }
    }
}

impl Drawable for Char {
    fn draw(&self, writer: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        write!(writer, "{}", self.char)
    }
}
