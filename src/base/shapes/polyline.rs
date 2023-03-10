use crate::base::defs::linalg::*;

/// A command for a broken polyline, i.e. multiple detached polylines.
#[derive(Clone, Copy, Debug)]
pub enum BrokenPolylineCommand {
    /// Move to a point without creating a line.
    MoveTo(Vec2f),

    /// Make a line to a point.
    LineTo(Vec2f),
}
