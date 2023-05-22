use std::{error::Error, fs::File, io::Write};

use repaint::{Canvas, base::{paint::{Color, Paint, Ink}, pen::Pen, defs::rect::F64Rect}, painter::methods::PaintStyle, nalgebra::Vector2};
use repaint_with_skia_safe::SkiaCanvas;
use skia_safe::{Surface, EncodedImageFormat};



fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut surface = Surface::new_raster_n32_premul((400, 300)).expect("no surface!");
    surface.canvas().clear(skia_safe::Color::WHITE);

    {
        let mut canvas = SkiaCanvas::new(&mut surface);
        let mut painter = canvas.painter().unwrap();

        painter.clear(Color::WHITE);

        let mut paint = Paint::default();
        paint.ink = Ink::Color(Color::new(0.0, 0.0, 0.0, 1.0));
        let mut pen = Pen::default();
        pen.paint = paint.clone();

        painter.rect((0.5, 0.5, 10.0, 10.0).into(), PaintStyle::Stroke(pen.clone()));

        paint.ink = Ink::Color(Color::new(1.0, 0.0, 0.0, 1.0));

        painter.rect((10.0, 10.0, 10.0, 10.0).into(), PaintStyle::Fill(paint));
    }

    let image = surface.image_snapshot();
    let data = image.encode_to_data(EncodedImageFormat::PNG).unwrap();
    let mut file = File::create("test.png").unwrap();
    let bytes = data.as_bytes();
    file.write_all(bytes).unwrap();

    Ok(())
}