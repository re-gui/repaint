/*!

Provides the [`Painter`] trait.

*/


pub mod methods;
pub mod text;

use std::{str::FromStr};

use methods::*;
pub use text::*;
use strum::EnumIter;

use crate::{Canvas, base::{shapes::{path::{PathCommand}, Shape}, transform::Transform2d, defs::{linalg::Vec2f64, rect::F64Rect, colors::{ColorType}}, paint::{Ink, Paint}, pen::Pen, blending::BlendMode, brush::Brush}};

pub trait Context {
}

// TODO move?
#[derive(Debug, Clone)] // TODO see https://api.skia.org/classSkCanvas.html#a06bd76ce35082366bb6b8e6dfcb6f435
pub struct SaveLayerRec {
    pub bounds: Option<F64Rect>,
    // TODO paint: Option<Paint<>>, or alpha
    // TODO backdrop
    // TODO flags
}

#[derive(Debug, Clone)]
pub enum ClipOperation {
    Difference,
    Intersect,
}

pub enum PointMode {
    Points,
    Lines,
    Polygon,
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

    /// The canvas type associated with this painter.
    type Canvas: Canvas<'context>;

    /// The context type associated with this painter.
    type Context: Context;

    /// Get the canvas that this painter is drawing to.
    ///
    /// The canvas is borrowed/owned by the painter and it cannot be accessed
    /// directly as mutable reference from this trait since it is unspecified what
    /// the painter does with the canvas. Implementations could possibly provide
    /// some other way to access the canvas by other means.
    fn canvas(&self) -> &Self::Canvas;

    // TODO fn context(&mut self) -> &'context RefCell<Self::Context>;

    /// Returns whether this painter can set the given blend mode.
    ///
    /// `BlendMode::enumerate_all().filter(move |mode| self.is_blend_mode_valid(*mode))`
    fn is_blend_mode_valid(&self, mode: BlendMode) -> bool;

    /// TODO doc
    fn with_save<'a, R>(
        &'a mut self,
        paint: impl FnOnce(&mut Self) -> R,
    ) -> R;

    // creates an isolated layer
    fn with_save_layer<'a, R>(
        &'a mut self,
        paint: impl FnOnce(&mut Self) -> R,
        layer_rec: SaveLayerRec,
    ) -> R;

    // TODO possibility to cache layers?

    /// Transforms the painter's coordinate system.
    /// 
    /// Let $T$ be the current transformation matrix and $M$ the given transformation matrix.
    /// The new transformation matrix will be $T \cdot M$.
    fn set_transform(&mut self, transform: &Transform2d) -> Result<(), TransformError>;

    fn concatenate_transform(&mut self, transform: &Transform2d) -> Result<(), TransformError>;

    fn translate(&mut self, offset: Vec2f64) -> Result<(), TransformError> {
        self.concatenate_transform(&Transform2d::Translate(offset))
    }

    fn rotate(&mut self, angle: f64) -> Result<(), TransformError> {
        self.concatenate_transform(&Transform2d::Rotate(angle))
    }

    /// TODO ???
    fn clip(&mut self, shape: &impl Shape, clip_operation: ClipOperation) -> Result<(), ClipError>;

    fn point(
        &mut self,
        pos: Vec2f64,
        style: PaintStyle<Self::NativeColor>,
    );

    fn points(
        &mut self,
        points: impl IntoIterator<Item = Vec2f64>,
        style: PaintStyle<Self::NativeColor>,
        point_mode: PointMode,
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

    fn fill_with(
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
        println!("rect: {:?}", rect);
        self.draw_path_iter(
            &mut rect_to_path(rect).iter().cloned(),
            style,
        );
    }

    // TODO ...
}

pub trait RasterPainter<'context>: BasicPainter<'context> {
    /// Returns whether this painter supports antialiasing.
    fn has_antialias(&self) -> bool;

    fn pixel(
        &mut self,
        pos: Vec2f64,
        ink: Ink<Self::NativeColor>,
    );

    // TODO other methods to efficiently draw pixels
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

pub trait Painter<'context>: BasicPainter<'context> + WithPathResource<'context> + WithText<'context> {}

impl<'context, T> Painter<'context> for T where T: BasicPainter<'context> + WithPathResource<'context> + WithText<'context> {}

//pub struct PainterResourceAccessor<'painter>(&'painter dyn Painter); // TODO dyn meybe unnecessary