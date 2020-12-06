use crossterm::event::{Event, KeyCode};
use semtext::widget::{Border, Label, LineStyle, Spacer};
use semtext::{layout, Edge, GridItem, Layout, Screen};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Layout Test")?;
    let s = Spacer::default().with_fill('.')?;
    let b = Border::default()
        .with_edges(Edge::ALL)
        .with_accents(Edge::BOTTOM_RIGHT)
        .with_line_style(LineStyle::Double);
    let a = Label::new("This is a bit of test text inside of a label");
    let c = Label::new("This label is on the right side");
    loop {
        let layout = layout!(screen.bbox(),
            [_ _ _ b],
            [a s c b],
        )?;
        screen.render(&layout)?;
        match screen.event()? {
            Event::Key(key) => {
                if key.code == KeyCode::Esc {
                    break;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
