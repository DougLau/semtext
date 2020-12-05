use semtext::{BBox, IntoGlyph, Screen};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Fill Test")?;
    let mut cells = screen.cells(BBox::new(10, 5, 10, 5));
    cells.fill(&'🏼'.into_glyph()?)?;
    screen.event()?;
    Ok(())
}
