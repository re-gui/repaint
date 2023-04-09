use std::rc::Rc;

use crate::base::{shapes::polyline::BrokenPolylineCommand, blending::BlendMode, defs::{colors::default_color_types::RgbaColor, linalg::Vec2f}};

pub struct LineISpan {
    pub start_index: usize,
    pub end_index: usize,
}

pub struct ImageBuffer<'a> {
    pub width: usize,
    pub height: usize,
    pub data: &'a mut [u8],
    pub stride: usize,
}

pub struct GlobalRasterizer<'a> {
    image_buffer: ImageBuffer<'a>,
}

impl<'a> GlobalRasterizer<'a> {
    pub fn new(image_buffer: ImageBuffer<'a>) -> Self {
        Self { image_buffer }
    }

    pub fn add_contour(
        &mut self,
        contour_commands: &mut impl Iterator<Item = BrokenPolylineCommand>,
        paint: Paint,
        blend_mode: BlendMode,
    ) {

    }
}

//#[derive(Clone, Debug, PartialEq)]
pub enum Paint {
    SolidColor(RgbaColor),
    YFunction(Rc<dyn Fn(f64) -> RgbaColor>),
    XFunction(Rc<dyn Fn(f64) -> RgbaColor>),
    XYFunction(Rc<dyn Fn(Vec2f) -> RgbaColor>),
}

fn ciao<T>(a: Rc<T>, b: Rc<T>) {
    a.
}