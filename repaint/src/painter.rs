/*!

Provides the [`Painter`] trait.

*/


pub mod methods;

use std::cell::RefCell;

use methods::*;

use crate::{Canvas, base::shapes::path::{PathCommand, PathResource}};

pub trait Context {
    fn has_path_resources(&self) -> bool;

    fn make_path<'context>(
        &'context mut self,
        path_iter: &mut dyn Iterator<Item = PathCommand>
    ) -> Result<PathResource<'context>, ()>;
}

/// A painter that can be used to draw on a [`Canvas`].
///
/// In order to organize the drawing process, the painter
/// trait is split into conceptually separate parts that provide
/// different functionality:
///  - [`BlendModeMethods`] contains the methods related to blend modes
///  - [`AntialiasMethods`] contains the methods related to antialiasing
///  - [`TransformMethods`] contains the methods related to transformations and clipping
///  - [`DrawingMethods`] contains the methods related to drawing paths
///  - [`BasicShapesMethods`] contains basic shapes
///
/// # Notes
///  * Although a [`Painter`] is just a trait, it is conceptually useful to keep in mind
///    that painters are not meant to be isolated objects, but they should exist in relation
///    to some [`Canvas`] they *paint* on.
pub trait Painter<'context>: BlendModeMethods + AntialiasMethods + TransformMethods + DrawingMethods + BasicShapesMethods {
    /// Get the canvas that this painter is drawing to.
    /// 
    /// The canvas is borrowed/owned by the painter and it cannot be accessed
    /// directly as mutable reference from this trait since it is unspecified what
    /// the painter does with the canvas. Implementations could possibly provide
    /// some way to access the canvas by other means.
    fn canvas(&self) -> &dyn Canvas<'context>;

    fn context(&self) -> &'context RefCell<&dyn Context> {
        todo!("remove deault impl");
    }
}

//pub struct PainterResourceAccessor<'painter>(&'painter dyn Painter); // TODO dyn meybe unnecessary