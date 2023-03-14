use std::error::Error;

use crate::{Canvas, base::{defs::{linalg::Vec2f, rect::FRect}, shapes::Shape, blending::BlendMode}, Painter};

mod blend_modes;
pub use blend_modes::*;


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


