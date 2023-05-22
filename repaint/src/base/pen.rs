use super::paint::Paint;


#[derive(Clone, Debug, PartialEq)]
pub struct Pen {
    // The paint used to draw the stroke
    pub paint: Paint,

    // The path effect to apply to the stroke
    pub path_effect: PathEffect,

    // The stroke width
    pub stroke_width: StrokeWidth,

    /// A cosmetic stroke is a stroke that as some properties that are not affected by the transformation:
    ///  - the [width](`Pen::stroke_width`)
    ///  - the dash pattern
    pub cosmetic_stroke: bool,
}

impl Default for Pen {
    fn default() -> Self {
        Pen {
            paint: Paint::default(),
            path_effect: PathEffect::None,
            stroke_width: StrokeWidth::Constant(1.0),
            cosmetic_stroke: false,
        }
    }
}

impl From<Paint> for Pen {
    fn from(paint: Paint) -> Self {
        Pen {
            paint,
            ..Pen::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PathEffect {
    None
}

#[derive(Clone, Debug, PartialEq)]
pub enum StrokeWidth {
    Constant(f32),
}