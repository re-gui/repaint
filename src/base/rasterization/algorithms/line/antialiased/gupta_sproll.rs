use nalgebra as na;
type Vec2f = na::Vector2<f32>;
//type Vec2i = na::Vector2<i32>;

/// See https://en.wikipedia.org/wiki/Line_drawing_algorithm
pub fn line<F: FnMut(u32, u32, f32) -> ()>(start: &Vec2f, end: &Vec2f, plot: &mut F) -> () {
    // TODO non funziona bene
    if start == end {
        return;
    }

    let (start, end) = if start.x > end.x {
        (end, start)
    } else {
        (start, end)
    };

    //let istart: Vec2i = start.map(|x| x.round() as i32);
    //let iend: Vec2i = end.map(|x| x.round() as i32);

    //let p: Vec2i = istart;
    let mut p: Vec2f = start.clone();
    //let d_vec: Vec2i = iend - istart;
    let d_vec: Vec2f = end - start;
    let mut discriminator = 2.0 * d_vec.y - d_vec.x;

    // Euclidean distance of point (x, y) from line (signed)
    let mut distance = 0.0;

    // Euclidean distance between points (x1, y1) and (x2, y2)
    let length = d_vec.norm();

    let sin = d_vec.y / length;
    let cos = d_vec.x / length;

    while p.x <= end.x {
        plot(p.x as u32, (p.y - 1.0) as u32, distance + cos);
        plot(p.x as u32, p.y as u32, distance);
        plot(p.x as u32, (p.y + 1.0) as u32, distance - cos);

        p.x += 1.0;
        if discriminator <= 0.0 {
            distance += sin;
            discriminator += 2.0 * d_vec.y;
        } else {
            distance += sin - cos;
            discriminator += 2.0 * (d_vec.y - d_vec.x);
            p.y += 1.0;
        }
    }
}
