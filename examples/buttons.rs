use semtext::input::Action;
use semtext::widget::Label;
use semtext::{grid_area, Screen, Widget};
use std::error::Error;

async fn async_main() -> Result<(), Box<dyn Error>> {
    let mut screen = Screen::new()?;
    let a = Label::new("A").into_button();
    let b = Label::new("B").into_button();
    let c = Label::new("C").into_button();
    let d = Label::new("D").into_button();
    let e = Label::new("E Wider").into_button();
    let f = Label::new("F").into_button();
    let g = Label::new("G").into_button();
    let h = Label::new("H").into_button();
    let i = Label::new("I").into_button();
    let j = Label::new("J").into_button();
    let k = Label::new("K").into_button();
    let l = Label::new("L").into_button();
    let grid = grid_area!(
        [a e i]
        [b f j]
        [c g k]
        [d h l]
    )?;
    while screen.step(&grid).await? != Action::Quit() {}
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    futures::executor::block_on(async_main())
}
