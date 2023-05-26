/*!

Provides the [`Painter`] trait.

*/


pub mod methods;

use methods::*;

use crate::{Canvas, base::{shapes::{path::{PathCommand}, Shape}, transform::Transform2d, defs::{linalg::Vec2f64, rect::F64Rect, colors::{ColorType}}, paint::{Ink, Paint}, pen::Pen, blending::BlendMode, brush::Brush}};

pub trait Context {
}

/// A painter that can be used to draw on a [`Canvas`].
///
/// # Notes
///  * Although a [`BasicPainter`] is just a trait, it is conceptually useful to keep in mind
///    that painters are not meant to be isolated objects, but they should exist in relation
///    to some [`Canvas`] they *paint* on.
pub trait BasicPainter<'context> {
    /// The prefferred color type of the painter.
    ///
    /// This is the preferred color type to be used with this painter as this is likely to
    /// match the internal color representation or, at least, it is likely to be the most
    /// efficient/accurate color type to use.
    type NativeColor: ColorType;

    //type Resources: Resources;
    type Canvas: Canvas<'context>;
    type Context: Context;

    /// Get the canvas that this painter is drawing to.
    ///
    /// The canvas is borrowed/owned by the painter and it cannot be accessed
    /// directly as mutable reference from this trait since it is unspecified what
    /// the painter does with the canvas. Implementations could possibly provide
    /// some other way to access the canvas by other means.
    fn canvas(&self) -> &Self::Canvas;

    //fn context(&mut self) -> &'context RefCell<Self::Context>;

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
    fn clip(&self, shape: &impl Shape) -> Result<(), ClipError>;

    fn point(
        &mut self,
        pos: Vec2f64,
        style: PaintStyle<Self::NativeColor>,
    );

    fn pixel(
        &mut self,
        pos: Vec2f64,
        ink: Ink<Self::NativeColor>,
    );

    fn line(
        &mut self,
        start: Vec2f64,
        end: Vec2f64,
        pen: Pen<Self::NativeColor>,
    );

    fn draw_path_iter<'s, 'a>(
        &'s mut self,
        path_iter: &mut dyn Iterator<Item = PathCommand>,
        style: PaintStyle<Self::NativeColor>,
    );

    // TODO this feature might not be supported because it is equivalent to the blend mode `Src`
    fn clear(
        &mut self,
        color: Self::NativeColor,
    );

    fn clear_with(
        &mut self,
        paint: &Paint<Self::NativeColor>,
    );

    // TODO clear, example for skia: surface.canvas().clear(Color::WHITE);

    /// Draw a rectangle.
    fn rect(
        &mut self,
        rect: F64Rect,
        style: PaintStyle<Self::NativeColor>,
    ) {
        self.draw_path_iter(
            &mut rect_to_path(rect).iter().cloned(),
            style,
        );
    }

    // TODO ...
}

pub trait WithPathResource<'context>: BasicPainter<'context> {
    type Path; // TODO bounds

    fn make_path(
        &mut self,
        path_iter: impl Iterator<Item = PathCommand>
    ) -> Result<Self::Path, ()>;

    fn path(&mut self, path: &Self::Path, style: PaintStyle<Self::NativeColor>);

    fn sweep(
        &mut self,
        path: &Self::Path,
        brush: &impl Brush<'context, Self>,
    ) {
        brush.sweep(self, path);
    }
}

pub trait Painter<'context>: BasicPainter<'context> + WithPathResource<'context> {}

impl<'context, T> Painter<'context> for T where T: BasicPainter<'context> + WithPathResource<'context> {}

//pub struct PainterResourceAccessor<'painter>(&'painter dyn Painter); // TODO dyn meybe unnecessary