use nalgebra as na;

use crate::base::{
    clipping::clip_line,
    defs::rect::{FRect, Rect},
};
type Vec2f = na::Vector2<f32>;
type Vec2u = na::Vector2<u32>;
type Vec2i = na::Vector2<i32>;

/// Draw a line from `start` to `end` using the Bresenham? algorithm.
///
/// The algorithm is taken from wikipedia https://en.wikipedia.org/wiki/Line_drawing_algorithm
///
/// ## Arguments
/// The function takes three arguments:
/// * `start` - the start point of the line
/// * `end` - the end point of the line
/// * `plot` - a function that is called for each point on the line
///
/// ## Notes
///  - this algorithm is not optimized for speed, yet
///  - this algorithm is not optimized for integer coordinates, yet
///  - this algorithm is not optimized for vertical or horizontal lines, yet
///  - this algorithm might have some bugs, to check
pub fn line<F: FnMut(u32, u32) -> ()>(
    start: &Vec2f,
    end: &Vec2f,
    plot: &mut F,
    rect: &Rect<u32>,
) -> () {
    // check for NaNs
    if start.x.is_nan() || start.y.is_nan() || end.x.is_nan() || end.y.is_nan() {
        return;
    }

    let f_rect = FRect {
        min: Vec2f::new(rect.min.x as f32, rect.min.y as f32),
        max: Vec2f::new(rect.max.x as f32, rect.max.y as f32),
    };

    if let Some((clipped_start, clipped_end)) = clip_line(&start, &end, &f_rect) {
        let sanitize_vec = |v: Vec2f| -> Vec2u {
            Vec2u::new(
                (v.x.max(0.0).round() as u32)
                    .max(rect.min.x)
                    .min(rect.max.x),
                (v.y.max(0.0).round() as u32)
                    .max(rect.min.y)
                    .min(rect.max.y),
            )
        };

        let start = sanitize_vec(clipped_start);
        let end = sanitize_vec(clipped_end);

        // if the line is a point, plot it
        if start == end {
            plot(start.x, start.y);
            return;
        }

        let delta = Vec2i::new(end.x as i32 - start.x as i32, end.y as i32 - start.y as i32);
        let steep = delta.y.abs() > delta.x.abs();

        if !steep {
            let (start, end): (Vec2u, Vec2u) = if start.x < end.x {
                (start.clone(), end.clone())
            } else {
                (end.clone(), start.clone())
            };

            for x in (start.x as u32)..(end.x as u32) {
                let y = (start.y as f32)
                    + (x as f32 - start.x as f32) * (delta.y as f32) / (delta.x as f32);
                let y = y.min(rect.max.y as f32).max(rect.min.y as f32) as u32;
                plot(x, y);
            }
        } else {
            let (start, end): (Vec2u, Vec2u) = if start.y < end.y {
                (start.clone(), end.clone())
            } else {
                (end.clone(), start.clone())
            };

            for y in (start.y as u32)..(end.y as u32) {
                let x = (start.x as f32)
                    + (y as f32 - start.y as f32) * (delta.x as f32) / (delta.y as f32);
                let x = x.min(rect.max.x as f32).max(rect.min.x as f32) as u32;
                plot(x as u32, y);
            }
        }
    }
}
