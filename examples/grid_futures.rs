use crossterm::event::{Event, KeyCode};
use semtext::widget::{Border, Label, Spacer};
use semtext::{layout, style::Line, Screen};

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Layout Test")?;
    let s = Spacer::default().with_fill('.')?;
    let b = Border::default()
        .with_all(Line::Solid)
        .with_bottom(Some(Line::Double))
        .with_right(Some(Line::Dashed));
    let a = Label::new("This is a bit of text in a label");
    let c = Label::new("This label has more text on the right side");
    loop {
        let layout = layout!(screen.bbox(),
            [_ _ _ _]
            [a a _ b]
            [_ _ _ b]
            [_ c c b]
            [s _ _ b]
        )?;
        screen.render(&layout)?;
        match screen.input().await? {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    futures::executor::block_on(async_main())
}
