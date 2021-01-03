use semtext::input::Action;
use semtext::widget::Button;
use semtext::{grid_area, Screen};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    let a = Button::new("A");
    let b = Button::new("B");
    let c = Button::new("C");
    let d = Button::new("D");
    let e = Button::new("E Wider");
    let f = Button::new("F");
    let g = Button::new("G");
    let h = Button::new("H");
    let i = Button::new("I");
    let j = Button::new("J");
    let k = Button::new("K");
    let l = Button::new("L");
    let grid = grid_area!(
        [a e i]
        [b f j]
        [c g k]
        [d h l]
    )?;
    while screen.step(&grid)? != Action::Quit() {}
    Ok(())
}
