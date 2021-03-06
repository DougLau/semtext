use semtext::input::Action;
use semtext::widget::{Label, Spacer};
use semtext::{grid_area, Screen, Widget};
use std::error::Error;

async fn async_main() -> Result<(), Box<dyn Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Layout Test")?;
    let s = Spacer::default().with_fill('.')?;
    let a = Label::new("This is a bit of text in a label");
    let c = Label::new("This label has more text on the right side blah blah, more text to display in the widget so that we can show off the scroll view features, plus some more mindless text with no particular meaning or purpose.")
        .into_scroll_view();
    let grid = grid_area!(
        [. a a . .]
        [. . s . .]
        [. . c c .]
        [. . . . .]
    )?;
    while screen.step(&grid).await? != Action::Quit() {}
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    futures::executor::block_on(async_main())
}
