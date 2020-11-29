use semtext::{BBox, Glyph, Screen};
use std::convert::TryFrom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Fill Test")?;
    let mut cells = screen.cells(BBox::new(10, 5, 10, 5));
    cells.fill(Glyph::try_from('🏼')?)?;
    screen.event()?;
    Ok(())
}
