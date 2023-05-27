use self::{polyline::BrokenPolylineCommand, path::PathCommand};

use super::defs::{rect::F64Rect, linalg::Vec2f64};

pub mod path;
pub mod polyline;

pub trait Shape: Clone + std::fmt::Debug + Send + Sync + 'static {
    type Iter: Iterator<Item = PathCommand>;

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
    fn rough_bounding_box(&self) -> F64Rect {
        self.bounding_box()
    }

    /// Returns a rough bounding box that contains the shape.
    ///
    /// Returns a rectangle that is **not guaranteed** to contain the shape, but it
    /// is an approximation that could be used for culling.
    ///
    /// # Notes
    ///  * This method could be faster than [`bounding_box`](Shape::bounding_box) in some cases.
    ///  * This method is **not guaranteed** to contain the shape.
    fn culling_bounding_box(&self) -> F64Rect {
        self.bounding_box()
    }

    /// Returns a basic shape that is equivalent to this shape.
    fn to_basic_shape(&self) -> Option<BasicShape>;

    fn to_path_iter(&self) -> Self::Iter;
}

impl Shape for F64Rect {
    type Iter = std::array::IntoIter<PathCommand, 4>;

    fn bounding_box(&self) -> F64Rect {
        *self
    }

    fn to_basic_shape(&self) -> Option<BasicShape> {
        Some(BasicShape::Rect(*self))
    }

    fn to_path_iter(&self) -> Self::Iter {
        todo!()
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum BasicShape {
    Rect(F64Rect),
    Path(Vec<path::PathCommand>),
    Polyline(Vec<BrokenPolylineCommand>),
    Circle{ center: Vec2f64, radius: f64 },
    // TODO ellipse, rounded rect, ...
    // TODO ...
}

impl Shape for BasicShape {
    type Iter = std::vec::IntoIter<PathCommand>;

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

    fn to_basic_shape(&self) -> Option<BasicShape> {
        Some(self.clone())
    }

    fn to_path_iter(&self) -> Self::Iter {
        match self {
            BasicShape::Rect(rect) => {
                let mut path = Vec::with_capacity(4);
                path.push(PathCommand::MoveTo(rect.min));
                path.push(PathCommand::LineTo(Vec2f64::new(rect.max.x, rect.min.y)));
                path.push(PathCommand::LineTo(rect.max));
                path.push(PathCommand::LineTo(Vec2f64::new(rect.min.x, rect.max.y)));
                path.push(PathCommand::ClosePath);
                path.into_iter()
            }
            BasicShape::Path(path) => path.clone().into_iter(),
            BasicShape::Polyline(_polyline) => todo!(), // TODO
            BasicShape::Circle{ center, radius } => {
                let path = vec![
                    PathCommand::MoveTo(Vec2f64::new(center.x - radius, center.y)),
                    PathCommand::EllipticalArcTo {
                        radii: Vec2f64::new(*radius, *radius),
                        x_axis_rotation: 0.0,
                        large_arc_flag: false,
                        sweep_flag: true,
                        end_pt: Vec2f64::new(center.x + radius, center.y),
                    },
                ];
                path.into_iter()
            }
        }
    }
}