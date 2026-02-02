use std::io::Stdout;
use std::io::Write;

use crate::ESC;

pub enum Line {
    LeftBottomCorner,
    LeftTopCorner,
    RightTopCorner,
    RightBottomCorner,
    Intersection,
    Horizontal,
    LeftIntersect,
    RightIntersect,
    TopIntersect,
    BottomIntersect,
    Vertical,
}

impl Line {
    pub fn draw(&self, output: &mut Stdout) -> Result<(), std::io::Error> {
        match self {
            Line::LeftBottomCorner => print!("{}{}", ESC, "(0"),
            Line::LeftTopCorner => print!("{}{}", ESC, "(0"),
            Line::RightTopCorner => print!("{}{}", ESC, "(0"),
            Line::RightBottomCorner => print!("{}{}", ESC, "(0"),
            Line::Intersection => print!("{}{}", ESC, "(0"),
            Line::Horizontal => print!("{}{}", ESC, "(0"),
            Line::LeftIntersect => print!("{}{}", ESC, "(0"),
            Line::RightIntersect => print!("{}{}", ESC, "(0"),
            Line::TopIntersect => print!("{}{}", ESC, "(0"),
            Line::BottomIntersect => print!("{}{}", ESC, "(0"),
            Line::Vertical => print!("{}{}", ESC, "(0"),
        }
        output.flush()?;
        Ok(())
    }
}
