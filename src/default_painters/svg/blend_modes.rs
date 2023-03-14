use strum::Display;

use crate::base::blending::BlendMode;

/// Svg blend modes.
///
/// <!--The SVG blend modes are a subset of the CSS blend modes.-->
///
/// See [MDN `mix-blend-mode`](https://developer.mozilla.org/en-US/docs/Web/CSS/mix-blend-mode)
/// for more info.
/// 
/// # Conversion
/// There is no direct conversion between [`SvgBlendMode`] and [`BlendMode`], but you can use the
/// [`From`] trait to convert from [`BlendMode`] to [`Option<SvgBlendMode>`] or [`SvgBlendMode`] to
/// [`Option<BlendMode>`]. Example:
/// ```
/// let svg_blend_mode: Option<SvgBlendMode> = blend_mode.into();
/// let blend_mode: Option<BlendMode> = svg_blend_mode.into();
/// ```
/// another example:
/// ```
/// let svg_blend_mode: Option<SvgBlendMode> = blend_mode.into();
/// let svg_blend_mode: SvgBlendMode = svg_blend_mode.unwrap_or(SvgBlendMode::Normal);
/// ```
/// 
/// [`BlendMode`]: crate::base::blending::BlendMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum SvgBlendMode {
    // Keyword values

    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,

    // Global values

    Inherit,
    Initial,
    Revert,
    RevertLayer,
    Unset,
}

impl SvgBlendMode {
    /// Convert from [`SvgBlendMode`] to a css blend mode string.
    /// 
    /// # Example
    /// ```
    /// let svg_blend_mode = SvgBlendMode::Normal;
    /// let css_blend_mode = svg_blend_mode.to_css_string();
    /// assert_eq!(css_blend_mode, "normal");
    /// ```
    pub fn to_css_string(&self) -> String {
        match self {
            SvgBlendMode::Normal => "normal".into(),
            SvgBlendMode::Multiply => "multiply".into(),
            SvgBlendMode::Screen => "screen".into(),
            SvgBlendMode::Overlay => "overlay".into(),
            SvgBlendMode::Darken => "darken".into(),
            SvgBlendMode::Lighten => "lighten".into(),
            SvgBlendMode::ColorDodge => "color-dodge".into(),
            SvgBlendMode::ColorBurn => "color-burn".into(),
            SvgBlendMode::HardLight => "hard-light".into(),
            SvgBlendMode::SoftLight => "soft-light".into(),
            SvgBlendMode::Difference => "difference".into(),
            SvgBlendMode::Exclusion => "exclusion".into(),
            SvgBlendMode::Hue => "hue".into(),
            SvgBlendMode::Saturation => "saturation".into(),
            SvgBlendMode::Color => "color".into(),
            SvgBlendMode::Luminosity => "luminosity".into(),

            SvgBlendMode::Inherit => "inherit".into(),
            SvgBlendMode::Initial => "initial".into(),
            SvgBlendMode::Revert => "revert".into(),
            SvgBlendMode::RevertLayer => "revert-layer".into(),
            SvgBlendMode::Unset => "unset".into(),
        }
    }

    /// parses a css blend mode string to a SvgBlendMode
    /// 
    /// # Example
    /// ```
    /// let css_blend_mode_string = "normal";
    /// let svg_blend_mode = SvgBlendMode::parse_css_blend_mode_string(css_blend_mode_string);
    /// assert_eq!(svg_blend_mode, Some(SvgBlendMode::Normal));
    /// ```
    pub fn parse_css_blend_mode_string(css: &str) -> Option<SvgBlendMode> {
        // TODO to check, written by copilot
        let css = css.to_lowercase();
        if css == "normal" {
            Some(SvgBlendMode::Normal)
        } else if css == "multiply" {
            Some(SvgBlendMode::Multiply)
        } else if css == "screen" {
            Some(SvgBlendMode::Screen)
        } else if css == "overlay" {
            Some(SvgBlendMode::Overlay)
        } else if css == "darken" {
            Some(SvgBlendMode::Darken)
        } else if css == "lighten" {
            Some(SvgBlendMode::Lighten)
        } else if css == "color-dodge" {
            Some(SvgBlendMode::ColorDodge)
        } else if css == "color-burn" {
            Some(SvgBlendMode::ColorBurn)
        } else if css == "hard-light" {
            Some(SvgBlendMode::HardLight)
        } else if css == "soft-light" {
            Some(SvgBlendMode::SoftLight)
        } else if css == "difference" {
            Some(SvgBlendMode::Difference)
        } else if css == "exclusion" {
            Some(SvgBlendMode::Exclusion)
        } else if css == "hue" {
            Some(SvgBlendMode::Hue)
        } else if css == "saturation" {
            Some(SvgBlendMode::Saturation)
        } else if css == "color" {
            Some(SvgBlendMode::Color)
        } else if css == "luminosity" {
            Some(SvgBlendMode::Luminosity)
        } else if css == "inherit" {
            Some(SvgBlendMode::Inherit)
        } else if css == "initial" {
            Some(SvgBlendMode::Initial)
        } else if css == "revert" {
            Some(SvgBlendMode::Revert)
        } else if css == "revert-layer" {
            Some(SvgBlendMode::RevertLayer)
        } else if css == "unset" {
            Some(SvgBlendMode::Unset)
        } else {
            None
        }
    }
}

/// example: `let svg_blend_mode: Option<SvgBlendMode> = blend_mode.into();`
impl From<BlendMode> for Option<SvgBlendMode> {
    fn from(value: BlendMode) -> Self {
        match value {
            BlendMode::Clear => None,
            BlendMode::Src => None,
            BlendMode::Dst => None,
            BlendMode::SrcOver => Some(SvgBlendMode::Normal),
            BlendMode::DstOver => None,
            BlendMode::SrcIn => None,
            BlendMode::DstIn => None,
            BlendMode::SrcOut => None,
            BlendMode::DstOut => None,
            BlendMode::SrcATop => None,
            BlendMode::DstATop => None,
            BlendMode::Xor => None,
            BlendMode::Plus => None,
            BlendMode::PlusClamped => None,
            BlendMode::Modulate => None,
            BlendMode::Screen => Some(SvgBlendMode::Screen),
            BlendMode::Overlay => Some(SvgBlendMode::Overlay),
            BlendMode::Darken => Some(SvgBlendMode::Darken),
            BlendMode::Lighten => Some(SvgBlendMode::Lighten),
            BlendMode::ColorDodge => Some(SvgBlendMode::ColorDodge),
            BlendMode::ColorBurn => Some(SvgBlendMode::ColorBurn),
            BlendMode::HardLight => Some(SvgBlendMode::HardLight),
            BlendMode::SoftLight => Some(SvgBlendMode::SoftLight),
            BlendMode::Difference => Some(SvgBlendMode::Difference),
            BlendMode::Exclusion => Some(SvgBlendMode::Exclusion),
            BlendMode::Multiply => Some(SvgBlendMode::Multiply),
            BlendMode::Hue => Some(SvgBlendMode::Hue),
            BlendMode::Saturation => Some(SvgBlendMode::Saturation),
            BlendMode::Color => Some(SvgBlendMode::Color),
            BlendMode::Luminosity => Some(SvgBlendMode::Luminosity),
        }
    }
}

/// example:
/// ```
/// let blend_mode: Option<BlendMode> = svg_blend_mode.into();
/// ```
impl From<SvgBlendMode> for Option<BlendMode> {
    fn from(value: SvgBlendMode) -> Self {
        match value {
            SvgBlendMode::Normal => Some(BlendMode::SrcOver),
            SvgBlendMode::Multiply => Some(BlendMode::Multiply),
            SvgBlendMode::Screen => Some(BlendMode::Screen),
            SvgBlendMode::Overlay => Some(BlendMode::Overlay),
            SvgBlendMode::Darken => Some(BlendMode::Darken),
            SvgBlendMode::Lighten => Some(BlendMode::Lighten),
            SvgBlendMode::ColorDodge => Some(BlendMode::ColorDodge),
            SvgBlendMode::ColorBurn => Some(BlendMode::ColorBurn),
            SvgBlendMode::HardLight => Some(BlendMode::HardLight),
            SvgBlendMode::SoftLight => Some(BlendMode::SoftLight),
            SvgBlendMode::Difference => Some(BlendMode::Difference),
            SvgBlendMode::Exclusion => Some(BlendMode::Exclusion),
            SvgBlendMode::Hue => Some(BlendMode::Hue),
            SvgBlendMode::Saturation => Some(BlendMode::Saturation),
            SvgBlendMode::Color => Some(BlendMode::Color),
            SvgBlendMode::Luminosity => Some(BlendMode::Luminosity),

            SvgBlendMode::Inherit => None,
            SvgBlendMode::Initial => None,
            SvgBlendMode::Revert => None,
            SvgBlendMode::RevertLayer => None,
            SvgBlendMode::Unset => None,
        }
    }
}