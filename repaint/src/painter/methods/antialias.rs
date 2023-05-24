use std::error::Error;

use strum::Display;

use crate::base::defs::antialiasing::AntialiasMode;



/// Methods related to antialiasing.
pub trait AntialiasMethods {
    /// Returns whether this painter supports antialiasing.
    fn has_antialias(&self) -> bool;
}