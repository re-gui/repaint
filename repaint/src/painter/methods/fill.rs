use crate::base::{shapes::{path::PathCommand, Shape}, paint::Paint};

use super::TransformMethods;


/// Methods for filling paths.
pub trait FillingMethods: TransformMethods {
    /// Indicates whether this painter can fill a path.
    fn can_fill(&self) -> bool;
    /// Fills the interior of the given path.
    fn fill_path(&mut self, path: &mut dyn Iterator<Item = &PathCommand>, ink: &Paint);
    /// Fills the interior of the given shape.
    fn fill_shape(&mut self, shape: &dyn Shape, ink: &Paint);
}