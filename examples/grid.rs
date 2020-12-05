use semtext::{layout, Border, Edge, Label, Layout, Screen, Spacer, Widget};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Layout Test")?;
    let s0 = Spacer::default().with_fill('.')?;
    let b0 = Border::default().with_edges(Edge::ALL);
    let a = Label::new("This is a bit of test text inside of a label");
    let b = Label::new("This label is on the right side");
    let layout = layout!(screen.bbox(),
        [b0 b0 b],
        [ a s0 b],
    )?;
    screen.render(&layout)?;
    screen.event()?;
    Ok(())
}
