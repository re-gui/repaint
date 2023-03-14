/*!

Provides the [`Painter`] trait.

*/

use crate::base::blending::BlendMode;
//use crate::canvas::Canvas;
use crate::base::defs::antialiasing::AntialiasMode;
use crate::base::defs::rect::{FRect, VideoRect};
use crate::base::paint::Paint;
use crate::base::pen::Pen;
use crate::base::shapes::path::PathCommand;

use crate::base::defs::linalg::*;

/// A painter that can be used to draw on a [`Canvas`](`crate::canvas::Canvas`).
///
/// In order to organize the drawing process, the painter
/// trait is split into conceptually separate parts that provide
/// different functionality:
///  - [`BlendModeMethods`] contains the methods related to blend modes
///  - [`AntialiasMethods`] contains the methods related to antialiasing
///  - [`TransformMethods`] contains the methods related to transformations
///  - [`ClippingMethods`] contains the methods related to clipping
///  - [`StrokingMethods`] contains the methods related to stroking
///  - [`FillingMethods`] contains the methods related to filling shapes
///  - [`BasicShapesMethods`] contains basic shapes
pub trait Painter: BlendModeMethods + AntialiasMethods + TransformMethods + ClippingMethods + StrokingMethods + FillingMethods + BasicShapesMethods {
}

/// Methods related to blend modes.
pub trait BlendModeMethods {

    /// Returns whether this painter can set the given blend mode.
    fn is_blend_mode_valid(&self, _mode: BlendMode) -> bool {
        false
    }

    /// Returns an iterator over all valid blend modes.
    fn enumerate_valid_blend_modes<'s>(&'s self) -> Box<dyn Iterator<Item = BlendMode> + 's> {
        Box::new(BlendMode::enumerate_all().filter(move |mode| self.is_blend_mode_valid(*mode)))
    }
}

/// Methods related to antialiasing.
pub trait AntialiasMethods {
    /// Returns whether this painter supports antialiasing.
    fn has_antialias(&self) -> bool {
        false
    }

    /// Returns whether this painter can set the given antialias mode.
    fn can_set_antialias_mode(&self, _mode: AntialiasMode) -> bool {
        self.has_antialias()
    }

    /// Returns whether this painter can change the antialias mode at this time.
    /// Some painters may not be able to change the antialias mode while drawing but only
    /// before or after. ???
    fn can_change_antialias_mode_now(&self) -> bool {
        false
    }

    /// Returns the current antialias mode.
    fn antialias_mode(&self) -> AntialiasMode {
        AntialiasMode::None
    }

    /// Sets the antialias mode. If the given antialias mode is not supported by this painter,
    /// the effective antialias mode will be undefined.
    fn set_antialias_mode(&mut self, _mode: AntialiasMode) {
        // ...
    }

    /// Tells whether antialiasing is enabled.
    fn antialiased(&self) -> bool {
        self.antialias_mode() != AntialiasMode::None
    }
}

pub trait TransformMethods {
}

pub trait ClippingMethods {
}

pub trait StrokingMethods: TransformMethods + ClippingMethods {

    fn can_stroke(&self) -> bool {
        false
    }

    /// Draw a point at `pos`.
    /// 
    /// # Arguments
    /// 
    /// * `pos` - The position of the point to draw.
    /// * `paint` - Pa to use for drawing the point.
    fn draw_point(&mut self, pos: Vec2f, paint: &Paint);

    /// Draw a line from `start` to `end`.
    /// 
    /// # Arguments
    /// 
    /// * `start` - The start position of the line.
    /// * `end` - The end position of the line.
    /// * `pen` - pen to use for drawing the line.
    fn stroke_line(&mut self, start: Vec2f, end: Vec2f, pen: &Pen);

    /// Stroke a path.
    /// 
    /// # Arguments
    /// 
    /// * `path` - iterator over the path commands.
    /// * `pen` - pen to use for drawing the path.
    fn stroke_path(&mut self, path: &mut dyn Iterator<Item = &PathCommand>, pen: &Pen);
}

pub trait FillingMethods: TransformMethods + ClippingMethods {
    fn fill_path(&mut self, path: &mut dyn Iterator<Item = &PathCommand>, ink: &Paint);
}

pub trait BasicShapesMethods: StrokingMethods + FillingMethods {
    /// Draw a rectangle.
    fn stroke_rect(&mut self, rect: FRect, pen: &Pen) {
        self.stroke_path(&mut rect_to_path(rect).iter(), pen);
    }

    /// Fill a rectangle.
    fn fill_rect(&mut self, rect: FRect, ink: &Paint) {
        self.fill_path(&mut rect_to_path(rect).iter(), ink);
    }
}


/// Convert a rectangle to a path.
fn rect_to_path(rect: FRect) -> [PathCommand; 5] {
    [
        PathCommand::MoveTo(rect.top_left()),
        PathCommand::LineTo(rect.top_right()),
        PathCommand::LineTo(rect.bottom_right()),
        PathCommand::LineTo(rect.bottom_left()),
        PathCommand::ClosePath,
    ]
}