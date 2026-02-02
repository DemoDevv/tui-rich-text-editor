pub mod lines;

/// Drawable trait for objects that can be drawn on a terminal.
pub trait Drawable {
    fn draw(&self, writer: &mut impl std::io::Write) -> Result<(), std::io::Error>;
}

#[derive(Default, Debug)]
pub struct VirtualCursor {
    pub x: i16,
    pub y: i16,
}

impl VirtualCursor {
    pub fn set_position(&mut self, x: i16, y: i16) {
        self.x = x;
        self.y = y;
    }
}

/// Buffer for storing frame data.
pub struct FrameBuffer {
    width: i16,
    height: i16,
    buffer: Vec<Cell>,
}

impl FrameBuffer {
    pub fn new(width: i16, height: i16) -> Self {
        Self {
            width,
            height,
            buffer: vec![Cell::default(); (width * height) as usize],
        }
    }

    pub fn insert(&mut self, c: char, x: i16, y: i16) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.buffer[(y * self.width + x) as usize].character = c;
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Cell {
    character: char,
}

impl Default for Cell {
    fn default() -> Self {
        Self { character: ' ' }
    }
}
