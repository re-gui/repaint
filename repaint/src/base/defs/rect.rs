use std::ops::{Add, AddAssign, Sub, SubAssign};

use na::Vector2;
use nalgebra as na;

pub type FRect = Rect<f32>;

/// This is a rectangle with a min and max point.
#[derive(Debug, Clone, Copy)]
pub struct Rect<T>
where
    T: na::Scalar + Copy + Sub<Output = T> + Add<Output = T> + AddAssign + PartialOrd,
{
    pub min: Vector2<T>,
    pub max: Vector2<T>,
}

impl<T> Rect<T>
where
    T: na::Scalar + Copy + Sub<Output = T> + Add<Output = T> + AddAssign + SubAssign + PartialOrd,
{
    /// Creates a new rectangle from a min and max point.
    /// The min and max points are not checked for validity.
    pub fn new(min: Vector2<T>, max: Vector2<T>) -> Rect<T> {
        return Rect { min: min, max: max };
    }

    /// Creates a new rectangle from a min and max point.
    /// The min and max points are inverted if necessary.
    pub fn new_checked(min: Vector2<T>, max: Vector2<T>) -> Rect<T> {
        let min = Vector2::<T>::new(
            *tmp_partial_min(&min.x, &max.x),
            *tmp_partial_min(&min.y, &max.y),
        );
        let max = Vector2::<T>::new(
            *tmp_partial_max(&min.x, &max.x),
            *tmp_partial_max(&min.y, &max.y),
        );

        return Rect { min, max };
    }

    /// Creates a new rectangle from a position and size.
    /// If the size is negative, the rectangle is inverted.
    pub fn from_pos_and_size(pos: Vector2<T>, size: Vector2<T>) -> Rect<T> {
        return Self::new_checked(pos, pos + size);
    }

    /// Creates a new rectangle from a position and size.
    /// If the size is negative, the rectangle is inverted.
    pub fn from_pos_and_size_coords(x: T, y: T, width: T, height: T) -> Rect<T> {
        Self::from_pos_and_size(Vector2::new(x, y), Vector2::new(width, height))
    }

    /// Get the width of the rectangle.
    pub fn width(&self) -> T {
        self.max.x - self.min.x
    }

    /// Get the height of the rectangle.
    pub fn height(&self) -> T {
        self.max.y - self.min.y
    }

    /// Get the size of the rectangle.
    /// This is the same as the width and height.
    pub fn size(&self) -> Vector2<T> {
        self.max - self.min
    }

    /// Intersects two rectangles and returns the result.
    /// If the rectangles do not intersect, None is returned.
    pub fn intersect(&self, other: &Rect<T>) -> Option<Rect<T>> {
        let min = Vector2::<T>::new(
            *tmp_partial_max(&self.min.x, &other.min.x),
            *tmp_partial_max(&self.min.y, &other.min.y),
        );
        let max = Vector2::<T>::new(
            *tmp_partial_min(&self.max.x, &other.max.x),
            *tmp_partial_min(&self.max.y, &other.max.y),
        );

        if min.x < max.x && min.y < max.y {
            return Some(Rect { min, max });
        }

        return None;
    }

    /// Returns true if the rectangle contains the given point.
    pub fn contains(&self, point: &Vector2<T>) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    /// get the x range of the rectangle
    pub fn x_range(&self) -> (T, T) {
        (self.min.x, self.max.x)
    }

    // get the y range of the rectangle
    pub fn y_range(&self) -> (T, T) {
        (self.min.y, self.max.y)
    }

    /// Reorders min and max if necessary.
    pub fn reorder(&mut self) {
        if self.min.x > self.max.x {
            std::mem::swap(&mut self.min.x, &mut self.max.x);
        }

        if self.min.y > self.max.y {
            std::mem::swap(&mut self.min.y, &mut self.max.y);
        }
    }
}

/// `VideoRect` represents a rectangle that can be used for video.
/// Since video coordinates are usually y-down, this trait provides
/// methods to get the corners of the rectangle.
pub trait VideoRect<T> {
    /// Get the top left corner of the rectangle.
    fn top_left(&self) -> Vector2<T>;

    /// Get the top right corner of the rectangle.
    fn top_right(&self) -> Vector2<T>;

    // Get the bottom left corner of the rectangle.
    fn bottom_left(&self) -> Vector2<T>;

    // Get the bottom right corner of the rectangle.
    fn bottom_right(&self) -> Vector2<T>;
}

impl<T> VideoRect<T> for Rect<T>
where
    T: na::Scalar + Copy + Sub<Output = T> + Add<Output = T> + AddAssign + SubAssign + PartialOrd,
{
    fn top_left(&self) -> Vector2<T> {
        self.min
    }

    fn top_right(&self) -> Vector2<T> {
        Vector2::new(self.max.x, self.min.y)
    }

    fn bottom_left(&self) -> Vector2<T> {
        Vector2::new(self.min.x, self.max.y)
    }

    fn bottom_right(&self) -> Vector2<T> {
        self.max
    }
}

/// Returns the minimum of two values using PartialOrd.
fn tmp_partial_min<'a, T>(a: &'a T, b: &'a T) -> &'a T
where
    T: PartialOrd,
{
    if a < b {
        a
    } else {
        b
    }
}

/// Returns the maximum of two values using PartialOrd.
fn tmp_partial_max<'a, T>(a: &'a T, b: &'a T) -> &'a T
where
    T: PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}
