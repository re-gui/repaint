
use repaint::{painter::methods::BlendModeMethods, base::blending::BlendMode};

use crate::SkiaPainter;

impl<'canvas, 'surface, 'context, 'context_lifecycle> BlendModeMethods for SkiaPainter<'canvas, 'surface, 'context, 'context_lifecycle> {
    fn is_blend_mode_valid(&self, _mode: BlendMode) -> bool {
        todo!()
    }
}