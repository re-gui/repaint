/*!

Provides the [`Painter`] trait.

*/


pub mod methods;

use methods::*;

use crate::{Canvas, base::{shapes::{path::{PathCommand}, Shape}, transform::Transform2d, defs::{linalg::Vec2f64, rect::F64Rect}, paint::{Ink, Color, Paint}, pen::Pen, blending::BlendMode}};

pub trait Context {
    fn has_path_resources(&self) -> bool;
}

/// A painter that can be used to draw on a [`Canvas`].
///
/// In order to organize the drawing process, the painter
/// trait is split into conceptually separate parts that provide
/// different functionality:
///  - [`BlendModeMethods`] contains the methods related to blend modes
///  - [`AntialiasMethods`] contains the methods related to antialiasing
///  - [`TransformMethods`] contains the methods related to transformations and clipping
///  - [`DrawingMethods`] contains the methods related to drawing paths
///  - [`BasicShapesMethods`] contains basic shapes
///
/// # Notes
///  * Although a [`Painter`] is just a trait, it is conceptually useful to keep in mind
///    that painters are not meant to be isolated objects, but they should exist in relation
///    to some [`Canvas`] they *paint* on.
pub trait Painter<'context> {
    //type Resources: Resources;
    type Canvas: Canvas<'context>;
    type Context: Context;

    type Path; // TODO bounds

    /// Get the canvas that this painter is drawing to.
    /// 
    /// The canvas is borrowed/owned by the painter and it cannot be accessed
    /// directly as mutable reference from this trait since it is unspecified what
    /// the painter does with the canvas. Implementations could possibly provide
    /// some other way to access the canvas by other means.
    fn canvas(&self) -> &Self::Canvas;

    //fn context_mut(&mut self) -> &'context mut Self::Context;

    /// Returns whether this painter supports antialiasing.
    fn has_antialias(&self) -> bool;

    /// Returns whether this painter can set the given blend mode.
    fn is_blend_mode_valid(&self, _mode: BlendMode) -> bool;

    /// Returns an iterator over all valid blend modes.
    fn enumerate_valid_blend_modes<'s>(&'s self);/* -> impl Iterator<Item = BlendMode> {
        BlendMode::enumerate_all().filter(move |mode| self.is_blend_mode_valid(*mode))
    }*/
    // TODO

    /// TODO doc
    fn with_save<'a>(&'a mut self, paint: impl FnOnce(&mut Self) -> Result<(), ()>) -> Result<(), ()>;/* {
        self.save()?;
        paint(self)?;
        self.restore()
    }*/

    /// Transforms the painter's coordinate system.
    /// 
    /// Let $T$ be the current transformation matrix and $M$ the given transformation matrix.
    /// The new transformation matrix will be $T \cdot M$.
    fn transform(&mut self, transform: &Transform2d) -> Result<(), TransformError>;

    /// TODO ???
    fn clip(&self, shape: &dyn Shape) -> Result<(), ClipError>;

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

    fn make_path(
        &mut self,
        path_iter: &mut dyn Iterator<Item = PathCommand>
    ) -> Result<Self::Path, ()>;

    fn path(&mut self, path: &Self::Path, style: PaintStyle);

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
    );

    // TODO clear, example for skia: surface.canvas().clear(Color::WHITE);

    /// Draw a rectangle.
    fn rect(
        &mut self,
        rect: F64Rect,
        style: PaintStyle,
    ) {
        self.draw_path_iter(
            &mut rect_to_path(rect).iter().cloned(),
            style,
        );
    }

    // TODO ...
}

//pub struct PainterResourceAccessor<'painter>(&'painter dyn Painter); // TODO dyn meybe unnecessary