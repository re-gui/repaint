use std::rc::Rc;

use repaint::{BasicPainter, Canvas, painter::{Context, WithPathResource, methods::PaintStyle}, base::{shapes::{path::PathCommand, Shape}, defs::{colors::default_color_types::RgbaFColor, linalg::Vec2f64}, paint::{Paint, Ink}, pen::Pen}};

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
    //fn make_path(&mut self, path_iter: &mut dyn Iterator<Item = PathCommand>) -> Result<PathResource<'context>, ()> { // TODO proper error
    //    Ok(PathResource(PainterResource::new(
    //        Rc::new(create_skia_path(path_iter)),
    //        self.lifecycle()
    //    )))
    //}
}

impl<'canvas, 'surface, 'context> BasicPainter<'context> for SkiaPainter<'canvas, 'surface, 'context> {
    type NativeColor = RgbaFColor;

    //type Resources = SkiaResources;
    type Canvas = SkiaCanvas<'surface, 'context>;
    type Context = SkiaContext;

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

    fn clip(&self, shape: &impl Shape) -> Result<(), repaint::painter::methods::ClipError> {
        todo!()
    }

    fn point(
        &mut self,
        pos: Vec2f64,
        style: PaintStyle<Self::NativeColor>,
    ) {
        todo!()
    }

    fn pixel(
        &mut self,
        pos: Vec2f64,
        ink: Ink<Self::NativeColor>,
    ) {
        todo!()
    }

    fn line(
        &mut self,
        start: Vec2f64,
        end: Vec2f64,
        pen: Pen<Self::NativeColor>,
    ) {
        self.canvas.surface.canvas().draw_line(
            (start.x as f32, start.y as f32),
            (end.x as f32, end.y as f32),
            &paint_style_to_skia_paint(&PaintStyle::Stroke(pen))
        );
    }

    fn draw_path_iter<'s, 'a>(
        &'s mut self,
        path_iter: &mut dyn Iterator<Item = PathCommand>,
        style: PaintStyle<Self::NativeColor>,
    ) {
        let path = create_skia_path(path_iter);
        let paint = paint_style_to_skia_paint(&style);
        self.canvas.surface.canvas().draw_path(path.as_ref(), &paint);
    }

    fn clear(
        &mut self,
        color: Self::NativeColor,
    ) {
        self.canvas.surface.canvas().clear(color_to_skia_color(color));
    }

    fn clear_with(
        &mut self,
        paint: &Paint<Self::NativeColor>,
    ) {
        todo!()
    }
}


impl<'canvas, 'surface, 'context> WithPathResource<'context> for SkiaPainter<'canvas, 'surface, 'context> {
    type Path = skia_safe::Path; // TODO ??

    fn make_path(
        &mut self,
        path_iter: impl Iterator<Item = PathCommand>
    ) -> Result<Self::Path, ()> {
        let path = create_skia_path(path_iter);
        Ok(path)
    }

    fn path(&mut self, path: &Self::Path, style: PaintStyle<Self::NativeColor>) {
        let paint = paint_style_to_skia_paint(&style);
        self.canvas.surface.canvas().draw_path(path.as_ref(), &paint);
    }
}




