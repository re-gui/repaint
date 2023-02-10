
use nalgebra as na;
type Vec2f = na::Vector2<f32>;
//type Vec2i = na::Vector2<i32>;

/// See https://www.inf.ed.ac.uk/teaching/courses/cg/lectures/cg4_2012.pdf
pub fn line<F: FnMut(u32, u32, f32) -> ()>(_start: &Vec2f, _end: &Vec2f, _plot: &mut F) -> () {
    todo!();
}