use crate::base::{defs::linalg::Vec2f64, paint::{Paint, Color, Ink}, pen::Pen, shapes::{path::{PathCommand, Path, PathResource}, Shape}};

use super::TransformMethods;

#[derive(Clone, Debug, PartialEq)]
pub enum PaintStyle {
    Stroke(Pen),
    Fill(Paint),
    StrokeAndFill(Pen),
}

impl PaintStyle {
    pub fn paint(&self) -> &Paint {
        match self {
            PaintStyle::Stroke(pen) | Self::StrokeAndFill(pen) => &pen.paint,
            PaintStyle::Fill(paint) => paint,
        }
    }

    pub fn paint_mut(&mut self) -> &mut Paint {
        match self {
            PaintStyle::Stroke(pen) | Self::StrokeAndFill(pen) => &mut pen.paint,
            PaintStyle::Fill(paint) => paint,
        }
    }
}

// TODO instead of f32 or f64 for points, we could make an enum to discriminate between integer and floating point coordinates...

pub trait DrawingMethods: TransformMethods {

    fn point(
        &mut self,
        pos: Vec2f64,
        style: PaintStyle,
    );

    fn pixel(
        &mut self,
        pos: Vec2f64,
        ink: Ink,
    );

    fn line(
        &mut self,
        start: Vec2f64,
        end: Vec2f64,
        pen: &Pen,
    );

    fn path(&mut self, path: &PathResource, style: PaintStyle);

    fn draw_path_iter<'s, 'a>(
        &'s mut self,
        path_iter: &mut dyn Iterator<Item = PathCommand>,
        style: PaintStyle,
    );

    //fn stroke_shape(&mut self, shape: &dyn Shape, pen: &Pen) {
    //    todo!("default implementation")
    //} // TODO maybe provide default implementation based on stroke_path

    // TODO this feature might not be supported because it is equivalent to the blend mode `Src`
    fn clear(
        &mut self,
        color: Color,
    );

    fn clear_with(
        &mut self,
        paint: &Paint,
    ) {
        todo!("default implementation")
    }

    // TODO clear, example for skia: surface.canvas().clear(Color::WHITE);
}