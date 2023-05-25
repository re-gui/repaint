use std::rc::Rc;

use repaint::{Painter, Canvas, painter::{Context}, base::shapes::path::PathCommand};

use crate::{SkiaPainter, SkiaCanvas, conversions::{create_skia_path, paint_style_to_skia_paint, color_to_skia_color}};




pub struct SkiaContext {
}

impl SkiaContext{
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Context for SkiaContext {

    fn has_path_resources(&self) -> bool {
        true
    }

    //fn make_path(&mut self, path_iter: &mut dyn Iterator<Item = PathCommand>) -> Result<PathResource<'context>, ()> { // TODO proper error
    //    Ok(PathResource(PainterResource::new(
    //        Rc::new(create_skia_path(path_iter)),
    //        self.lifecycle()
    //    )))
    //}
}

impl<'canvas, 'surface: 'canvas, 'context: 'canvas + 'surface> Painter<'context> for SkiaPainter<'canvas, 'surface, 'context> {
    //type Resources = SkiaResources;
    type Canvas = SkiaCanvas<'surface, 'context>;
    type Context = SkiaContext;

    type Path = skia_safe::Path; // TODO ??

    fn canvas(&self) -> &Self::Canvas {
        self.canvas
    }

    fn has_antialias(&self) -> bool {
        todo!()
    }

    fn is_blend_mode_valid(&self, _mode: repaint::base::blending::BlendMode) -> bool {
        todo!()
    }

    fn enumerate_valid_blend_modes<'s>(&'s self) {
        todo!()
    }

    fn with_save<'a>(&'a mut self, paint: impl FnOnce(&mut Self) -> Result<(), ()>) -> Result<(), ()> {
        todo!()
    }

    fn transform(&mut self, transform: &repaint::base::transform::Transform2d) -> Result<(), repaint::painter::methods::TransformError> {
        todo!()
    }

    fn clip(&self, shape: &dyn repaint::base::shapes::Shape) -> Result<(), repaint::painter::methods::ClipError> {
        todo!()
    }

    fn point(
        &mut self,
        pos: repaint::base::defs::linalg::Vec2f64,
        style: repaint::painter::methods::PaintStyle,
    ) {
        todo!()
    }

    fn pixel(
        &mut self,
        pos: repaint::base::defs::linalg::Vec2f64,
        ink: repaint::base::paint::Ink,
    ) {
        todo!()
    }

    fn line(
        &mut self,
        start: repaint::base::defs::linalg::Vec2f64,
        end: repaint::base::defs::linalg::Vec2f64,
        pen: &repaint::base::pen::Pen,
    ) {
        todo!()
    }

    fn make_path(
        &mut self,
        path_iter: &mut dyn Iterator<Item = PathCommand>
    ) -> Result<Self::Path, ()> {
        let path = create_skia_path(path_iter);
        Ok(path)
    }

    fn path(&mut self, path: &Self::Path, style: repaint::painter::methods::PaintStyle) {
        let paint = paint_style_to_skia_paint(&style);
        self.canvas.surface.canvas().draw_path(path.as_ref(), &paint);
    }

    fn draw_path_iter<'s, 'a>(
        &'s mut self,
        path_iter: &mut dyn Iterator<Item = PathCommand>,
        style: repaint::painter::methods::PaintStyle,
    ) {
        todo!()
    }

    fn clear(
        &mut self,
        color: repaint::base::paint::Color,
    ) {
        self.canvas.surface.canvas().clear(color_to_skia_color(color));
    }

    fn clear_with(
        &mut self,
        paint: &repaint::base::paint::Paint,
    ) {
        todo!()
    }
}








