use crate::base::{pen::Pen, defs::rect::{FRect, VideoRect}, paint::Paint, shapes::path::PathCommand};

use super::{StrokingMethods, FillingMethods};


/// Methods for drawing basic shapes.
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
    // TODO move this to the path module
    [
        PathCommand::MoveTo(rect.top_left()),
        PathCommand::LineTo(rect.top_right()),
        PathCommand::LineTo(rect.bottom_right()),
        PathCommand::LineTo(rect.bottom_left()),
        PathCommand::ClosePath,
    ]
}