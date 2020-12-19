use crossterm::event::{Event, KeyCode};
use semtext::widget::Label;
use semtext::{grid_area, Screen};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    let a = Label::new("Hello!");
    let grid = grid_area!(
        [. . .]
        [. a .]
        [. . .]
    )?;
    loop {
        screen.render(&grid)?;
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
