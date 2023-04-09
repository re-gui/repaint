use self::polyline::BrokenPolylineCommand;

use super::defs::{rect::FRect, linalg::Vec2f};

pub mod path;
pub mod polyline;


pub enum Shape {
    Rect(FRect),
    Path(Vec<path::PathCommand>),
    Polyline(Vec<BrokenPolylineCommand>),
    Circle{ center: Vec2f, radius: f32 },
    // TODO ellipse,
    // TODO ...
}

impl Shape {
    /// Returns the exact bounding box of the shape.
    ///
    /// This is the smallest rectangle that contains the shape.
    ///
    /// # Notes
    ///  * This method might be **very expensive** to compute, especially for complex shapes.
    ///    In many cases, [`rough_bounding_box`](Shape::rough_bounding_box) is a better choice.
    pub fn bounding_box(&self) -> FRect {
        match self {
            Shape::Rect(rect) => *rect,
            Shape::Path(_path) => unimplemented!(),
            Shape::Polyline(_polyline) => unimplemented!(),
            Shape::Circle{ center: _, radius: _ } => unimplemented!(),
        }
    }

    /// Returns a rough bounding box that contains the shape.
    ///
    /// Returns a rectangle that is guaranteed to contain the shape, but might be larger than the
    /// exact bounding box.
    ///
    /// # Notes
    ///  * This method could be faster than [`bounding_box`](Shape::bounding_box) in some cases.
    pub fn rough_bounding_box(&self) -> FRect {
        match self {
            Shape::Rect(rect) => *rect,
            Shape::Path(_path) => unimplemented!(),
            Shape::Polyline(_polyline) => unimplemented!(),
            Shape::Circle{ center: _, radius: _ } => unimplemented!()
        }
    }

    /// Returns a rough bounding box that contains the shape.
    ///
    /// Returns a rectangle that is **not guaranteed** to contain the shape, but it
    /// is an approximation that could be used for culling.
    ///
    /// # Notes
    ///  * This method could be faster than [`bounding_box`](Shape::bounding_box) in some cases.
    ///  * This method is **not guaranteed** to contain the shape.
    pub fn culling_bounding_box(&self) -> FRect {
        unimplemented!()
    }
}