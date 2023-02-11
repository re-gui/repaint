use std::fmt::Debug;

/// A generic color type.
///
/// This trait is implemented by all color types in the `default_color_types` crate,
/// it provides a way to convert between color types so that they can be used
/// together.
pub trait ColorType:
        Sized + Default + Clone + Copy + PartialEq /*+ Eq*/ + PartialOrd /*+ Ord + Hash*/ + Debug +
        Into<default_color_types::RgbaFColor> + From<default_color_types::RgbaFColor>
{
}

pub mod default_color_types {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq /*, Eq*/, PartialOrd /*Ord, Hash*/)]
    pub struct RgbaTColor<T: Sized + Default> {
        pub data: [T; 4],
    }

    impl<T: Sized + Default + Copy> RgbaTColor<T> {
        /// Create a new color from the given RGBA components.
        pub const fn new(r: T, g: T, b: T, a: T) -> RgbaTColor<T> {
            RgbaTColor::<T> { data: [r, g, b, a] }
        }

        // Create a new color from the given RGBA components.
        pub fn from_slice(data: &[T]) -> RgbaTColor<T> {
            assert_eq!(data.len(), 4);
            RgbaTColor::<T> {
                data: [data[0], data[1], data[2], data[3]],
            }
        }

        /// Convert to a slice.
        pub fn to_slice(&self) -> &[T] {
            &self.data
        }

        /// Convert to a mutable slice.
        pub fn to_slice_mut(&mut self) -> &mut [T] {
            &mut self.data
        }

        /// convert to tuple
        pub fn to_tuple(&self) -> (T, T, T, T) {
            (self.data[0], self.data[1], self.data[2], self.data[3])
        }

        /// Get red component.
        pub fn r(&self) -> T {
            self.data[0]
        }

        /// Get mutable red component.
        pub fn r_mut(&mut self) -> &mut T {
            &mut self.data[0]
        }

        /// Get green component.
        pub fn g(&self) -> T {
            self.data[1]
        }

        /// Get mutable green component.
        pub fn g_mut(&mut self) -> &mut T {
            &mut self.data[1]
        }

        /// Get blue component.
        pub fn b(&self) -> T {
            self.data[2]
        }

        /// Get mutable blue component.
        pub fn b_mut(&mut self) -> &mut T {
            &mut self.data[2]
        }

        /// Get alpha component.
        pub fn a(&self) -> T {
            self.data[3]
        }

        /// Get mutable alpha component.
        pub fn a_mut(&mut self) -> &mut T {
            &mut self.data[3]
        }
    }

    impl<T: Sized + Default> Default for RgbaTColor<T> {
        fn default() -> Self {
            RgbaTColor {
                data: std::array::from_fn(|_| T::default()),
            }
        }
    }

    pub type RgbaFColor = RgbaTColor<f32>;
    pub type RgbaColor = RgbaTColor<u8>;
    impl ColorType for RgbaFColor {}
    impl ColorType for RgbaColor {}

    impl From<RgbaFColor> for RgbaColor {
        fn from(color: RgbaFColor) -> Self {
            RgbaColor::new(
                (color.r().min(0.0).max(1.0) * 255.0).round() as u8,
                (color.g().min(0.0).max(1.0) * 255.0).round() as u8,
                (color.b().min(0.0).max(1.0) * 255.0).round() as u8,
                (color.a().min(0.0).max(1.0) * 255.0).round() as u8,
            )
        }
    }

    impl Into<RgbaFColor> for RgbaColor {
        fn into(self) -> RgbaFColor {
            RgbaFColor::new(
                self.r() as f32 / 255.0,
                self.g() as f32 / 255.0,
                self.b() as f32 / 255.0,
                self.a() as f32 / 255.0,
            )
        }
    }
}

pub mod default_colors {

    use super::default_color_types::RgbaFColor;

    pub const TRANSPARENT: RgbaFColor = RgbaFColor::new(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: RgbaFColor = RgbaFColor::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: RgbaFColor = RgbaFColor::new(1.0, 1.0, 1.0, 1.0);
    pub const GRAY: RgbaFColor = RgbaFColor::new(0.5, 0.5, 0.5, 1.0);
    pub const GRAY_SEMI: RgbaFColor = RgbaFColor::new(0.5, 0.5, 0.5, 0.5);

    pub const RED: RgbaFColor = RgbaFColor::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: RgbaFColor = RgbaFColor::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: RgbaFColor = RgbaFColor::new(0.0, 0.0, 1.0, 1.0);

    pub const YELLOW: RgbaFColor = RgbaFColor::new(1.0, 1.0, 0.0, 1.0);
    pub const CYAN: RgbaFColor = RgbaFColor::new(0.0, 1.0, 1.0, 1.0);
    pub const MAGENTA: RgbaFColor = RgbaFColor::new(1.0, 0.0, 1.0, 1.0);

    pub const ORANGE: RgbaFColor = RgbaFColor::new(1.0, 0.5, 0.0, 1.0);
    pub const PURPLE: RgbaFColor = RgbaFColor::new(0.5, 0.0, 1.0, 1.0);
    pub const PINK: RgbaFColor = RgbaFColor::new(1.0, 0.0, 0.5, 1.0);
}
