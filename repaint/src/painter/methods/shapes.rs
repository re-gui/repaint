use crate::base::{defs::rect::{F64Rect, VideoRect}, shapes::path::PathCommand};



// TODO move
/// Convert a rectangle to a path.
pub fn rect_to_path(rect: F64Rect) -> [PathCommand; 5] {
    // TODO move this to the path module
    [
        PathCommand::MoveTo(rect.top_left()),
        PathCommand::LineTo(rect.top_right()),
        PathCommand::LineTo(rect.bottom_right()),
        PathCommand::LineTo(rect.bottom_left()),
        PathCommand::ClosePath,
    ]
}