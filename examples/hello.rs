use semtext::{grid_area, input::Action, widget::Button, Screen};
use std::error::Error;

async fn async_main() -> Result<(), Box<dyn Error>> {
    let mut screen = Screen::new()?;
    let a = Button::new("Hello!").with_border();
    let grid = grid_area!(
        [. . .]
        [. a .]
        [. . .]
    )?;
    while screen.step(&grid).await? != Action::Quit() {}
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    futures::executor::block_on(async_main())
}
