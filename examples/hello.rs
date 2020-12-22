use semtext::input::Action;
use semtext::widget::Button;
use semtext::{grid_area, Screen};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    let a = Button::new("Hello!");
    let grid = grid_area!(
        [. . .]
        [. a .]
        [. . .]
    )?;
    while screen.step(&grid)? != Action::Quit() {}
    Ok(())
}
