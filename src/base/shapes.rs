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
    pub fn bounding_box(&self) -> FRect {
        match self {
            Shape::Rect(rect) => *rect,
            Shape::Path(_path) => unimplemented!(),
            Shape::Polyline(_polyline) => unimplemented!(),
            Shape::Circle{ center: _, radius: _ } => unimplemented!(),
        }
    }
}