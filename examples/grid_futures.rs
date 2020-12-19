use crossterm::event::{Event, KeyCode};
use semtext::style::Outline;
use semtext::widget::{Border, Label, Spacer};
use semtext::{grid_area, Screen};

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Layout Test")?;
    let s = Spacer::default().with_fill('.')?;
    let b = Border::default()
        .with_outline(Outline::Solid)
        .with_bottom(Some(Outline::Double))
        .with_right(Some(Outline::Dashed));
    let a = Label::new("This is a bit of text in a label");
    let c = Label::new("This label has more text on the right side");
    let grid = grid_area!(
        [. . . .]
        [a a . b]
        [. . . b]
        [. c c b]
        [s . . b]
    )?;
    loop {
        screen.render(&grid)?;
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
