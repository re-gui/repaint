use crate::base::defs::rect::FRect;

use na::Vector2;
use nalgebra as na;

use crate::base::defs::linalg::*;

/// Clip a rectangle to another rectangle. Returns `None` if the rectangle is completely outside the clip rectangle.
pub fn clip_rect(rect: &FRect, clip_rect: &FRect) -> Option<FRect> {
    // old
    //if !rect.min.x.is_finite() || !rect.min.y.is_finite() || !rect.max.x.is_finite() || !rect.max.y.is_finite() {
    //    return None;
    //}

    if rect.min.x.is_nan() || rect.min.y.is_nan() || rect.max.x.is_nan() || rect.max.y.is_nan() {
        return None;
    }

    Some(FRect {
        min: Vec2f::new(
            rect.min.x.max(clip_rect.min.x),
            rect.min.y.max(clip_rect.min.y),
        ),
        max: Vec2f::new(
            rect.max.x.min(clip_rect.max.x),
            rect.max.y.min(clip_rect.max.y),
        ),
    })
}

/// Clip a line to a rectangle. Returns `None` if the line is completely outside the rectangle.
pub fn clip_line(
    start: &Vector2<f32>,
    end: &Vector2<f32>,
    rect: &FRect,
) -> Option<(Vector2<f32>, Vector2<f32>)> {
    // TODO better checks to allow for infinite lines but not NaNs
    if !start.x.is_finite() || !start.y.is_finite() || !end.x.is_finite() || !end.y.is_finite() {
        return None;
    }

    let mut start = *start;
    let mut end = *end;

    if start == end {
        if (start.x < rect.min.x)
            || (start.x > rect.max.x)
            || (start.y < rect.min.y)
            || (start.y > rect.max.y)
        {
            return None;
        }
    }

    // clip X
    if start.x > end.x {
        std::mem::swap(&mut start, &mut end);
    }
    if ((start.x < rect.min.x) && (end.x < rect.min.x))
        || ((start.x > rect.max.x) && (end.x > rect.max.x))
    {
        return None;
    }
    if start.x < rect.min.x {
        start.y = start.y + (end.y - start.y) * (rect.min.x - start.x) / (end.x - start.x);
        start.x = rect.min.x;
    }
    if end.x > rect.max.x {
        end.y = start.y + (end.y - start.y) * (rect.max.x - start.x) / (end.x - start.x);
        end.x = rect.max.x;
    }

    // clip Y
    if start.y > end.y {
        std::mem::swap(&mut start, &mut end);
    }
    if ((start.y < rect.min.y) && (end.y < rect.min.y))
        || ((start.y > rect.max.y) && (end.y > rect.max.y))
    {
        return None;
    }
    if start.y < rect.min.y {
        start.x = start.x + (end.x - start.x) * (rect.min.y - start.y) / (end.y - start.y);
        start.y = rect.min.y;
    }
    if end.y > rect.max.y {
        end.x = start.x + (end.x - start.x) * (rect.max.y - start.y) / (end.y - start.y);
        end.y = rect.max.y;
    }

    return Some((
        Vector2::<f32>::new(
            start.x.max(rect.min.x).min(rect.max.x),
            start.y.max(rect.min.y).min(rect.max.y),
        ),
        Vector2::<f32>::new(
            end.x.max(rect.min.x).min(rect.max.x),
            end.y.max(rect.min.y).min(rect.max.y),
        ),
    ));
}
