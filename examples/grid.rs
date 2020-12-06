use crossterm::event::{Event, KeyCode};
use semtext::widget::{Border, Label, LineStyle, Spacer};
use semtext::{layout, Edge, Screen};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Layout Test")?;
    let s = Spacer::default().with_fill('.')?;
    let b = Border::default()
        .with_edges(Edge::ALL)
        .with_accents(Edge::BOTTOM_RIGHT)
        .with_line_style(LineStyle::Double);
    let a = Label::new("This is a bit of text in a label");
    let c = Label::new("This label has more text on the right side");
    loop {
        let layout = layout!(screen.bbox(),
            [_ _ _ _],
            [a a _ b],
            [_ _ _ b],
            [_ c c b],
            [s _ _ b],
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
