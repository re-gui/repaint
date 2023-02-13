/*!

Provides the [`Painter`] trait.

*/

use crate::base::blending::BlendMode;
use crate::canvas::Canvas;

/// A painter that can be used to draw on a [`Canvas`].
/// 
/// In order to organize the drawing process, the painter
/// trait is split into conceptually separate parts that provide
/// different functionality:
///  - [`WithBlendMode`] contains the methods related to blend modes
pub trait Painter: WithBlendMode {
}

pub trait WithBlendMode {

    /// Returns whether this painter supports blend modes.
    fn has_blend_mode(&self) -> bool {
        false
    }
    fn can_set_blend_mode(&self, _mode: BlendMode) -> bool {
        self.has_blend_mode()
    }
    fn blend_mode(&self) -> BlendMode {
        BlendMode::SrcOver
    }
    fn set_blend_mode(&mut self, _mode: BlendMode) {
        // ...
    }
    fn enumerate_valid_blend_modes<'s>(&'s self) -> Box<dyn Iterator<Item = BlendMode> + 's> {
        Box::new(BlendMode::enumerate_all().filter(move |mode| self.can_set_blend_mode(*mode)))
    }
}