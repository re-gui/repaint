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

    fn path(&mut self, path: &PathResource, style: PaintStyle) {
        let paint = paint_style_to_skia_paint(&style);
        let path = path.0.handle
            .clone()
            .downcast::<skia_safe::Path>()
            .unwrap(); // TODO error handling
        self.canvas.surface.canvas().draw_path(path.as_ref(), &paint);
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
        self.canvas.surface.canvas().clear(color_to_skia_color(color));
    }

    fn clear_with(
        &mut self,
        paint: &Paint,
    ) {
    
    }
}


// TODO move and remove pub
pub fn create_skia_path(path_iter: &mut dyn Iterator<Item = PathCommand>) -> skia_safe::Path {
    let mut sk_path = skia_safe::Path::new();

    for element in path_iter {
        match element {
            PathCommand::MoveTo(pos) => sk_path.move_to((pos.x as f32, pos.y as f32)),
            PathCommand::MoveToOffset(pos) => sk_path.r_move_to((pos.x as f32, pos.y as f32)),
            PathCommand::LineTo(pos) => sk_path.line_to((pos.x as f32, pos.y as f32)),
            PathCommand::LineToOffset(pos) => sk_path.r_line_to((pos.x as f32, pos.y as f32)),
            PathCommand::HorizontalLineTo(x) => sk_path.line_to((x as f32, 0.0)),
            PathCommand::HorizontalLineToOffset(x) => sk_path.r_line_to((x as f32, 0.0)),
            PathCommand::VerticalLineTo(y) => sk_path.line_to((0.0, y as f32)),
            PathCommand::VerticalLineToOffset(y) => sk_path.r_line_to((0.0, y as f32)),
            PathCommand::ClosePath => sk_path.close(),
            PathCommand::CubicBezierTo {
                control_pt_1,
                control_pt_2,
                end_pt
            } => sk_path.cubic_to(
                (control_pt_1.x as f32, control_pt_1.y as f32),
                (control_pt_2.x as f32, control_pt_2.y as f32),
                (end_pt.x as f32, end_pt.y as f32),
            ),
            PathCommand::CubicBezierToOffset {
                control_pt_1_offset,
                control_pt_2_offset,
                end_pt_offset
            } => sk_path.r_cubic_to(
                (control_pt_1_offset.x as f32, control_pt_1_offset.y as f32),
                (control_pt_2_offset.x as f32, control_pt_2_offset.y as f32),
                (end_pt_offset.x as f32, end_pt_offset.y as f32),
            ),
            PathCommand::SmoothCubicBezierCurveTo {
                control_pt_2,
                end_pt
            } => todo!("SmoothCubicBezierCurveTo"),
            PathCommand::SmoothCubicBezierCurveToOffset {
                control_pt_2_offset,
                end_pt_offset
            } => todo!("SmoothCubicBezierCurveToOffset"),
            PathCommand::QuadraticBezierCurveTo {
                control_pt,
                end_pt
            } => sk_path.quad_to(
                (control_pt.x as f32, control_pt.y as f32),
                (end_pt.x as f32, end_pt.y as f32),
            ),
            PathCommand::QuadraticBezierCurveToOffset {
                control_pt_offset,
                end_pt_offset
            } => sk_path.r_quad_to(
                (control_pt_offset.x as f32, control_pt_offset.y as f32),
                (end_pt_offset.x as f32, end_pt_offset.y as f32),
            ),
            PathCommand::SmoothQuadraticBezierCurveTo(end_pt) => todo!("SmoothQuadraticBezierCurveTo"),
            PathCommand::SmoothQuadraticBezierCurveToOffset(end_pt_offset) => todo!("SmoothQuadraticBezierCurveToOffset"),
            PathCommand::EllipticalArcTo {
                radii,
                x_axis_rotation,
                large_arc_flag,
                sweep_flag,
                end_pt
            } => todo!("EllipticalArcTo"),
            PathCommand::EllipticalArcToOffset {
                radii,
                x_axis_rotation,
                large_arc_flag,
                sweep_flag,
                end_pt_offset
            } => todo!("EllipticalArcToOffset"),
        }; // TODO following skia, *Offset -> Relative* or something like that
        // TODO add functions to convert from relative to absolute given the current position,
        // also handle the smooth case
        // TODO add a function to convert EllipticalArcTo center parameterization and add center parameterization to PathCommand
    }

    sk_path
}