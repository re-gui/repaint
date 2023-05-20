
use crate::base::{
    clipping::clip_line,
    defs::rect::{F32Rect, Rect, F64Rect},
};
use crate::base::defs::linalg::*;

/// Draw a line from `start` to `end` using the Bresenham? algorithm.
///
/// The algorithm is taken from wikipedia <https://en.wikipedia.org/wiki/Line_drawing_algorithm>
///
/// ## Arguments
/// The function takes three arguments:
/// * `start` - the start point of the line
/// * `end` - the end point of the line
/// * `plot` - a function that is called for each point on the line
/// * `rect` - the clipping rectangle
/// ## Notes
///  - this algorithm is not optimized for speed, yet
///  - this algorithm is not optimized for integer coordinates, yet
///  - this algorithm is not optimized for vertical or horizontal lines, yet
///  - this algorithm might have some bugs, to check
pub fn line<F: FnMut(u32, u32) -> ()>(
    start: &Vec2f32,
    end: &Vec2f32,
    plot: &mut F,
    rect: &Rect<u32>,
) -> () {
    // check for NaNs
    if start.x.is_nan() || start.y.is_nan() || end.x.is_nan() || end.y.is_nan() {
        return;
    }

    let f_rect = F32Rect {
        min: Vec2f32::new(rect.min.x as f32, rect.min.y as f32),
        max: Vec2f32::new(rect.max.x as f32, rect.max.y as f32),
    };

    if let Some((clipped_start, clipped_end)) = clip_line(&start.cast(), &end.cast(), &F64Rect::new(f_rect.min.cast(), f_rect.max.cast())) {
        let sanitize_vec = |v: Vec2f32| -> Vec2u32 {
            Vec2u32::new(
                (v.x.max(0.0).round() as u32)
                    .max(rect.min.x)
                    .min(rect.max.x),
                (v.y.max(0.0).round() as u32)
                    .max(rect.min.y)
                    .min(rect.max.y),
            )
        };

        let start = sanitize_vec(clipped_start.cast());
        let end = sanitize_vec(clipped_end.cast());

        // if the line is a point, plot it
        if start == end {
            plot(start.x, start.y);
            return;
        }

        let delta = Vec2i32::new(end.x as i32 - start.x as i32, end.y as i32 - start.y as i32);
        let steep = delta.y.abs() > delta.x.abs();

        if !steep {
            let (start, end): (Vec2u32, Vec2u32) = if start.x < end.x {
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
            let (start, end): (Vec2u32, Vec2u32) = if start.y < end.y {
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
