use std::io::Stdout;
use std::io::Write;

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
            Line::LeftBottomCorner => print!("j"),
            Line::LeftTopCorner => print!("k"),
            Line::RightTopCorner => print!("l"),
            Line::RightBottomCorner => print!("m"),
            Line::Intersection => print!("n"),
            Line::Horizontal => print!("q"),
            Line::LeftIntersect => print!("t"),
            Line::RightIntersect => print!("u"),
            Line::TopIntersect => print!("v"),
            Line::BottomIntersect => print!("w"),
            Line::Vertical => print!("x"),
        }
        output.flush()?;
        Ok(())
    }
}
