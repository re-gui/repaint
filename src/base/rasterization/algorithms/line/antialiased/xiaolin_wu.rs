use nalgebra as na;
type Vec2f = na::Vector2<f32>;

pub fn line<F: FnMut(u32, u32, f32) -> ()>(start: &Vec2f, end: &Vec2f, plot: &mut F) -> () {
    // https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
    // ! probably not correct, has some problems at the line ends

    if start == end {
        return;
    }

    let mut y0 = start.y;
    let mut y1 = end.y;
    let mut x0 = start.x;
    let mut x1 = end.x;

    let steep = (y1 - y0).abs() > (x1 - x0).abs();

    let mut plot = |x: u32, y: u32, c: f32| {
        if steep {
            plot(y, x, c);
        } else {
            plot(x, y, c);
        }
    };

    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = (x1 - x0) as f32;
    let dy = (y1 - y0) as f32;

    if dx == 0.0 {
        return;
    }

    let gradient = if dx == 0.0 { 1.0 } else { dy / dx };

    // integer part of x
    fn ipart(x: f32) -> i32 {
        x.floor() as i32
    }
    fn fpart(x: f32) -> f32 {
        x.fract()
    }
    fn rfpart(x: f32) -> f32 {
        1.0 - fpart(x)
    }

    // handle first endpoint
    let xend = x0.round() as i32;
    let yend = y0 + gradient * (xend as f32 - x0);
    let xgap = rfpart(x0 + 0.5);
    let xpxl1 = xend; // this will be used in the main loop
    let ypxl1 = ipart(yend);
    plot(xpxl1 as u32, ypxl1 as u32, rfpart(yend) * xgap);
    plot(xpxl1 as u32, (ypxl1 + 1) as u32, fpart(yend) * xgap);
    let mut intery = yend + gradient; // first y-intersection for the main loop

    // handle second endpoint
    let xend = x1.round() as i32;
    let yend = y1 + gradient * (xend as f32 - x1);
    let xgap = fpart(x1 + 0.5);
    let xpxl2 = xend; //this will be used in the main loop
    let ypxl2 = ipart(yend);
    plot(xpxl2 as u32, ypxl2 as u32, rfpart(yend) * xgap);
    plot(xpxl2 as u32, (ypxl2 + 1) as u32, fpart(yend) * xgap);

    // main loop
    //return;
    for x in (xpxl1 + 1)..(xpxl2 - 1 + 1) {
        plot(x as u32, ipart(intery) as u32, rfpart(intery));
        plot(x as u32, (ipart(intery) + 1) as u32, fpart(intery));
        intery = intery + gradient;
    }
}
