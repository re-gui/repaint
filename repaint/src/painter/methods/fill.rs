use crate::base::{shapes::path::PathCommand, paint::Paint};

use super::TransformMethods;


/// Methods for filling paths.
pub trait FillingMethods: TransformMethods {
    fn fill_path(&mut self, path: &mut dyn Iterator<Item = &PathCommand>, ink: &Paint);
}