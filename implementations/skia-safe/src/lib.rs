use repaint::{Canvas, Painter, painter::methods::{BasicShapesMethods, TransformMethods, StrokingMethods, AntialiasMethods, BlendModeMethods}, base::{defs::linalg::Vec2f64, paint::Paint, pen::Pen, blending::BlendMode, shapes::{path::PathCommand, Shape}, transform::Transform2d}};
use skia_safe::Surface;

mod painter;

pub struct SkiaCanvas<'surface> {
    surface: &'surface mut Surface,
}

impl<'surface> SkiaCanvas<'surface> {
    pub fn new(surface: &'surface mut Surface) -> Self {
        Self {
            surface,
        }
    }
}

impl<'surface> Canvas for SkiaCanvas<'surface> {
    fn painter<'s>(&'s mut self) -> Result<Box<dyn repaint::Painter + 's>, repaint::canvas::GetPainterError> {
        let painter = SkiaPainter {
            canvas: self,
        };

        Ok(Box::new(painter))
    }
    fn shape(&self) -> Box<dyn Shape> {
        todo!()
    }
}

pub struct SkiaPainter<'canvas, 'surface> {
    canvas: &'canvas mut SkiaCanvas<'surface>,
}


mod conversions {
    use repaint::{base::{defs::colors::default_color_types::RgbaFColor, paint::{Paint, Ink, Color}, blending::BlendMode}, painter::methods::PaintStyle};

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

        // TODO width and path effect
        sk_paint.set_anti_alias(true);

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
    }
}


// TODO remove Vec2f64 alias from repaint lib to be more explicit