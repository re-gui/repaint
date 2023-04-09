/*!

Provides the [`Painter`] trait.

*/


pub mod methods;

use methods::*;

use crate::Canvas;

/// A painter that can be used to draw on a [`Canvas`].
///
/// In order to organize the drawing process, the painter
/// trait is split into conceptually separate parts that provide
/// different functionality:
///  - [`BlendModeMethods`] contains the methods related to blend modes
///  - [`AntialiasMethods`] contains the methods related to antialiasing
///  - [`TransformMethods`] contains the methods related to transformations and clipping
///  - [`StrokingMethods`] contains the methods related to stroking
///  - [`FillingMethods`] contains the methods related to filling shapes
///  - [`BasicShapesMethods`] contains basic shapes
///
/// # Notes
///  * Although a [`Painter`] is just a trait, it is conceptually useful to keep in mind
///    that painters are not meant to be isolated objects, but they should exist in relation
///    to some [`Canvas`] they *paint* on.
pub trait Painter: BlendModeMethods + AntialiasMethods + TransformMethods + StrokingMethods + FillingMethods + BasicShapesMethods {
    /// Get the canvas that this painter is drawing to.
    /// 
    /// The canvas is borrowed/owned by the painter and it cannot be accessed
    /// directly as mutable reference from this trait since it is unspecified what
    /// the painter does with the canvas. Implementations could possibly provide
    /// some way to access the canvas by other means.
    fn canvas(&self) -> &dyn Canvas;
}

