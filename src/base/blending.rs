

/// A blend mode is a function that combines a source color with a destination color to produce a new color.
/// 
/// ## References
/// - <https://skia.org/docs/user/api/skblendmode_overview/>
/// - <https://developer.mozilla.org/en-US/docs/Web/CSS/mix-blend-mode>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation>
/// - <https://graphics.pixar.com/library/Compositing/paper.pdf>
/// - <https://en.wikipedia.org/wiki/Blend_modes>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendMode {
    // TODO ...
}

impl BlendMode {
    pub fn enumerate() -> impl Iterator<Item = BlendMode> {
        vec![].into_iter()
    }
}