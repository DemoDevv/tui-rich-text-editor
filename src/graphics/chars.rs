use crate::graphics::Drawable;

#[derive(Clone, Copy, PartialEq)]
pub struct Char {
    char: char,
}

impl Char {
    pub fn is_newline(&self) -> bool {
        self.char == '\n'
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
