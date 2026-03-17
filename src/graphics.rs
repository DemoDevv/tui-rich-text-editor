use std::collections::HashMap;

use crate::{COORD, NORM_COORD, graphics::chars::Char};

pub mod chars;
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
    tracking_buffer: HashMap<COORD, Cell>,
}

impl FrameBuffer {
    pub fn new(width: i16, height: i16) -> Self {
        Self {
            width,
            height,
            tracking_buffer: HashMap::new(),
        }
    }

    pub fn insert(&mut self, c: Char, x: i16, y: i16) {
        if !(x >= 0 && x < self.width && y >= 0 && y < self.height) {
            return;
        }

        self.tracking_buffer
            .insert(COORD { x, y }, Cell { character: c });
    }

    pub fn changes(&self) -> Vec<(COORD, Cell)> {
        self.tracking_buffer
            .iter()
            .map(|(&coord, &cell)| (coord, cell))
            .collect()
    }

    pub fn clear(&mut self) {
        self.tracking_buffer.clear();
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    pub character: Char,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            character: Char::from(' '),
        }
    }
}

pub fn transform_normal_coord_to_terminal_coord(
    norm_coord: NORM_COORD,
    width: f32,
    height: f32,
) -> COORD {
    COORD {
        x: ((norm_coord.x as f32 + 1.) / 2. * width) as i16,
        y: ((1. - (norm_coord.y as f32 + 1.) / 2.) * height) as i16,
    }
}
