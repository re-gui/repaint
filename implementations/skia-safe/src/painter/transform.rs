use repaint::{painter::methods::TransformMethods, base::{transform::Transform2d, shapes::Shape}, Painter};

use crate::SkiaPainter;


impl<'canvas, 'surface> TransformMethods for SkiaPainter<'canvas, 'surface> {
    fn with_save<'a>(&'a mut self) -> Box<dyn Painter> {
        todo!()
    }
    fn transform(&mut self, transform: &Transform2d) -> Result<(), repaint::painter::methods::TransformError> {
        todo!()
    }
    fn clip(&self, shape: &dyn Shape) -> Result<(), repaint::painter::methods::ClipError> {
        todo!()
    }
}