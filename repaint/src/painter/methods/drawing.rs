use crate::base::{paint::Paint, pen::Pen, defs::colors::ColorType};


// TODO move
#[derive(Clone, Debug, PartialEq)]
pub enum PaintStyle<Color> {
    Stroke(Pen<Color>),
    Fill(Paint<Color>),
    StrokeAndFill(Pen<Color>),
}

impl<Color: ColorType> PaintStyle<Color> {
    pub fn paint(&self) -> &Paint<Color> {
        match self {
            PaintStyle::Stroke(pen) | Self::StrokeAndFill(pen) => &pen.paint,
            PaintStyle::Fill(paint) => paint,
        }
    }

    pub fn paint_mut(&mut self) -> &mut Paint<Color> {
        match self {
            PaintStyle::Stroke(pen) | Self::StrokeAndFill(pen) => &mut pen.paint,
            PaintStyle::Fill(paint) => paint,
        }
    }
}

// TODO instead of f32 or f64 for points, we could make an enum to discriminate between integer and floating point coordinates...