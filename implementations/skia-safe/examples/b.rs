use std::{error::Error, fs::File, io::Write, cell::RefCell};

use repaint::{Canvas, base::{paint::{Paint, Ink}, pen::{Pen, PenCap}, shapes::path::{PathBuilder, PathCommand}}, painter::{methods::PaintStyle, Painter}, nalgebra::Vector2, Color};
use repaint_with_skia_safe::{SkiaCanvas, make_skia_context};
use skia_safe::{Surface, EncodedImageFormat};

fn draw<'context>(painter: &mut impl Painter<'context, NativeColor = Color>) {

    painter.clear(Color::WHITE.into());

    let mut paint = Paint::default();

    paint.ink = Color::new(0.0, 0.0, 0.0, 1.0).into();
    paint.anti_alias = true;
    let mut pen = Pen::default();
    pen.paint = paint.clone();
    pen.cap = PenCap::Round;

    let mut pen = pen;
    let style = PaintStyle::Stroke(pen.clone());

    painter.rect((0.5, 0.5, 10.0, 10.0).into(), style);

    paint.ink = Ink::Color(Color::new(1.0, 0.0, 0.0, 1.0).into());

    painter.rect((10.0, 10.0, 10.0, 10.0).into(), PaintStyle::Fill(paint));

    let mut builder = PathBuilder::new();
    builder.push(PathCommand::MoveTo(Vector2::new(20.0, 20.0)));
    builder.push(PathCommand::LineTo(Vector2::new(40.0, 42.0)));

    pen.stroke_width = 5.0.into();

    let res = painter.make_path(&mut builder.commands.iter().cloned()).unwrap();
    //let res = ctx.make_path(&mut builder.commands.iter().cloned()).unwrap();
    painter.path(&res, PaintStyle::Stroke(pen.clone()));

    painter.line(Vector2::new(0.0, 100.0), Vector2::new(100.0, 0.0), pen);
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut surface = Surface::new_raster_n32_premul((100, 100)).expect("no surface!");
    surface.canvas().clear(skia_safe::Color::WHITE);

    {
        let ctx = RefCell::new(make_skia_context());

        let mut canvas = SkiaCanvas::new(&mut surface, &ctx);
        let mut painter = canvas.painter().unwrap();

        draw(&mut painter);
    }

    let image = surface.image_snapshot();
    let data = image.encode_to_data(EncodedImageFormat::PNG).unwrap();
    let mut file = File::create("test.png").unwrap();
    let bytes = data.as_bytes();
    file.write_all(bytes).unwrap();

    Ok(())
}