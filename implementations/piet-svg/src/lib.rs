/*!
A Piet backend that renders to SVG.
*/

#![warn(missing_docs)]

use std::any::Any;

use piet::{kurbo::Size as PietSize, RenderContext};
use repaint::{painter::methods::{BlendModeMethods, AntialiasMethods, TransformMethods, TransformError, StrokingMethods, FillingMethods, BasicShapesMethods}, base::{blending::BlendMode, transform::Transform2d, defs::linalg::Vec2f64, paint::Paint}, Painter, Canvas};

/// A Piet backend that renders to SVG.
pub struct PietRefBackend<'a, PietContext>
where
    PietContext: piet::RenderContext,
{
    /// The underlying SVG rendering context.
    pub piet: &'a mut PietContext,
}

trait TmpTrait {
    fn tmp(&self);
    fn tmp_mut(&mut self);
}

impl<'a, PietContext> TmpTrait for PietRefBackend<'a, PietContext>
where
    PietContext: piet::RenderContext,
{
    fn tmp(&self) {
    }
    fn tmp_mut(&mut self) {
    }
}

impl<'a, PietContext> PietRefBackend<'a, PietContext>
where
    PietContext: piet::RenderContext,
{
    /// Create a new [`PietRefBackend`].
    pub fn new(piet: &'a mut PietContext) -> Self {
        Self { piet }
    }

    pub fn tmp<'b>(&'b mut self) -> Box<dyn TmpTrait> {
        struct TmpPainter<'c, 'd, PietContext: piet::RenderContext> {
            painter: &'c mut PietRefBackend<'d, PietContext>,
        }

        impl<'c, 'd, PietContext: piet::RenderContext> TmpTrait for TmpPainter<'c, 'd, PietContext> {
            fn tmp(&self) {
            }
            fn tmp_mut(&mut self) {
            }
        }

        return Box::new(TmpPainter::<'b, 'a, PietContext> {
            painter: self,
        });
    }

    fn ciao(p: &mut Self) {
        let mut s = p.tmp();
        let mut s2 = p.tmp();
    }
}

impl<'a, PietContext> BlendModeMethods for PietRefBackend<'a, PietContext>
where
    PietContext: piet::RenderContext,
{
    fn enumerate_valid_blend_modes<'s>(&'s self) -> Box<dyn Iterator<Item = BlendMode> + 's> {
        Box::new([
            BlendMode::SrcOver,
        ].into_iter())
    }
}

impl<'a, PietContext> AntialiasMethods for PietRefBackend<'a, PietContext>
where
    PietContext: piet::RenderContext,
{
    // no antialias support for SVG
}

impl<'a, PietContext> TransformMethods for PietRefBackend<'a, PietContext>
where
    PietContext: piet::RenderContext,
{
    /*fn with_save(&mut self, f: &mut dyn FnOnce(&mut dyn repaint::Painter)) {

        //self.piet.with_save(|piet| {
        //    f(&mut PietRefBackend::new(piet));
        //    Ok(())
        //});
        self.piet.save();
        f(self);
        self.piet.restore();
    }*/

    fn with_save<'b>(&'a mut self) -> Box<dyn Painter> {
        struct SavedPainter {
            piet: PietContext,
        }

        todo!()
    }

    fn transform(&mut self, transform: &Transform2d) -> Result<(), TransformError> {
        let (
            deformation,
            translation
        ) = transform.to_affine().ok_or(TransformError::Unsupported)?;

        self.piet.transform(piet::kurbo::Affine::new([
            deformation[(0, 0)], deformation[(0, 1)],
            deformation[(1, 0)], deformation[(1, 1)],
            translation.x, translation.y,
        ]));

        Ok(())
    }

    fn clip(&self, shape: &dyn repaint::base::shapes::Shape) -> Result<(), repaint::painter::methods::ClipError> {
        todo!()
    }
}

impl<'a, PietContext> StrokingMethods for PietRefBackend<'a, PietContext>
where
    PietContext: piet::RenderContext,
{
    fn can_stroke(&self) -> bool {
        true
    }

    fn draw_point(&mut self, pos: Vec2f64, paint: &Paint) {
        todo!()
    }

    fn stroke_line(&mut self, start: Vec2f64, end: Vec2f64, pen: &repaint::base::pen::Pen) {
        todo!()
    }

    fn stroke_path(&mut self, path: &mut dyn Iterator<Item = &repaint::base::shapes::path::PathCommand>, pen: &repaint::base::pen::Pen) {
        todo!()
    }

    fn stroke_shape(&mut self, shape: &dyn repaint::base::shapes::Shape, pen: &repaint::base::pen::Pen) {
        todo!()
    }
}

impl<'a, PietContext> FillingMethods for PietRefBackend<'a, PietContext>
where
    PietContext: piet::RenderContext,
{
    fn can_fill(&self) -> bool {
        true
    }

    fn fill_path(&mut self, path: &mut dyn Iterator<Item = &repaint::base::shapes::path::PathCommand>, ink: &Paint) {
        todo!()
    }

    fn fill_shape(&mut self, shape: &dyn repaint::base::shapes::Shape, ink: &Paint) {
        todo!()
    }
}

impl<'a, PietContext> BasicShapesMethods for PietRefBackend<'a, PietContext>
where
    PietContext: piet::RenderContext,
{
}

impl<'a, PietContext> Painter for PietRefBackend<'a, PietContext>
where
    PietContext: piet::RenderContext,
{
    fn canvas(&self) -> &dyn repaint::Canvas {
        todo!()
    }
}