use std::{error::Error, fs::File, io::Write, cell::RefCell};

use repaint::{Canvas, base::{paint::{Paint, Ink}, pen::{Pen, PenCap}, shapes::{path::{PathBuilder, PathCommand}, BasicShape}}, nalgebra::Vector2, Color, methods::PaintStyle, ClipOperation, Painter, FontStyle, Typeface, FontManager};
use repaint_with_skia_safe::{SkiaCanvas};
use skia_safe::{Surface, EncodedImageFormat, Font, TextBlob};

fn draw(painter: &mut impl Painter<NativeColor = Color>) {

    painter.clear(Color::WHITE);

    //return;

    let mut paint = Paint::default();

    paint.ink = Color::new(0.0, 0.0, 0.0, 1.0).into();
    //paint.anti_alias = true;
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

    //painter.clip(&BasicShape::Circle { center: Vector2::new(50.0, 50.0), radius: 10.0 }, ClipOperation::Intersect).unwrap();

    //painter.rect((0.0, 0.0, 100.0, 100.0).into(), Color::GREEN.into());

    //painter.line(Vector2::new(0.0, 100.0), Vector2::new(100.0, 0.0), pen.clone());

    painter.with_save(|painter| {
        painter.translate(Vector2::new(5.0, 5.0)).unwrap();
        painter.rotate(45.0).unwrap();
        painter.clip(&BasicShape::Rect((5.0, 5.0, 25.0, 25.0).into()), ClipOperation::Difference).unwrap();
        painter.rect((0.0, 0.0, 10.0, 10.0).into(), Color::BLUE.into());
        painter.point(Vector2::new(50.5, 10.5), Color::BLUE.into());
    });

    //painter.point(Vector2::new(50.5, 10.5), Color::BLUE.into());
    let mut style = FontStyle::default();
    style.weight = 700.into();
    //style.width = FontWidth::UltraExpanded;
    let face = painter.typeface("Arial", style);
    println!("names: {:?}: {:?}", face.family_name(), face.enumerate_family_names());
    //for family in painter.font_manager().families() {
    //    println!("family: {:?}", family);
    //}
    for family in painter.font_manager().families() {
        // TODO let face = family.typeface(0);
        println!("family: {:?}", family);
        let face = painter.typeface(&family, style);
        face.design_parameters();
    }
    println!("parameters: {:?}", face.design_parameters());
    let font = painter.font(&face, 50.0);
    //let font = Font::default();
    let blob = painter.make_text_blob("cgao", &font);
    let pen: Pen<Color> = Color::RED.into();
    painter.rect((50.0, 50.0, 50.0, -50.0).into(), Color::BLUE.into());
    painter.draw_text_blob(&blob, Vector2::new(50.0, 50.0), Color::RED.into());
    //painter.draw_text_blob(&blob, Vector2::new(50.0, 50.0), PaintStyle::Stroke(pen));

    //painter.line(Vector2::new(0.0, 100.0), Vector2::new(100.0, 0.0), pen);
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    //let font = Font::new(("Arial", "a"), 12.0);
    let typeface: skia_safe::Typeface;
    //typeface.clone_with_arguments(arguments)
    let font = Font::default();
    let blob = TextBlob::new("ciao", &font);

    let mut surface = Surface::new_raster_n32_premul((200, 200)).expect("no surface!");
    surface.canvas().clear(skia_safe::Color::WHITE);

    {
        let w = surface.width() as f64;
        let h = surface.height() as f64;
        let mut canvas = SkiaCanvas::new(surface.canvas(), w, h);
        let mut painter = canvas.painter().unwrap();

        let start = std::time::Instant::now();
        draw(&mut painter);
        let end = std::time::Instant::now();
        println!("draw took {:?}", end - start);
    }

    let image = surface.image_snapshot();
    let data = image.encode_to_data(EncodedImageFormat::PNG).unwrap();
    let mut file = File::create("test.png").unwrap();
    let bytes = data.as_bytes();
    file.write_all(bytes).unwrap();

    Ok(())
}