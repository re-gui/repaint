use piet_svg::piet::{RenderContext, kurbo::{Point, BezPath}, Color};



fn main() {
    println!("Hello There!");

    let mut piet = piet_svg::RenderContext::new((100.0, 100.0).into());

    let path = circle((50.0, 50.0), 50.0, 100);
    piet.stroke(path, &Color::BLACK, 1.0);

    piet.finish().unwrap();

    let mut out_string = Vec::new();
    piet.write(&mut out_string).unwrap();
    let out_string = String::from_utf8(out_string).unwrap();
    
    // write to file
    std::fs::write("a.svg", out_string).unwrap();
}

fn circle<V: Into<Point>>(center: V, radius: f64, num_segments: usize) -> BezPath {
    let mut path = BezPath::new();
    if num_segments == 0 {
        return path;
    }

    let center = center.into();
    let centerx = center.x;
    let centery = center.y;
    for segment in 0..num_segments {
        let theta = 2.0 * std::f64::consts::PI * (segment as f64) / (num_segments as f64);
        let x = radius * theta.cos();
        let y = radius * theta.sin();
        if segment == 0 {
            path.move_to((x + centerx, y + centery));
        } else {
            let end = (x + centerx, y + centery);
            path.line_to(end);
        }
    }

    path.close_path();
    path
}
