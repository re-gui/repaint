use crate::base::{defs::linalg::Vec2f64, paint::Paint, pen::Pen, shapes::{path::PathCommand, Shape}};

use super::TransformMethods;


/// Methods for stroking paths.
pub trait StrokingMethods: TransformMethods {

    /// Tells if this painter can stroke
    fn can_stroke(&self) -> bool;

    /// Draw a point at `pos`.
    /// 
    /// # Arguments
    /// 
    /// * `pos` - The position of the point to draw.
    /// * `paint` - Pa to use for drawing the point.
    fn draw_point(&mut self, pos: Vec2f64, paint: &Paint);

    /// Draw a line from `start` to `end`.
    /// 
    /// # Arguments
    /// 
    /// * `start` - The start position of the line.
    /// * `end` - The end position of the line.
    /// * `pen` - pen to use for drawing the line.
    fn stroke_line(&mut self, start: Vec2f64, end: Vec2f64, pen: &Pen);

    /// Stroke a path.
    /// 
    /// # Arguments
    /// 
    /// * `path` - iterator over the path commands.
    /// * `pen` - pen to use for drawing the path.
    fn stroke_path(&mut self, path: &mut dyn Iterator<Item = &PathCommand>, pen: &Pen);

    /// Stroke a shape
    fn stroke_shape(&mut self, shape: &dyn Shape, pen: &Pen);
}