use crate::graphics::Drawable;

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

impl Drawable for Line {
    fn draw(&self, writer: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        let symbol = match self {
            Line::LeftBottomCorner => "└",
            Line::LeftTopCorner => "┌",
            Line::RightTopCorner => "┐",
            Line::RightBottomCorner => "┘",
            Line::Intersection => "┼",
            Line::Horizontal => "─",
            Line::LeftIntersect => "├",
            Line::RightIntersect => "┤",
            Line::TopIntersect => "┬",
            Line::BottomIntersect => "┴",
            Line::Vertical => "│",
        };
        write!(writer, "{}", symbol)
    }
}
