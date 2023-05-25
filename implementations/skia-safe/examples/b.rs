use std::{error::Error, fs::File, io::Write, cell::RefCell, borrow::{Borrow, BorrowMut}};

use repaint::{Canvas, base::{paint::{Color, Paint, Ink}, pen::{Pen, PenCap}, defs::rect::F64Rect, shapes::path::{PathBuilder, PathCommand}}, painter::{methods::PaintStyle, Context}, nalgebra::Vector2, Painter};
use repaint_with_skia_safe::{SkiaCanvas, make_skia_context};
use skia_safe::{Surface, EncodedImageFormat};



fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut surface = Surface::new_raster_n32_premul((100, 100)).expect("no surface!");
    surface.canvas().clear(skia_safe::Color::WHITE);

    {
        let mut ctx = RefCell::new(make_skia_context());

        let mut canvas = SkiaCanvas::new(&mut surface, &ctx);
        let mut painter = canvas.painter().unwrap();

        painter.clear(Color::WHITE);

        let mut paint = Paint::default();

        paint.ink = Ink::Color(Color::new(0.0, 0.0, 0.0, 1.0));
        paint.anti_alias = true;
        let mut pen = Pen::default();
        pen.paint = paint.clone();
        pen.stroke_width = 10.0.into();
        pen.cap = PenCap::Round;

        painter.rect((0.5, 0.5, 10.0, 10.0).into(), PaintStyle::Stroke(pen.clone()));

        paint.ink = Ink::Color(Color::new(1.0, 0.0, 0.0, 1.0));

        painter.rect((10.0, 10.0, 10.0, 10.0).into(), PaintStyle::Fill(paint));

        let mut builder = PathBuilder::new();
        builder.push(PathCommand::MoveTo(Vector2::new(20.0, 20.0)));
        builder.push(PathCommand::LineTo(Vector2::new(40.0, 42.0)));

        let res = painter.make_path(&mut builder.commands.iter().cloned()).unwrap();
        //let res = ctx.make_path(&mut builder.commands.iter().cloned()).unwrap();
        painter.path(&res, PaintStyle::Stroke(pen));
    }

    let image = surface.image_snapshot();
    let data = image.encode_to_data(EncodedImageFormat::PNG).unwrap();
    let mut file = File::create("test.png").unwrap();
    let bytes = data.as_bytes();
    file.write_all(bytes).unwrap();

    Ok(())
}