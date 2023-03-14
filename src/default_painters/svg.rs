use std::error::Error;

use crate::{Canvas, base::{defs::{linalg::Vec2f, rect::FRect}, shapes::Shape, blending::BlendMode}, Painter};



pub struct SvgCanvas {
    doc: svg::Document,
    viewbox: FRect,
}

impl SvgCanvas {
    pub fn new(viewbox: FRect) -> Self {
        let mut doc = svg::Document::new();
        doc.get_attributes_mut().insert(
            "viewbox".into(),
            format!("{} {} {} {}", viewbox.min.x, viewbox.min.y, viewbox.max.x, viewbox.max.y).into()
        );

        Self {
            doc,
            viewbox
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let doc = self.doc.to_string();
        std::fs::write("test.svg", doc)?;
        Ok(())
    }
}

impl Canvas for SvgCanvas {
    fn painter<'s>(&'s mut self) -> Result<Box<dyn crate::Painter + 's>, crate::canvas::GetPainterError> {
        Ok(Box::new(SvgPainter {
            canvas: self,
        }))
    }

    fn shape(&self) -> Shape {
        Shape::Rect(self.viewbox)
    }
}

pub struct SvgPainter<'a> {
    canvas: &'a mut SvgCanvas,
}

impl<'a> crate::painter::BlendModeMethods for SvgPainter<'a> {}
impl<'a> crate::painter::AntialiasMethods for SvgPainter<'a> {}
impl<'a> crate::painter::TransformMethods for SvgPainter<'a> {}
impl<'a> crate::painter::ClippingMethods for SvgPainter<'a> {}
impl<'a> crate::painter::StrokingMethods for SvgPainter<'a> {}
impl<'a> crate::painter::FillingMethods for SvgPainter<'a> {}
impl<'a> crate::painter::BasicShapesMethods for SvgPainter<'a> {}

impl<'a> Painter for SvgPainter<'a> {}


/// See https://developer.mozilla.org/en-US/docs/Web/CSS/mix-blend-mode
pub enum SvgBlendMode {
    // Keyword values

    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,

    // Global values

    Inherit,
    Initial,
    Revert,
    RevertLayer,
    Unset,
}

/// example: `let svg_blend_mode: Option<SvgBlendMode> = blend_mode.into();`
impl From<BlendMode> for Option<SvgBlendMode> {
    fn from(value: BlendMode) -> Self {
        match value {
            BlendMode::Clear => None,
            BlendMode::Src => None,
            BlendMode::Dst => None,
            BlendMode::SrcOver => Some(SvgBlendMode::Normal),
            BlendMode::DstOver => None,
            BlendMode::SrcIn => None,
            BlendMode::DstIn => None,
            BlendMode::SrcOut => None,
            BlendMode::DstOut => None,
            BlendMode::SrcATop => None,
            BlendMode::DstATop => None,
            BlendMode::Xor => None,
            BlendMode::Plus => None,
            BlendMode::PlusClamped => None,
            BlendMode::Modulate => None,
            BlendMode::Screen => Some(SvgBlendMode::Screen),
            BlendMode::Overlay => Some(SvgBlendMode::Overlay),
            BlendMode::Darken => Some(SvgBlendMode::Darken),
            BlendMode::Lighten => Some(SvgBlendMode::Lighten),
            BlendMode::ColorDodge => Some(SvgBlendMode::ColorDodge),
            BlendMode::ColorBurn => Some(SvgBlendMode::ColorBurn),
            BlendMode::HardLight => Some(SvgBlendMode::HardLight),
            BlendMode::SoftLight => Some(SvgBlendMode::SoftLight),
            BlendMode::Difference => Some(SvgBlendMode::Difference),
            BlendMode::Exclusion => Some(SvgBlendMode::Exclusion),
            BlendMode::Multiply => Some(SvgBlendMode::Multiply),
            BlendMode::Hue => Some(SvgBlendMode::Hue),
            BlendMode::Saturation => Some(SvgBlendMode::Saturation),
            BlendMode::Color => Some(SvgBlendMode::Color),
            BlendMode::Luminosity => Some(SvgBlendMode::Luminosity),
        }
    }
}

/// example:
/// ```
/// let blend_mode: Option<BlendMode> = svg_blend_mode.into();
/// ```
impl From<SvgBlendMode> for Option<BlendMode> {
    fn from(value: SvgBlendMode) -> Self {
        match value {
            SvgBlendMode::Normal => Some(BlendMode::SrcOver),
            SvgBlendMode::Multiply => Some(BlendMode::Multiply),
            SvgBlendMode::Screen => Some(BlendMode::Screen),
            SvgBlendMode::Overlay => Some(BlendMode::Overlay),
            SvgBlendMode::Darken => Some(BlendMode::Darken),
            SvgBlendMode::Lighten => Some(BlendMode::Lighten),
            SvgBlendMode::ColorDodge => Some(BlendMode::ColorDodge),
            SvgBlendMode::ColorBurn => Some(BlendMode::ColorBurn),
            SvgBlendMode::HardLight => Some(BlendMode::HardLight),
            SvgBlendMode::SoftLight => Some(BlendMode::SoftLight),
            SvgBlendMode::Difference => Some(BlendMode::Difference),
            SvgBlendMode::Exclusion => Some(BlendMode::Exclusion),
            SvgBlendMode::Hue => Some(BlendMode::Hue),
            SvgBlendMode::Saturation => Some(BlendMode::Saturation),
            SvgBlendMode::Color => Some(BlendMode::Color),
            SvgBlendMode::Luminosity => Some(BlendMode::Luminosity),

            SvgBlendMode::Inherit => None,
            SvgBlendMode::Initial => None,
            SvgBlendMode::Revert => None,
            SvgBlendMode::RevertLayer => None,
            SvgBlendMode::Unset => None,
        }
    }
}