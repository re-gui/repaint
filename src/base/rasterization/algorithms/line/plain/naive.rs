
use nalgebra as na;
type Vec2f = na::Vector2<f32>;

/// From wikipedia https://en.wikipedia.org/wiki/Line_drawing_algorithm
pub fn line<F: FnMut(u32, u32) -> ()>(start: &Vec2f, end: &Vec2f, plot: &mut F) -> () {

    if start == end {
        return;
    }

    let d: Vec2f = end - start;
    let steep = d.y.abs() > d.x.abs();

    if !steep {
        let (start, end): (Vec2f, Vec2f) = if start.x < end.x {
            (start.clone(), end.clone())
        } else {
            (end.clone(), start.clone())
        };

        for x in start.x.round() as u32..end.x.round() as u32 {
            let y = start.y + (x as f32 - start.x) * d.y / d.x;
            plot(x, y.round() as u32);
        }
    } else {

        let (start, end): (Vec2f, Vec2f) = if start.y < end.y {
            (start.clone(), end.clone())
        } else {
            (end.clone(), start.clone())
        };

        for y in start.y.round() as u32..end.y.round() as u32 {
            let x = start.x + (y as f32 - start.y) * d.x / d.y;
            plot(x.round() as u32, y);
        }
    }

}