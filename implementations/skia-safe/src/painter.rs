use repaint::{Painter, Canvas};

use crate::SkiaPainter;



mod blend;
mod antialias;
mod stroking;
mod transform;
mod shapes;

impl<'canvas, 'surface> Painter for SkiaPainter<'canvas, 'surface> {
    fn canvas(&self) -> &dyn Canvas {
        self.canvas
    }
}








