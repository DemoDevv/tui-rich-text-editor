pub mod lines;

pub trait Drawable {
    fn draw(&self, writer: &mut impl std::io::Write) -> Result<(), std::io::Error>;
}
