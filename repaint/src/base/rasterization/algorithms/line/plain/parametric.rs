use crate::base::defs::linalg::*;

/// Originally written by copilot.
/// Produces artifacts, maybe it is better to use [`super::naive::line`] instead.
pub fn line<F: FnMut(u32, u32) -> ()>(start: &Vec2f32, end: &Vec2f32, plot: &mut F) -> () {
    if start == end {
        return;
    }

    let len = (end - start).norm();
    let d: Vec2f32 = (end - start).normalize();

    let mut p: Vec2f32 = start.clone();
    for _t in 0..(len as u32) {
        plot(p.x.round() as u32, p.y.round() as u32);
        p += d;
    }
}
