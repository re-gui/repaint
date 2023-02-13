
use strum::{EnumIter, IntoEnumIterator};


/// A blend mode is a function that combines a *source* color with a *destination* color to produce a new color.
/// 
/// The **source** is the color that is being drawn on the canvas, while the **destination** is the color that is already present on the canvas (same as **backdrop** in this context).
/// 
/// It is advised to read [this article](https://www.w3.org/TR/compositing-1/) to understand the concept of blending and the different blend modes.
/// 
/// ## Blending
/// 
/// The term "*blending*" is used to describe the process of combining the source and destination colors. We want to maintain consistency with the [w3 definition](https://www.w3.org/TR/compositing-1/#blending) which states that:
/// > [**Blending**](https://www.w3.org/TR/compositing-1/#blending) is the aspect of [compositing](https://www.w3.org/TR/compositing-1/#whatiscompositing) that calculates the mixing of colors where the source element and [backdrop](https://www.w3.org/TR/compositing-1/#backdropCalc) overlap.
/// 
/// As such, the process of blending only applies to the overlapping area of the source and destination, the rest of the destination is left untouched.
/// 
/// ### General formula for Compositing and Blending
/// 
/// Some of the blend modes can be expressed as a general formula (algorithm) for compositing and blending, which is, as defined by [w3](https://www.w3.org/TR/compositing-1/#generalformula):
///  1. apply *blend in place*:
///    <tex-math>
///      s_C \leftarrow (1 - b_\alpha) \cdot s_C + b_\alpha \cdot B(b_C, s_C)
///    </tex-math>
///  2. apply *composite* (maybe alpha compositing???):
///    <tex-math>
///      r_C \leftarrow s_\alpha \cdot F_a \cdot s_C + b_\alpha \cdot F_b \cdot b_C
///    </tex-math>
/// 
///  where:
///   - <i-math>B</i-math> is the *mixing function* (aka *blend function*)
///   - <i-math>F_a</i-math> and <i-math>F_b</i-math> are defined by the Peter Duff operator in use
/// 
/// Also, the [*simple alpha compositing*](https://www.w3.org/TR/compositing-1/#simplealphacompositing) formula is:
///  - <i-math>r_c = s_C \\, s_\alpha + b_C \\, b_\alpha \\, (1 - s_\alpha) = s_c + b_c \\, (1 - s_\alpha)</i-math>
///  - <i-math>r_\alpha = s_\alpha + b_\alpha \\, (1 - s_\alpha)</i-math>
///  - <i-math>r_C = r_c / r_\alpha</i-math>
/// 
/// ### Standard Blending Formula
/// 
/// We define a standard blending formula (see [this formula](https://www.w3.org/TR/compositing-1/#blending)) that is used by some of the blend modes using the [general formula](#general-formula-for-compositing-and-blending) above:
///  1. use the simple alpha compositing formula to calculate the result color and alpha:
///    <tex-math>
///    \begin{aligned}
///      r_c &= s_c + b_c \\, (1 - s_\alpha) = s_C \\, s_\alpha + b_C \\, b_\alpha \\, (1 - s_\alpha)\\\\\
///      r_\alpha &= s_\alpha + b_\alpha \\, (1 - s_\alpha)
///    \end{aligned}
///    </tex-math>
///  2. substitute the result of the *blend in place* step in the *composite* step:
///    <tex-math>
///    \begin{aligned}
///     r_c &= s_\alpha \\, ((1 - b_\alpha) s_C + b_\alpha \\, B(b_C, s_C)) + b_\alpha \\, b_C \\, (1 - s_\alpha) \\\\
///         &= s_\alpha \\, (1 - b_\alpha) s_C + s_\alpha \\, b_\alpha \\, B(b_C, s_C) + b_\alpha \\, b_C \\, (1 - s_\alpha)
///    \end{aligned}
///    </tex-math>
/// 
/// #### Example
/// 
/// In the [normal](`BlendMode::SrcOver`) blending mode, the *mixing function* is:
/// <tex-math>
///   B(b_C, s_C) = s_C
/// </tex-math>
/// this means that the resulting color is:
/// <tex-math>
/// \begin{aligned}
/// r_c &= s_\alpha \\, (1 - b_\alpha) s_C + s_\alpha \\, b_\alpha \\, B(b_C, s_C) + b_\alpha \\, b_C \\, (1 - s_\alpha) \\\\
///     &= s_\alpha \\, (1 - b_\alpha) s_C + s_\alpha \\, b_\alpha \\, s_C + b_\alpha \\, b_C \\, (1 - s_\alpha) \\\\
///     %&= s_\alpha s_C - s_\alpha b_\alpha s_C + s_\alpha b_\alpha s_C + b_\alpha b_C - s_\alpha b_\alpha b_C \\\\
///     %&= s_\alpha s_C + b_\alpha b_C - s_\alpha b_\alpha b_C \\\\
///     %&= s_\alpha s_C + (b_\alpha - s_\alpha b_\alpha) b_C \\\\
///     &= s_c + (1 - s_\alpha) b_c
/// \end{aligned}
/// </tex-math>
/// that is the [`BlendMode::SrcOver`] blend mode.
/// 
/// ## Notes
/// 
/// To keep [Skia](https://skia.org/docs/user/api/skblendmode_overview/) and [w3](https://skia.org/docs/user/api/skblendmode_overview/) nomenclature:
///  - `s` is the **source** color and alpha (all components)
///  - `d` (or `b`) is the **destination** (or **backdrop**) color and alpha (all components)
///  - `r` is the **result** color and alpha (all components)
///  - `*_a` is the **alpha** component of the color
///  - `*_C` is the **color** channel component of the color
///  - `*_c` is the **premultiplied color** channel component of the color, i.e. `C * a`, `C = c / a`
/// 
/// All components are real numbers in the range <i-math>[0, 1]</i-math>.
/// 
/// ## References
/// - <https://www.w3.org/TR/compositing-1/#blending>
/// - <https://skia.org/docs/user/api/skblendmode_overview/>
/// - <https://developer.mozilla.org/en-US/docs/Web/CSS/mix-blend-mode>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation>
/// - <https://graphics.pixar.com/library/Compositing/paper.pdf>
/// - <https://en.wikipedia.org/wiki/Blend_modes>
/// 
/// TODO LATEX EQUATIONS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum BlendMode {
    /// `r = 0`
    /// 
    /// The source is discarded where it overlaps the destination, effectively "erasing" the destination.
    Clear,

    /// `r = s`, aka "copy", "normal", "replace", "source"
    /// 
    /// The source is drawn over the destination, replacing the destination.
    Src,


    /// `r = d`
    /// 
    /// The destination is the result color. In most cases this means that the source is not drawn at all.
    Dst,

    /// AKA **normal** or "simple alpha composing"
    /// 
    /// The output of this mode satisfies the following equations:
    /// <tex-math>
    /// \begin{split}
    ///   r_c &= s_c + (1 - s_a) b_c \\\\
    ///   r_a &= s_a + b_a \\, (1 - s_a)
    /// \end{split}
    /// </tex-math>
    /// 
    /// This equation satisfies the [standard blending formula](#standard-blending-formula) with
    /// <i-math>B(b_C, s_C) = s_C</i-math> i.e. the color are not mixed and the source behaves like if
    /// it is "on top of the destination".
    /// 
    /// ### Derivation from the [standard blending formula](#standard-blending-formula)
    /// Substituting <i-math>B(b_C, s_C) = s_C</i-math> in the [standard blending formula](#standard-blending-formula):
    /// <tex-math>
    /// \begin{aligned}
    /// r_c &= s_\alpha \\, (1 - b_\alpha) s_C + s_\alpha \\, b_\alpha \\, B(b_C, s_C) + b_\alpha \\, b_C \\, (1 - s_\alpha) \\\\
    ///     &= s_\alpha \\, (1 - b_\alpha) s_C + s_\alpha \\, b_\alpha \\, s_C + b_\alpha \\, b_C \\, (1 - s_\alpha) \\\\
    ///     %&= s_\alpha s_C - s_\alpha b_\alpha s_C + s_\alpha b_\alpha s_C + b_\alpha b_C - s_\alpha b_\alpha b_C \\\\
    ///     %&= s_\alpha s_C + b_\alpha b_C - s_\alpha b_\alpha b_C \\\\
    ///     %&= s_\alpha s_C + (b_\alpha - s_\alpha b_\alpha) b_C \\\\
    ///     &= s_c + (1 - s_\alpha) b_c
    /// \end{aligned}
    /// </tex-math>
    /// 
    /// ## Notes
    ///  - This is usually the default blend mode.
    /// 
    /// ![example](https://www.w3.org/TR/compositing-1/examples/normal.png)
    SrcOver,

    /// Opposite of [`SrcOver`](`BlendMode::SrcOver`)
    /// 
    /// The output of this mode satisfies the following equations:
    /// <tex-math>
    /// \begin{split}
    ///   r_c &= b_c + (1 - b_a) s_c \\\\
    ///   r_a &= b_a + s_a \\, (1 - b_a)
    /// \end{split}
    /// </tex-math>
    /// 
    /// This is the same as [`SrcOver`](`BlendMode::SrcOver`) with the source and destination swapped.
    DstOver,

    /// `r = s * d_a`
    /// 
    /// The output of this mode satisfies the following equations:
    /// <tex-math>
    /// \begin{split}
    ///   r_c &= s_c \\, b_\alpha \\\\
    ///   r_a &= s_a \\, b_\alpha
    /// \end{split}
    /// </tex-math>
    SrcIn,

    /// `r = d * s_a`
    /// 
    /// The output of this mode satisfies the following equations:
    /// <tex-math>
    /// \begin{split}
    ///   r_c &= b_c \\, s_\alpha \\\\
    ///   r_a &= b_a \\, s_\alpha
    /// \end{split}
    /// </tex-math>
    /// 
    /// This is the same as [`SrcIn`](`BlendMode::SrcIn`) with the source and destination swapped.
    DstIn,

    /// `r = s * (1 - d_a)`
    /// 
    /// Behaves like "draw where there is no destination" (i.e. the source is drawn only where the destination is transparent).
    /// 
    /// TODO ...
    SrcOut,

    /// `r = d * (1 - s_a)`
    /// 
    /// Behaves like "erase where there is source" (i.e. the destination is erased where the source is not transparent).
    /// It might seem the same as [`Clear`](`BlendMode::Clear`), but the difference is that here the process is weighted by the alpha of the source.
    /// 
    /// TODO ...
    DstOut,

    /// `r = s * d_a + d * (1 - s_a)`
    /// 
    /// TODO ...
    SrcATop,

    /// `r = d * s_a + s * (1 - d_a)`
    /// 
    /// TODO ...
    DstATop,

    /// `r = s * (1 - d_a) + d * (1 - s_a)`
    /// 
    /// TODO ...
    Xor,

    /// `r = s + d`
    /// 
    /// TODO ...
    Plus,

    /// `r = min(s + d, 1)`
    /// 
    /// TODO ...
    PlusClamped,

    /// `r = s * d`
    /// 
    /// TODO ...
    Modulate,

    /// `r = s + d - s * d`
    /// 
    /// TODO ...
    Screen,

    /// [`Multiply`](`BlendMode::Multiply`) or [`Screen`](`BlendMode::Screen`) depending on the destination (color?!?!).
    /// TODO ?
    /// 
    /// TODO ...
    Overlay,

    /// `r_c = s + d - max(s * d_a, d * s_a)`, `r_a = SrcOver`
    /// 
    /// TODO ...
    Darken,

    /// `r_c = s + d - min(s * d_a, d * s_a)`, `r_a = SrcOver`
    /// 
    /// TODO ...
    Lighten,

    /// "Brighten the destination to reflect the source."
    /// TODO ?
    /// TODO ...
    ColorDodge,

    /// "Darken the destination to reflect the source."
    /// TODO ?
    /// TODO ...
    ColorBurn,

    /// "multiply or screen, depending on source"
    /// TODO ?
    /// TODO ...
    HardLight,

    /// "lighten or darken, depending on source "
    /// TODO ?
    /// TODO ...
    SoftLight,

    /// `r_c = s + d - 2 * min(s * d_a, d * s_a)`, `r_a = SrcOver`
    /// 
    /// TODO ...
    Difference,

    /// `r_c = s + d - 2 * (s * d)`, `r_a = SrcOver`
    /// 
    /// TODO ...
    Exclusion,

    /// `r = s * (1 - d_a) + d * (1 - s_a) + s * d`
    /// 
    /// TODO ...
    Multiply,

    /// "hue of source with saturation and luminosity of destination"
    /// TODO ?
    /// 
    /// TODO ...
    Hue,

    /// "saturation of source with hue and luminosity of destination"
    /// TODO ?
    /// 
    /// TODO ...
    Saturation,

    /// "hue and saturation of source with luminosity of destination"
    /// TODO ?
    /// 
    /// TODO ...
    Color,

    /// "luminosity of source with hue and saturation of destination"
    /// TODO ?
    /// 
    /// TODO ...
    Luminosity,
}

impl BlendMode {
    pub fn enumerate_all() -> impl Iterator<Item = BlendMode> {
        BlendMode::iter()
    }
}

/// Implementation of the blend modes.
pub mod modes_impl {
    //use super::*;
    use crate::base::defs::colors::default_color_types::RgbaFColor;
    use crate::base::defs::colors::default_colors;

    /// Standard Blending Formula
    #[inline]
    pub fn standard_blending_formula<Mix: Fn((f32, f32, f32), (f32, f32, f32)) -> (f32, f32, f32)>(
    s_col: (f32, f32, f32), s_a: f32,
    b_col: (f32, f32, f32), b_a: f32,
    mixing: Mix) -> (f32, f32, f32, f32) {
        let mixed = mixing(b_col, s_col);
        let (r_c, g_c, b_c) = (
            s_a * (1.0 - b_a) * s_col.0 + s_a * b_a * mixed.0 + (1.0 - s_a) * b_a * b_col.0,
            s_a * (1.0 - b_a) * s_col.1 + s_a * b_a * mixed.1 + (1.0 - s_a) * b_a * b_col.1,
            s_a * (1.0 - b_a) * s_col.2 + s_a * b_a * mixed.2 + (1.0 - s_a) * b_a * b_col.2
        );
        let r_a = s_a + b_a * (1.0 - s_a);
        let (r_col, g_col, b_col) = if r_a == 0.0 {
            (0.0, 0.0, 0.0)
        } else {
            (r_c / r_a, g_c / r_a, b_c / r_a)
        };
        (r_col, g_col, b_col, r_a)
    }

    /// Implementation of [`BlendMode::Clear`](`super::BlendMode::Clear`).
    #[inline]
    pub fn clear(_src: &RgbaFColor, dst: &mut RgbaFColor) {
        *dst = default_colors::TRANSPARENT;
    }

    /// Implementation of [`BlendMode::Src`](`super::BlendMode::Src`).
    #[inline]
    pub fn src(src: &RgbaFColor, dst: &mut RgbaFColor) {
        *dst = *src;
    }

    /// Implementation of [`BlendMode::Dst`](`super::BlendMode::Dst`).
    #[inline]
    pub fn dst(_src: &RgbaFColor, _dst: &mut RgbaFColor) {
        // do nothing
    }

    /// Implementation of [`BlendMode::SrcOver`](`super::BlendMode::SrcOver`).
    #[inline]
    pub fn src_over(src: &RgbaFColor, dst: &mut RgbaFColor) {

        let mixing = |_b_col: (f32, f32, f32), s_col: (f32, f32, f32)| {
            s_col
        };

        let s_col: (f32, f32, f32) = (src.r(), src.g(), src.b());
        let s_a: f32 = src.a();
        let b_col: (f32, f32, f32) = (dst.r(), dst.g(), dst.b());
        let b_a: f32 = dst.a();
        let (r_col, g_col, b_col, r_a) = standard_blending_formula(s_col, s_a, b_col, b_a, mixing);

        *dst.r_mut() = r_col;
        *dst.g_mut() = g_col;
        *dst.b_mut() = b_col;
        *dst.a_mut() = r_a;
    }

    // TODO ...
}