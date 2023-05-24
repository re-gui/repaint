use repaint::painter::methods::AntialiasMethods;

use crate::SkiaPainter;

impl<'canvas, 'surface, 'context, 'context_lifecycle> AntialiasMethods for SkiaPainter<'canvas, 'surface, 'context, 'context_lifecycle> {
    fn has_antialias(&self) -> bool {
        true
    }
}