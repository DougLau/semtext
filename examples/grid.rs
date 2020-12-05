use semtext::{layout, Label, Layout, Screen, Spacer, Widget};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Layout Test")?;
    let s0 = Spacer::default();
    let s1 = Spacer::default();
    let a = Label::new("This is a bit of test text inside of a label");
    let b = Label::new("This label is on the right side");
    let layout = layout!(screen.bbox(),
        [s0 s0 b],
        [ a s1 b],
    )?;
    screen.render(&layout)?;
    screen.event()?;
    Ok(())
}
