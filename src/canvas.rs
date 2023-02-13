/*!

Provides the [`Canvas`] trait.

In this module we define the [`Canvas`] trait that represents a drawing surface and provides a painter that can be used to draw on it as well as some utility functions and types.

*/

use crate::painter::{Painter, WithBlendMode};



/// An object that can be drawn on.
/// 
/// The canvas provides a painter that can be used to draw on it, see the [repaint architecture](crate#architecture) for more details on how this works conceptually.
pub trait Canvas {
    /// Returns a [`Painter`] that can be used to paint on the canvas.
    /// 
    /// Each canvas can provide a different painter in order to interact with the actual drawing surface
    /// and possibly optimize the drawing process.
    /// 
    /// Since the painter is dynamic, it is returned as a [`Box`] to avoid the need to know the exact type at compile time. In the cases where the boxing is not necessary, for example because it is a reference or a sub-region painter, the [`Canvas::painter_compact`] method can be used to avoid the need to allocate a [`Box`].
    fn painter<'s>(&'s mut self) -> Box<dyn Painter + 's>;

    /// Returns a compact painter that can be used to paint on the canvas.
    /// The goal is to avoid the need to allocate a Box for the painter if possible.
    /// 
    /// This might reduce dynamic dispatch in the cases the painter falls into one of the
    /// compact [`CompactPainter`] variants.
    fn painter_compact<'s>(&'s mut self) -> CompactPainter<'s> {
        CompactPainter::Boxed(self.painter())
    }
}

/// A finite set of known painters that can be uses to try to reduce boxing while still allowing dynamic dispatch.
pub enum CompactPainter<'a> {
    Reference(&'a mut dyn Painter),
    Boxed(Box<dyn Painter + 'a>),
    // TODO known types of painters, for example a sub-region painter
}

impl<'a> CompactPainter<'a> {
    pub fn painter(&self) -> &dyn Painter {
        match self {
            CompactPainter::Reference(painter) => *painter,
            CompactPainter::Boxed(painter) => painter.as_ref(),
        }
    }
}

/// [`CompactPainter`] is also a painter!
impl Painter for CompactPainter<'_> {
}

impl WithBlendMode for CompactPainter<'_> {
}