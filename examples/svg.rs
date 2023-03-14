use std::{error::Error, path::Path};

use repaint::{base::defs::{rect::FRect, linalg::Vec2f}, Canvas};


pub fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello There!");

    let mut svg = repaint::default_painters::svg::SvgCanvas::new(FRect::new(
        Vec2f::new(0.0, 0.0),
        Vec2f::new(100.0, 100.0))
    );

    {
        let mut painter = svg.painter()
            .map_err(|e| Box::new(e));

        
    }

    svg.save(Path::new("test.svg"))?;

    println!("Goodbye!");


    Ok(())
}