/*!

Provides the [`Canvas`] trait.

In this module we define the [`Canvas`] trait that represents a drawing surface and provides a painter that can be used to draw on it as well as some utility functions and types.

*/

use std::ops::{Deref, DerefMut};

use crate::painter::Painter;



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
    fn painter<'s>(&'s mut self) -> Result<Box<dyn Painter + 's>, GetPainterError>;

    /// Returns a compact painter that can be used to paint on the canvas.
    /// The goal is to avoid the need to allocate a Box for the painter if possible.
    /// 
    /// This might reduce dynamic dispatch in the cases the painter falls into one of the
    /// compact [`CompactPainter`] variants.
    fn painter_compact<'s>(&'s mut self) -> Result<CompactPainter<'s>, GetPainterError> {
        //CompactPainter::Boxed(self.painter())
        if let Ok(painter) = self.painter() {
            Ok(CompactPainter::Boxed(painter))
        } else {
            Err(GetPainterError::Unknown)
        }
    }
}

pub enum GetPainterError {
    Unknown,
    OnlyOnePainterAllowed,
}

/// A finite set of known painters that can be uses to try to reduce boxing while still allowing dynamic dispatch.
pub enum CompactPainter<'a> {
    Reference(&'a mut dyn Painter),
    Boxed(Box<dyn Painter + 'a>),
    // TODO known types of painters, for example a sub-region painter
}

// now we try to make CompactPainter to behave like Box so that we can use it as a painter
// to do that, we implement some traits that Box implements

impl<'a> Deref for CompactPainter<'a> {
    type Target = dyn Painter + 'a;

    fn deref(&self) -> &Self::Target {
        match self {
            CompactPainter::Reference(painter) => *painter,
            CompactPainter::Boxed(painter) => painter.deref(),
        }
    }
}

impl<'a> DerefMut for CompactPainter<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            CompactPainter::Reference(painter) => *painter,
            CompactPainter::Boxed(painter) => painter.deref_mut(),
        }
    }
}