
use crate::base::defs::colors::default_color_types::RgbaFColor;

use super::{blending::BlendMode};

#[derive(Clone, Debug, PartialEq)]
pub struct Paint<Color> {
    pub ink: Ink<Color>,
    // ??? pub antialias_mode: AntialiasMode,
    pub blend_mode: BlendMode,
    pub anti_alias: bool,
    // TODO filters, etc...
    // see https://skia.org/docs/user/api/skpaint_overview/
}

impl<Color> Default for Paint<Color> {
    fn default() -> Self {
        Paint {
            ink: Ink::None,
            blend_mode: BlendMode::default(),
            anti_alias: true,
        }
    }
}

impl<Color> From<Ink<Color>> for Paint<Color> {
    fn from(ink: Ink<Color>) -> Self {
        Paint {
            ink,
            ..Paint::default()
        }
    }
}

impl<Color> From<Color> for Paint<Color> {
    fn from(color: Color) -> Self {
        Paint {
            ink: Ink::Color(color),
            ..Paint::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ink<Color> {
    /// No ink, nothing is painted.
    None,

    /// The ink is a solid color.
    Color(Color),

    /// The ink is a shader.
    Shader, // TODO

    // TODO gradient, image, 1d image, etc...
}

impl<Color> From<Color> for Ink<Color> {
    fn from(color: Color) -> Self {
        Ink::Color(color)
    }
}


/// The shader used to paint the ink.
#[derive(Clone, Debug)]
pub enum InkShader {
    /// The shader is invalid, its behavior is undefined.
    Invalid,

    /// The shader has custom code
    Custom, // TODO

    /// The shader is a composition of two other shaders using a blend mode.
    Compose {
        blend_mode: BlendMode,
        shader_source: Box<InkShader>,
        shader_backdrop: Box<InkShader>,
    },

    /// The shader is a composition of two other shaders using a blend mode.
    UniformColor(RgbaFColor),

    /// The shader is a linear gradient.
    LinearGradient{
        start: (f32, f32),
        end: (f32, f32),
        start_color: RgbaFColor,
        end_color: RgbaFColor,
    },

    /// The shader is a radial gradient.
    RadialGradient{
        center: (f32, f32),
        radius: f32,
        center_color: RgbaFColor,
        edge_color: RgbaFColor,
    },

    /// The shader is a sweep gradient.
    SweepGradient{
        center: (f32, f32),
        start_angle: f32,
        end_angle: f32,
        center_color: RgbaFColor,
        edge_color: RgbaFColor,
    },

    /// The shader is a conical gradient.
    TwoPointConicalGradient{
        start: (f32, f32),
        start_radius: f32,
        end: (f32, f32),
        end_radius: f32,
        start_color: RgbaFColor,
        end_color: RgbaFColor,
    },

    // TODO ...
    // see https://skia.org/docs/user/api/skpaint_overview/
}