use super::paint::Paint;



/// A brush used to paint a shape.
pub struct Brush {
    /// The paint used to draw the brush.
    pub paint: Paint,
}

impl Default for Brush {
    fn default() -> Self {
        Brush {
            paint: Paint::default(),
        }
    }
}

impl From<Paint> for Brush {
    fn from(paint: Paint) -> Self {
        Brush {
            paint,
        }
    }
}