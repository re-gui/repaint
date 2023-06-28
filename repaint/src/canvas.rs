/*!

Provides the [`Canvas`] trait.

In this module we define the [`Canvas`] trait that represents a drawing surface and provides a painter that can be used to draw on it as well as some utility functions and types.

*/

use std::error::Error;

use strum::Display;

use crate::{painter::BasicPainter, base::shapes::Shape};



/// An object that can be drawn on.
/// 
/// The canvas provides a painter that can be used to draw on it, see the [repaint architecture](crate#architecture) for more details on how this works conceptually.
pub trait Canvas {
    type Shape: Shape;

    //type Painter<'canvas>: BasicPainter where Self: 'canvas;
    type Painter<'s>: BasicPainter where Self: 's;

    /// Returns a [`Painter`] that can be used to paint on the canvas.
    /// 
    /// Each canvas can provide a different painter in order to interact with the actual drawing surface
    /// and possibly optimize the drawing process.
    /// 
    /// Since the painter is dynamic, it is returned as a [`Box`] to avoid the need to know the exact type at compile time. In the cases where the boxing is not necessary, for example because it is a reference or a sub-region painter, the [`Canvas::painter_compact`] method can be used to avoid the need to allocate a [`Box`].
    fn painter<'s>(&'s mut self) -> Result<Self::Painter<'s>, GetPainterError>;

    // TODO
    fn shape(&self) -> Self::Shape;
}

#[derive(Debug, Display)]
pub enum GetPainterError {
    Unknown,
    OnlyOnePainterAllowed,
}

impl Error for GetPainterError {}
