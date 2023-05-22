
use repaint::{painter::methods::BlendModeMethods, base::blending::BlendMode};

use crate::SkiaPainter;

impl<'canvas, 'surface> BlendModeMethods for SkiaPainter<'canvas, 'surface> {
    fn is_blend_mode_valid(&self, _mode: BlendMode) -> bool {
        todo!()
    }
}