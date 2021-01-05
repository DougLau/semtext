use semtext::input::Action;
use semtext::widget::Button;
use semtext::{grid_area, Screen};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    let a = Button::new("A").with_border();
    let b = Button::new("B").with_border();
    let c = Button::new("C").with_border();
    let d = Button::new("D").with_border();
    let e = Button::new("E Wider").with_border();
    let f = Button::new("F").with_border();
    let g = Button::new("G").with_border();
    let h = Button::new("H").with_border();
    let i = Button::new("I").with_border();
    let j = Button::new("J").with_border();
    let k = Button::new("K").with_border();
    let l = Button::new("L").with_border();
    let grid = grid_area!(
        [a e i]
        [b f j]
        [c g k]
        [d h l]
    )?;
    while screen.step(&grid)? != Action::Quit() {}
    Ok(())
}
