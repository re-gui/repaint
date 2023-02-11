use nalgebra as na;

use na::Vector2;

use crate::{base::defs::rect::Rect, base::shapes::polyline::BrokenPolylineCommand};

type Vec2f = na::Vector2<f32>;

/*struct LineSpan {
    start_index: usize,
    end_index: usize,

    start_weight: f32,
    end_weight: f32,
}*/

pub trait LineSpanConsumer {
    fn start_line(&mut self, idx: usize);
    fn end_line(&mut self) {}
    fn put_weighted(&mut self, idx: usize, weight: f32);
    fn put_solid_span(&mut self, start_idx: usize, end_idx: usize) {
        for idx in start_idx..end_idx {
            self.put_weighted(idx, 1.0);
        }
    }
}

/*pub trait LineScanner {
    //fn scan_line<CB: Fn(&LineSpan) -> ()>(cb: &CB);
    fn scan_line<C: LineSpanConsumer>(consumer: &mut C);
}*/

#[derive(Clone, Copy, Debug)]
pub struct Segment {
    pub start: Vector2<f32>,
    pub end: Vector2<f32>,
}

impl Segment {
    fn new(start: Vector2<f32>, end: Vector2<f32>) -> Self {
        Self { start, end }
    }
}

static mut LAST_VALUE: usize = 0;

pub fn fill<C: LineSpanConsumer>(
    contour_commands: &[BrokenPolylineCommand],
    clip_rect: &Rect<usize>,
    antialiased: bool,
    consumer: &mut C,
) {
    // check if positively oriented
    //assert!(signed_polygon_area(contour) > 0.0);

    let mut segments = {
        let mut segments = Vec::<Segment>::new();
        segments.reserve(contour_commands.len());

        let mut pos = Vector2::new(0.0, 0.0);

        for cmd in contour_commands {
            match cmd {
                BrokenPolylineCommand::MoveTo(pt) => {
                    pos = *pt;
                }
                BrokenPolylineCommand::LineTo(pt) => {
                    let mut segment = Segment::new(pos, *pt);

                    // ingore degenerate segments
                    if segment.start == segment.end {
                        continue;
                    }

                    // make sure that start.y <= end.y
                    if segment.start.y > segment.end.y {
                        std::mem::swap(&mut segment.start, &mut segment.end);
                    }

                    segments.push(segment);
                    pos = *pt;
                }
            }
        }

        segments.sort_by(|a, b| a.start.y.partial_cmp(&b.start.y).unwrap());

        segments
    };

    let (min, max): (Vec2f, Vec2f) = {
        let mut min = Vec2f::new(std::f32::MAX, std::f32::MAX);
        let mut max = Vec2f::new(std::f32::MIN, std::f32::MIN);
        for segment in &segments {
            min.x = min.x.min(segment.start.x).min(segment.end.x);
            min.y = min.y.min(segment.start.y).min(segment.end.y);
            max.x = max.x.max(segment.start.x).max(segment.end.x);
            max.y = max.y.max(segment.start.y).max(segment.end.y);
        }
        (min, max)
    };

    // The active segments are the segments that are intersecting the current line
    let mut active_segments = Vec::<Segment>::new();

    // The intersections are the x coordinates of the intersections between the current line and the
    // active segments. We need to sort the intersections to know where to start and end the spans.
    // TODO maybe use a binary heap instead of a vector
    let mut intersections = Vec::<f32>::new();
    //let mut intersections = std::collections::BinaryHeap::<F32>::new();

    let (start_line, end_line): (usize, usize) = {
        let start_line = (min.y.floor() as usize).clamp(0, clip_rect.height());
        let end_line = (max.y.ceil() as usize).clamp(0, clip_rect.height());
        (start_line, end_line)
    };

    let y_offset: f32 = 0.0;
    let x_offset: f32 = 0.0;

    //println!("---------");

    // scan the lines
    for y_line in start_line..end_line {
        let y = y_line as f32 + y_offset;

        let active_segments: &Vec<Segment> = {
            active_segments.clear();

            // https://users.rust-lang.org/t/how-to-delete-element-when-iterating-a-vec/65862/6
            {
                let mut idx_wr = 0;
                for idx_rd in 0..segments.len() {
                    if segments[idx_rd].start.y <= y as f32 && segments[idx_rd].end.y > y as f32 {
                        active_segments.push(segments[idx_rd]);
                    } else {
                        if segments[idx_rd].start.y > y as f32 {
                            break;
                        }
                        if segments[idx_rd].end.y <= y as f32 {
                            segments.swap(idx_wr, idx_rd);
                            idx_wr += 1;
                        }
                    }
                    //let segment = &segments[idx_rd];
                }
                segments.drain(..idx_wr);
            }
            unsafe {
                if segments.len() != LAST_VALUE {
                    //println!("at line {}: {} -> {}, dropped {}", y_line, LAST_VALUE, segments.len(), LAST_VALUE as isize - segments.len() as isize);
                    LAST_VALUE = segments.len();
                }
            }

            /*for i in active_segments_index_start..segments.len() {
                let segment = &segments[i];

                if segment.start.y <= y as f32 && segment.end.y > y as f32 {
                    active_segments.push(*segment);
                } else {
                    if segment.end.y <= y as f32 {
                        active_segments_index_start = i + 1;
                    }
                    if segment.start.y > y as f32 {
                        break;
                    }
                }
            }*/

            &active_segments
        };

        let intersections = {
            intersections.clear();

            for segment in active_segments {
                let x = segment.start.x
                    + (segment.end.x - segment.start.x) * ((y as f32 + y_offset) - segment.start.y)
                        / (segment.end.y - segment.start.y);
                let min_x = segment.start.x.min(segment.end.x);
                let max_x = segment.start.x.max(segment.end.x);
                let intersection = (x + x_offset)
                    .clamp(min_x, max_x)
                    .clamp(0.0, clip_rect.width() as f32);
                //intersections.push();
                intersections.push(intersection);
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

            &intersections
        };

        consumer.start_line(y_line);

        if !antialiased {
            if intersections.len() % 2 == 0 {
                for i in (0..intersections.len()).step_by(2) {
                    if i + 1 >= intersections.len() {
                        break;
                    }

                    let start = intersections[i];
                    let end = intersections[i + 1];
                    let start = start.floor() as u32; // ? floor???
                    let end = end.ceil() as u32; // ? floor???
                    consumer.put_solid_span(start as usize, end as usize);
                }
            }
        } else {
            for i in (0..intersections.len()).step_by(2) {
                if i + 1 >= intersections.len() {
                    break;
                }

                let start = intersections[i];
                let end = intersections[i + 1];

                // the first and last pixel are weighted by the distance to the next/previous pixel
                let start_f = start;
                let end_f = end;

                let start = start.floor() as u32;
                let end = end.ceil() as u32;

                let start_weight = 1.0 - start_f.fract();
                let end_weight = end_f.fract();

                if start == end {
                    let x = start;
                    let intensity = start_weight * end_weight;
                    consumer.put_weighted(x as usize, intensity);
                } else {
                    let x = start;
                    let intensity = start_weight;
                    consumer.put_weighted(x as usize, intensity);

                    consumer.put_solid_span(start as usize + 1, end as usize);

                    let x = end;
                    let intensity = end_weight;
                    consumer.put_weighted(x as usize, intensity);
                }
            }
        }

        consumer.end_line();
    }
}

pub fn signed_polygon_area(polygon: &Vec<Vector2<f32>>) -> f32 {
    let mut area: f32 = 0.0;

    for i in 0..polygon.len() {
        let j = (i + 1) % polygon.len();
        area += polygon[i].x * polygon[j].y - polygon[j].x * polygon[i].y;
    }

    area / 2.0
}

#[derive(PartialOrd, PartialEq)]
struct F32 {
    value: f32,
}

impl Eq for F32 {}

impl Ord for F32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
