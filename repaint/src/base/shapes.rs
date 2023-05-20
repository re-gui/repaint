use self::polyline::BrokenPolylineCommand;

use super::defs::{rect::F64Rect, linalg::Vec2f64};

pub mod path;
pub mod polyline;

pub trait Shape {
    /// Returns the exact bounding box of the shape.
    ///
    /// This is the smallest rectangle that contains the shape.
    ///
    /// # Notes
    ///  * This method might be **very expensive** to compute, especially for complex shapes.
    ///    In many cases, [`rough_bounding_box`](Shape::rough_bounding_box) is a better choice.
    fn bounding_box(&self) -> F64Rect;

    /// Returns a rough bounding box that contains the shape.
    ///
    /// Returns a rectangle that is guaranteed to contain the shape, but might be larger than the
    /// exact bounding box.
    ///
    /// # Notes
    ///  * This method could be faster than [`bounding_box`](Shape::bounding_box) in some cases.
    fn rough_bounding_box(&self) -> F64Rect;

    /// Returns a rough bounding box that contains the shape.
    ///
    /// Returns a rectangle that is **not guaranteed** to contain the shape, but it
    /// is an approximation that could be used for culling.
    ///
    /// # Notes
    ///  * This method could be faster than [`bounding_box`](Shape::bounding_box) in some cases.
    ///  * This method is **not guaranteed** to contain the shape.
    fn culling_bounding_box(&self) -> F64Rect;

    /// Returns a basic shape that is equivalent to this shape.
    fn to_basic_shape(&self) -> BasicShape;

    fn path_commands(&self) -> Box<dyn Iterator<Item = path::PathCommand>>;
}


#[derive(Clone, Debug, PartialEq)]
pub enum BasicShape {
    Rect(F64Rect),
    Path(Vec<path::PathCommand>),
    Polyline(Vec<BrokenPolylineCommand>),
    Circle{ center: Vec2f64, radius: f32 },
    // TODO ellipse,
    // TODO ...
}

impl Shape for BasicShape {
    fn bounding_box(&self) -> F64Rect {
        match self {
            BasicShape::Rect(rect) => *rect,
            BasicShape::Path(_path) => todo!(),
            BasicShape::Polyline(_polyline) => todo!(),
            BasicShape::Circle{ center: _, radius: _ } => todo!(),
        }
    }

    fn rough_bounding_box(&self) -> F64Rect {
        match self {
            BasicShape::Rect(rect) => *rect,
            BasicShape::Path(_path) => todo!(),
            BasicShape::Polyline(_polyline) => todo!(),
            BasicShape::Circle{ center: _, radius: _ } => todo!()
        }
    }

    fn culling_bounding_box(&self) -> F64Rect {
        unimplemented!()
    }

    fn to_basic_shape(&self) -> BasicShape {
        self.clone()
    }

    fn path_commands(&self) -> Box<dyn Iterator<Item = path::PathCommand>> {
        todo!()
    }
}