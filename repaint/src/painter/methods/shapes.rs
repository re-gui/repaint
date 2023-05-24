use crate::base::{pen::Pen, defs::rect::{F64Rect, VideoRect}, paint::Paint, shapes::path::PathCommand};

use super::{DrawingMethods, PaintStyle};


/// Methods for drawing basic shapes.
pub trait BasicShapesMethods: DrawingMethods {
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


/// Convert a rectangle to a path.
fn rect_to_path(rect: F64Rect) -> [PathCommand; 5] {
    // TODO move this to the path module
    [
        PathCommand::MoveTo(rect.top_left()),
        PathCommand::LineTo(rect.top_right()),
        PathCommand::LineTo(rect.bottom_right()),
        PathCommand::LineTo(rect.bottom_left()),
        PathCommand::ClosePath,
    ]
}