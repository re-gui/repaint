use std::cell::RefCell;

use painter::SkiaContext;
use repaint::{Canvas, Painter, painter::methods, base::{defs::linalg::Vec2f64, paint::Paint, pen::Pen, blending::BlendMode, shapes::{path::PathCommand, Shape}, transform::Transform2d}};
use skia_safe::Surface;

mod painter;

// TODO move
pub fn make_skia_context() -> SkiaContext {
    SkiaContext::new()
}

pub struct SkiaCanvas<'surface, 'context: 'surface>
{
    surface: &'surface mut Surface,
    context: &'context RefCell<SkiaContext>,
}

impl<'surface, 'context> SkiaCanvas<'surface, 'context> {
    pub fn new(surface: &'surface mut Surface, context: &'context RefCell<SkiaContext>) -> Self {
        Self {
            surface,
            context,
        }
    }
}

impl<'surface, 'context: 'surface> Canvas<'context> for SkiaCanvas<'surface, 'context> {
    type Painter<'canvas> = SkiaPainter<'canvas, 'surface, 'context> where Self: 'canvas;

    fn painter<'s>(&'s mut self) -> Result<Self::Painter<'s>, repaint::canvas::GetPainterError> {
        let painter = SkiaPainter {
            canvas: self,
        };

        Ok(painter)
    }

    fn shape(&self) -> Box<dyn Shape> {
        todo!()
    }

    //fn painter<'s>(&'s mut self) -> Result<Box<dyn repaint::Painter<'context> + 's>, repaint::canvas::GetPainterError> {
    //    let painter = SkiaPainter {
    //        canvas: self,
    //    };
//
    //    Ok(Box::new(painter))
    //}
    //fn shape(&self) -> Box<dyn Shape> {
    //    todo!()
    //}
}

pub struct SkiaPainter<'canvas, 'surface, 'context> {
    canvas: &'canvas mut SkiaCanvas<'surface, 'context>,
}

impl<'canvas, 'surface, 'context> SkiaPainter<'canvas, 'surface, 'context> {
    //fn canvas<'s>(&'s self) -> &'canvas SkiaCanvas<'surface, 'context> {
    fn canvas<'s>(&'s self) -> &'canvas SkiaCanvas {
        self.canvas
    }
}

mod conversions {
    use repaint::{base::{defs::colors::default_color_types::RgbaFColor, paint::{Paint, Ink, Color}, blending::BlendMode, pen::{StrokeWidth, PenCap}, shapes::path::PathCommand}, painter::methods::PaintStyle};

    pub fn color_to_skia_color(color: RgbaFColor) -> skia_safe::Color4f {
        skia_safe::Color4f {
            r: color.r(),
            g: color.g(),
            b: color.b(),
            a: color.a(),
        }
    }

    pub fn blend_mode_to_skia(blend_mode: BlendMode) -> skia_safe::BlendMode {
        use skia_safe::BlendMode as SkiaBlendMode;
        match blend_mode {
            BlendMode::Clear => SkiaBlendMode::Clear,
            BlendMode::Src => SkiaBlendMode::Src,
            BlendMode::Dst => SkiaBlendMode::Dst,
            BlendMode::SrcOver => SkiaBlendMode::SrcOver,
            BlendMode::DstOver => SkiaBlendMode::DstOver,
            BlendMode::SrcIn => SkiaBlendMode::SrcIn,
            BlendMode::DstIn => SkiaBlendMode::DstIn,
            BlendMode::SrcOut => SkiaBlendMode::SrcOut,
            BlendMode::DstOut => SkiaBlendMode::DstOut,
            BlendMode::SrcATop => SkiaBlendMode::SrcATop,
            BlendMode::DstATop => SkiaBlendMode::DstATop,
            BlendMode::Xor => SkiaBlendMode::Xor,
            BlendMode::Plus => SkiaBlendMode::Plus,
            BlendMode::PlusClamped => SkiaBlendMode::Plus, // ??
            BlendMode::Modulate => SkiaBlendMode::Modulate,
            BlendMode::Screen => SkiaBlendMode::Screen,
            BlendMode::Overlay => SkiaBlendMode::Overlay,
            BlendMode::Darken => SkiaBlendMode::Darken,
            BlendMode::Lighten => SkiaBlendMode::Lighten,
            BlendMode::ColorDodge => SkiaBlendMode::ColorDodge,
            BlendMode::ColorBurn => SkiaBlendMode::ColorBurn,
            BlendMode::HardLight => SkiaBlendMode::HardLight,
            BlendMode::SoftLight => SkiaBlendMode::SoftLight,
            BlendMode::Difference => SkiaBlendMode::Difference,
            BlendMode::Exclusion => SkiaBlendMode::Exclusion,
            BlendMode::Multiply => SkiaBlendMode::Multiply,
            BlendMode::Hue => SkiaBlendMode::Hue,
            BlendMode::Saturation => SkiaBlendMode::Saturation,
            BlendMode::Color => SkiaBlendMode::Color,
            BlendMode::Luminosity => SkiaBlendMode::Luminosity,
        }
    }

    pub fn paint_style_to_skia_paint(style: &PaintStyle) -> skia_safe::Paint {
        use skia_safe::{
            Paint as SkiaPaint,
            paint::Style as SkiaPaintStyle,
        };

        let mut sk_paint = SkiaPaint::default();

        add_paint_to_skia_paint(&mut sk_paint, style.paint());

        sk_paint.set_style(match style {
            PaintStyle::Stroke(_) => SkiaPaintStyle::Stroke,
            PaintStyle::Fill(_) => SkiaPaintStyle::Fill,
            PaintStyle::StrokeAndFill(_) => SkiaPaintStyle::StrokeAndFill,
        });

        if let PaintStyle::Stroke(pen) = style {
            match pen.stroke_width {
                StrokeWidth::Normal(width) => sk_paint.set_stroke_width(width),
                StrokeWidth::Hairline => sk_paint.set_stroke_width(0.0),
                StrokeWidth::Cosmetic(_width) => sk_paint.set_stroke_width(0.0), // TODO Skia doesn't support cosmetic stroke width
            };
            match pen.cap {
                PenCap::Butt => sk_paint.set_stroke_cap(skia_safe::PaintCap::Butt),
                PenCap::Round => sk_paint.set_stroke_cap(skia_safe::PaintCap::Round),
                PenCap::Square => sk_paint.set_stroke_cap(skia_safe::PaintCap::Square),
            };
        }

        sk_paint
    }

    pub fn add_paint_to_skia_paint(sk_paint: &mut skia_safe::Paint, paint: &Paint) {
        match paint.ink {
            Ink::None => {},
            Ink::Color(color) => {
                sk_paint.set_color4f(color_to_skia_color(color), None); // TODO color_space?
            },
            Ink::Shader => {
                todo!()
            },
        };

        // TODO bleand mode

        sk_paint.set_anti_alias(paint.anti_alias);
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
}


// TODO remove Vec2f64 alias from repaint lib to be more explicit