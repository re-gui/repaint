use std::error::Error;

use strum::Display;

use crate::{Painter, base::{transform::Transform2d, shapes::Shape}};

#[cfg_attr(doc, katexit::katexit)]
/// Methods related to transformations and clipping.
pub trait TransformMethods {
    fn with_save(&mut self, f: &mut dyn FnOnce(&mut dyn Painter));

    /// Transforms the painter's coordinate system.
    /// 
    /// Let $T$ be the current transformation matrix and $M$ the given transformation matrix.
    /// The new transformation matrix will be $T \cdot M$.
    fn transform(&mut self, transform: &Transform2d) -> Result<(), TransformError>;

    fn clip(&self, shape: Shape) -> Result<(), ClipError>;
}

/// Error that could occur when transforming the painter's coordinate system.
#[derive(Debug, Display)]
pub enum TransformError {
    /// The given transformation could not be applied for an unknown reason.
    CannotTransform(Box<dyn Error>),

    /// The given transformation is not supported by this painter.
    Unsupported,

    /// The given transformation is invalid, this could be returned when invalid
    /// values are passed to the [`transform`] method, for example when the
    /// transformation matrix is not invertible and/or not finite.
    /// 
    /// [`transform`]: TransformMethods::transform
    InvalidTransform,

    /// Unknown error.
    Other(Box<dyn Error>)
}

impl Error for TransformError {}

/// Error that could occur when clipping the painter's coordinate system.
#[derive(Debug, Display)]
pub enum ClipError {
    /// The painter does not support clipping.
    Unsupported,

    /// The given shape is not supported by this painter.
    UnsupportedShape,

    /// The given shape is invalid, this could be returned when invalid
    /// values are passed to the [`clip`] method.
    /// 
    /// [`clip`]: TransformMethods::clip
    InvalidShape,

    /// Unknown error.
    Other(Box<dyn Error>)
}

impl Error for ClipError {}