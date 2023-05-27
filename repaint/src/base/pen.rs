use super::paint::Paint;


/// A pen defines how a stroke is drawn.
#[derive(Clone, Debug, PartialEq)]
pub struct Pen<Color> {
    /// The paint used to draw the stroke
    pub paint: Paint<Color>,

    /// The path effect to apply to the stroke
    pub path_effect: PathEffect,

    /// The stroke width
    pub stroke_width: StrokeWidth,

    /// The cap to use for the stroke
    pub cap: PenCap,
}

/// Allows to concisely create a pen from a color.
///
/// The paint will be a solid color paint, the other fields will be set to their default value.
///
/// # Example
/// ```
/// use repaint::Color;
/// use repaint::base::Pen;
///
/// let pen: Pen<Color> = Color::RED.into();
/// ```
impl<Color> From<Color> for Pen<Color> {
    fn from(color: Color) -> Self {
        Pen {
            paint: color.into(),
            ..Pen::default()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PenCap {
    Butt,
    Round,
    Square,
}

impl<Color> Default for Pen<Color> {
    fn default() -> Self {
        Pen {
            paint: Paint::default(),
            path_effect: PathEffect::None,
            stroke_width: StrokeWidth::Hairline,
            cap: PenCap::Butt,
        }
    }
}

impl<Color> From<Paint<Color>> for Pen<Color> {
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