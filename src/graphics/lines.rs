//! Drawing primitives for TUI borders and boxes.
//!
//! This module provides the [`Line`] enum which maps Unicode Box Drawing
//! characters to logical border components.

use crate::graphics::Drawable;

/// Represents the various types of border characters using Unicode "Box Drawing" symbols.
///
/// These variants are used to construct frames, tables, or visual separators
/// within a TUI (Terminal User Interface).
pub enum Line {
    /// Lower-left corner: '└'
    LeftBottomCorner,
    /// Upper-left corner: '┌'
    LeftTopCorner,
    /// Upper-right corner: '┐'
    RightTopCorner,
    /// Lower-right corner: '┘'
    RightBottomCorner,
    /// Center intersection (cross): '┼'
    Intersection,
    /// Horizontal line: '─'
    Horizontal,
    /// T-junction pointing right: '├'
    LeftIntersect,
    /// T-junction pointing left: '┤'
    RightIntersect,
    /// T-junction pointing down: '┬'
    TopIntersect,
    /// T-junction pointing up: '┴'
    BottomIntersect,
    /// Vertical line: '│'
    Vertical,
}

impl Drawable for Line {
    /// Draws the Unicode symbol corresponding to the line variant into the provided writer.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::graphics::lines::Line;
    /// use your_crate::graphics::Drawable;
    ///
    /// let mut buffer = Vec::new();
    /// Line::Horizontal.draw(&mut buffer).unwrap();
    /// assert_eq!(buffer, "─".as_bytes());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`std::io::Error`] if writing to the `writer` fails.
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
