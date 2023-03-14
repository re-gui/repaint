use std::{error::Error, path::Path};

use crate::{Canvas, base::{defs::{linalg::Vec2f, rect::FRect}, shapes::Shape, blending::BlendMode, transform::Transform2d}, Painter, painter::methods::{ClipError, TransformError}};

mod blend_modes;
pub use blend_modes::*;

use crate::painter::methods;

pub struct SvgCanvas {
    doc: svg::Document,
    viewbox: FRect,
}

impl SvgCanvas {
    /// Create a new [`SvgCanvas`] with the given viewbox.
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

    /// Save the SVG document to a file
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let contents = self.doc.to_string();
        std::fs::write(path, contents)?;
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

impl<'a> methods::BlendModeMethods for SvgPainter<'a> {
    fn is_blend_mode_valid(&self, mode: BlendMode) -> bool {
        let svg_mode: Option<SvgBlendMode> = mode.into();
        svg_mode.is_some()
    }
}

impl<'a> methods::AntialiasMethods for SvgPainter<'a> {
    fn has_antialias(&self) -> bool {
        // it makes no sense to enable/disable antialiasing in SVG
        // since it is a vector format
        false
    }
}

impl<'a> methods::TransformMethods for SvgPainter<'a> {
    fn with_save(&mut self, _f: &mut dyn FnOnce(&mut dyn Painter)) {
        unimplemented!()
    }

    fn transform(&mut self, _transform: &Transform2d) -> Result<(), TransformError> {
        unimplemented!()
    }

    fn clip(&self, _shape: Shape) -> Result<(), ClipError> {
        unimplemented!()
    }
}
impl<'a> methods::StrokingMethods for SvgPainter<'a> {
    fn draw_point(&mut self, _pos: Vec2f, _paint: &crate::base::paint::Paint) {
        unimplemented!()
    }

    fn stroke_line(&mut self, _start: Vec2f, _end: Vec2f, _pen: &crate::base::pen::Pen) {
        unimplemented!()
    }

    fn stroke_path(&mut self, _path: &mut dyn Iterator<Item = &crate::base::shapes::path::PathCommand>, _pen: &crate::base::pen::Pen) {
        unimplemented!()
    }
}
impl<'a> methods::FillingMethods for SvgPainter<'a> {
    fn fill_path(&mut self, _path: &mut dyn Iterator<Item = &crate::base::shapes::path::PathCommand>, _ink: &crate::base::paint::Paint) {
        unimplemented!()
    }
}
impl<'a> methods::BasicShapesMethods for SvgPainter<'a> {}

impl<'a> Painter for SvgPainter<'a> {
    fn canvas(&self) -> &dyn Canvas {
        self.canvas
    }
}


