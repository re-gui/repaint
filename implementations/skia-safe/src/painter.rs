use std::rc::Rc;

use repaint::{Painter, Canvas, painter::Context, base::shapes::path::{PathCommand, PathResource}, resource::PainterResource};

use crate::SkiaPainter;

use self::drawing::create_skia_path;



mod blend;
mod antialias;
mod drawing;
mod transform;
mod shapes;

pub struct SkiaContext {
}

impl SkiaContext{
    pub fn new() -> Self {
        Self {
        }
    }
}

impl<'context_lifecycle> Context<'context_lifecycle> for SkiaContext<'context_lifecycle> {
    fn lifecycle(&self) -> &'context_lifecycle () {
        self.lifecycle
    }

    fn has_path_resources(&self) -> bool {
        true
    }

    fn make_path(&mut self, path_iter: &mut dyn Iterator<Item = PathCommand>) -> Result<PathResource<'context_lifecycle>, ()> { // TODO proper error
        Ok(PathResource(PainterResource::new(
            Rc::new(create_skia_path(path_iter)),
            self.lifecycle()
        )))
    }
}

impl<'canvas, 'surface, 'context, 'context_lifecycle> Painter<'context_lifecycle> for SkiaPainter<'canvas, 'surface, 'context, 'context_lifecycle> {
    fn canvas(&self) -> &dyn Canvas<'context_lifecycle> {
        self.canvas
    }

    fn context(&self) -> &dyn Context<'context_lifecycle> {
        self.canvas.context
    }

    fn context_mut(&mut self) -> &mut dyn Context<'context_lifecycle> {
        self.canvas.context
    }
}








