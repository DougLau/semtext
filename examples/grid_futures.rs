use semtext::input::Action;
use semtext::text::{Outline, Stroke};
use semtext::widget::{BorderHeight, BorderStyle, Label, Spacer};
use semtext::{grid_area, Screen};

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Layout Test")?;
    let s = Spacer::default().with_fill('.')?;
    let mut b = Spacer::default().with_border();
    b.set_border_style(Some(BorderStyle::Bevel(
        Outline::Heavy(Stroke::Dashed),
        BorderHeight::Raised,
    )));
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    futures::executor::block_on(async_main())
}
