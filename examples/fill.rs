use semtext::Screen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Fill Test")?;
    screen.fill(screen.area(), '🏼')?;
    screen.event()?;
    Ok(())
}
