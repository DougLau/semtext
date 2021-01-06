use semtext::{grid_area, input::Action, widget::Label, Screen, Widget};

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    let a = Label::new("Hello!").into_button();
    let grid = grid_area!(
        [. . .]
        [. a .]
        [. . .]
    )?;
    while screen.step(&grid).await? != Action::Quit() {}
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    futures::executor::block_on(async_main())
}
