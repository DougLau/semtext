use semtext::{BBox, Border, Edge, LineStyle, Screen, Widget};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Border Test")?;
    let mut cells = screen.cells(BBox::new(20, 5, 15, 6));
    let bdr = Border::default()
        .with_edges(Edge::ALL)
        .with_accents(Edge::TOP_LEFT);
    bdr.render(&mut cells)?;
    let mut cells = screen.cells(BBox::new(40, 4, 12, 8));
    let bdr = Border::default()
        .with_edges(Edge::ALL)
        .with_accents(Edge::BOTTOM_RIGHT)
        .with_line_style(LineStyle::Double);
    bdr.render(&mut cells)?;
    screen.event()?;
    Ok(())
}
