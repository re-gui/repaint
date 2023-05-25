use crate::base::{paint::Paint, pen::Pen};


// TODO move
#[derive(Clone, Debug, PartialEq)]
pub enum PaintStyle {
    Stroke(Pen),
    Fill(Paint),
    StrokeAndFill(Pen),
}

impl PaintStyle {
    pub fn paint(&self) -> &Paint {
        match self {
            PaintStyle::Stroke(pen) | Self::StrokeAndFill(pen) => &pen.paint,
            PaintStyle::Fill(paint) => paint,
        }
    }

    pub fn paint_mut(&mut self) -> &mut Paint {
        match self {
            PaintStyle::Stroke(pen) | Self::StrokeAndFill(pen) => &mut pen.paint,
            PaintStyle::Fill(paint) => paint,
        }
    }
}

// TODO instead of f32 or f64 for points, we could make an enum to discriminate between integer and floating point coordinates...