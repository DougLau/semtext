use semtext::{BBox, Label, Screen, Widget};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Label Test")?;
    let mut cells = screen.cells(BBox::new(20, 5, 10, 5));
    let label = Label::new("This is a bit of test text inside of a label");
    label.render(&mut cells)?;
    screen.event()?;
    Ok(())
}
