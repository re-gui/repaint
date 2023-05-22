use crate::base::blending::BlendMode;

/// Methods related to blend modes.
pub trait BlendModeMethods {

    /// Returns whether this painter can set the given blend mode.
    fn is_blend_mode_valid(&self, _mode: BlendMode) -> bool;

    /// Returns an iterator over all valid blend modes.
    fn enumerate_valid_blend_modes<'s>(&'s self) -> Box<dyn Iterator<Item = BlendMode> + 's> {
        Box::new(BlendMode::enumerate_all().filter(move |mode| self.is_blend_mode_valid(*mode)))
    }
}