use semtext::input::Action;
use semtext::widget::{Border, BorderHeight, BorderStyle, Label, Spacer};
use semtext::{grid_area, Screen};

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Layout Test")?;
    let s = Spacer::default().with_fill('.')?;
    let b = Border::new(BorderStyle::Bevel(BorderHeight::Raised));
    let a = Label::new("This is a bit of text in a label");
    let c = Label::new("This label has more text on the right side");
    let grid = grid_area!(
        [. . . .]
        [a a . b]
        [. s . b]
        [. c c b]
        [. . . b]
    )?;
    while screen.step_future(&grid).await? != Action::Quit() {}
    Ok(())
}

fn main() {
    pasts::exec!(async { async_main().await.expect("async_main errored") });
}
