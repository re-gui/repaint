/*!

Provides the [`Painter`] trait.

*/

use crate::base::blending::BlendMode;
//use crate::canvas::Canvas;
use crate::base::defs::antialiasing::AntialiasMode;

/// A painter that can be used to draw on a [`Canvas`](`crate::canvas::Canvas`).
/// 
/// In order to organize the drawing process, the painter
/// trait is split into conceptually separate parts that provide
/// different functionality:
///  - [`WithBlendMode`] contains the methods related to blend modes
pub trait Painter: WithBlendModeMethods + WithAntialiasMethods {
}

/// Methods related to blend modes.
pub trait WithBlendModeMethods {

    /// Returns whether this painter supports blend modes.
    /// 
    /// If this method returns `false`, all other methods related to blend modes
    /// will have no effect.
    /// 
    /// Default implementation returns `false`.
    fn has_blend_mode(&self) -> bool {
        false
    }

    /// Returns whether this painter can set the given blend mode.
    fn can_set_blend_mode(&self, _mode: BlendMode) -> bool {
        self.has_blend_mode()
    }

    /// Returns the current blend mode.
    fn blend_mode(&self) -> BlendMode {
        BlendMode::SrcOver
    }

    /// Sets the blend mode. If the given blend mode is not supported by this painter,
    /// the effective blend mode will be undefined, probably the default blend mode.
    /// 
    /// When an invalid blend mode is set, the painter will not panic, and it will behave like
    /// any other blend mode respect to the push/pop? state.
    fn set_blend_mode(&mut self, _mode: BlendMode) {
        // ...
    }

    /// Returns an iterator over all valid blend modes.
    fn enumerate_valid_blend_modes<'s>(&'s self) -> Box<dyn Iterator<Item = BlendMode> + 's> {
        Box::new(BlendMode::enumerate_all().filter(move |mode| self.can_set_blend_mode(*mode)))
    }
}

/// Methods related to antialiasing.
pub trait WithAntialiasMethods {
    /// Returns whether this painter supports antialiasing.
    fn has_antialias(&self) -> bool {
        false
    }

    /// Returns whether this painter can set the given antialias mode.
    fn can_set_antialias_mode(&self, _mode: AntialiasMode) -> bool {
        self.has_antialias()
    }

    /// Returns whether this painter can change the antialias mode at this time.
    /// Some painters may not be able to change the antialias mode while drawing but only
    /// before or after. ???
    fn can_change_antialias_mode_now(&self) -> bool {
        false
    }

    /// Returns the current antialias mode.
    fn antialias_mode(&self) -> AntialiasMode {
        AntialiasMode::None
    }

    /// Sets the antialias mode. If the given antialias mode is not supported by this painter,
    /// the effective antialias mode will be undefined.
    fn set_antialias_mode(&mut self, _mode: AntialiasMode) {
        // ...
    }

    /// Tells whether antialiasing is enabled.
    fn antialiased(&self) -> bool {
        self.antialias_mode() != AntialiasMode::None
    }
}

