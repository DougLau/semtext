use semtext::{Area, Glyph, Screen};
use std::convert::TryFrom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Fill Test")?;
    let mut grid = screen.grid(Area::new(10, 5, 10, 5));
    grid.fill(Glyph::try_from('🏼')?)?;
    screen.event()?;
    Ok(())
}
