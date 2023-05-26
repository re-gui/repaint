use super::{paint::Paint, defs::colors::ColorType};


#[derive(Clone, Debug, PartialEq)]
pub struct Pen<Color> {
    // The paint used to draw the stroke
    pub paint: Paint<Color>,

    // The path effect to apply to the stroke
    pub path_effect: PathEffect,

    // The stroke width
    pub stroke_width: StrokeWidth,

    /// A cosmetic stroke is a stroke that as some properties that are not affected by the transformation:
    ///  - the [width](`Pen::stroke_width`)
    ///  - the dash pattern
    pub cosmetic_stroke: bool,

    pub cap: PenCap,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PenCap {
    Butt,
    Round,
    Square,
}

impl<Color: ColorType> Default for Pen<Color> {
    fn default() -> Self {
        Pen {
            paint: Paint::default(),
            path_effect: PathEffect::None,
            stroke_width: StrokeWidth::Hairline,
            cosmetic_stroke: false,
            cap: PenCap::Butt,
        }
    }
}

impl<Color: ColorType> From<Paint<Color>> for Pen<Color> {
    fn from(paint: Paint<Color>) -> Self {
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
    Hairline,
    Normal(f32),
    Cosmetic(f32),
}

impl From<f32> for StrokeWidth {
    fn from(width: f32) -> Self {
        StrokeWidth::Normal(width)
    }
}

impl From<f64> for StrokeWidth {
    fn from(width: f64) -> Self {
        StrokeWidth::Normal(width as f32)
    }
}

impl From<i32> for StrokeWidth {
    fn from(width: i32) -> Self {
        StrokeWidth::Normal(width as f32)
    }
}