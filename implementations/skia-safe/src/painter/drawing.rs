use std::rc::Rc;

use repaint::{painter::methods::{DrawingMethods, PaintStyle}, base::{paint::{Paint, Color}, defs::linalg::Vec2f64, pen::Pen, shapes::{path::{PathCommand, PathResource}, Shape}}, resource::PainterResource, Painter};

use crate::{SkiaPainter, conversions::{color_to_skia_color, paint_style_to_skia_paint}};

impl<'canvas, 'surface, 'context, 'context_lifecycle> DrawingMethods for SkiaPainter<'canvas, 'surface, 'context, 'context_lifecycle> {

    fn point(
        &mut self,
        pos: Vec2f64,
        style: repaint::painter::methods::PaintStyle,
    ) {
        todo!()
    }

    fn pixel(
        &mut self,
        pos: Vec2f64,
        ink: repaint::base::paint::Ink,
    ) {
        todo!()
    }

    fn line(
        &mut self,
        start: Vec2f64,
        end: Vec2f64,
        pen: &Pen,
    ) {
        todo!()
    }

    fn draw_path_iter(
        &mut self,
        path_iter: &mut dyn Iterator<Item = PathCommand>,
        style: PaintStyle,
    ) {
        let paint = paint_style_to_skia_paint(&style);
        self.canvas.surface.canvas().draw_path(&create_skia_path(path_iter), &paint);
    }

    fn clear(
        &mut self,
        color: Color,
    ) {
        
    }

    fn clear_with(
        &mut self,
        paint: &Paint,
    ) {
    
    }
}
