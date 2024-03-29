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

pub trait ColorWithAlpha: ColorType {
    fn alpha_value(&self) -> f32;
    fn set_alpha_value(&mut self, alpha: f32);
    fn with_alpha_value(&self, alpha: f32) -> Self {
        let mut color = *self;
        color.set_alpha_value(alpha);
        color
    }
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
    impl ColorWithAlpha for RgbaFColor {
        fn alpha_value(&self) -> f32 {
            self.a()
        }
        fn set_alpha_value(&mut self, alpha: f32) {
            *self.a_mut() = alpha;
        }
    }
    impl ColorWithAlpha for RgbaColor {
        fn alpha_value(&self) -> f32 {
            self.a() as f32 / 255.0
        }
        fn set_alpha_value(&mut self, alpha: f32) {
            *self.a_mut() = (alpha.min(0.0).max(1.0) * 255.0).round() as u8;
        }
    }

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

    /// A CSS style color.
    pub enum CssColor {
        /// None color
        /// 
        /// In most cases, it is the same as transparent.
        None,

        /// Css like RGB color.
        /// 
        /// Values for the components are in the range 0 to 255.
        Rgb{r: u8, g: u8, b: u8},

        /// Css like RGBA color.
        /// 
        /// Values for the components are in the range 0 to 255 while the alpha value is in the range 0.0 to 1.0.
        Rgba{r: u8, g: u8, b: u8, a: f32},

        /// RGB color with floating point components.
        Rgbf{r: f32, g: f32, b: f32},

        /// RGBA color with floating point components.
        Rgbaf{r: f32, g: f32, b: f32, a: f32},

        /// Hex color.
        Hex([u8; 3]),

        /// Hex color with alpha.
        Hexa([u8; 4]),

        /// HSL color.
        /// 
        /// The hue component is in the range 0 to 360 while the saturation and lightness components are in the range 0.0 to 1.0.
        Hsl{h: f32, s: f32, l: f32},

        /// HSLA color.
        /// 
        /// The hue component is in the range 0 to 360 while the saturation, lightness and alpha components are in the range 0.0 to 1.0.
        Hsla{h: f32, s: f32, l: f32, a: f32},

        Named(StandardCssColors),
    }

    impl CssColor {
        /// Parse a CSS color string.
        pub fn from_css_string(s: &str) -> CssColor {
            if let Some(color) = Self::from_css_string_checked(s) {
                color
            } else {
                CssColor::None
            }
        }

        /// Parse a CSS color string.
        pub fn from_css_string_checked(s: &str) -> Option<CssColor> {
            // TODO most of the code is generated by copilot, check if it is correct
            let s = s.trim();

            if s.starts_with("rgb(") && s.ends_with(")") {
                let s = &s[4..s.len() - 1];
                let mut iter = s.split(',');
                let r = iter.next()?.trim().parse::<u8>().ok()?; // ? written by copilot ????
                let g = iter.next()?.trim().parse::<u8>().ok()?;
                let b = iter.next()?.trim().parse::<u8>().ok()?;
                return Some(CssColor::Rgb{r, g, b});
            }

            if s.starts_with("rgba(") && s.ends_with(")") {
                let s = &s[5..s.len() - 1];
                let mut iter = s.split(',');
                let r = iter.next()?.trim().parse::<u8>().ok()?;
                let g = iter.next()?.trim().parse::<u8>().ok()?;
                let b = iter.next()?.trim().parse::<u8>().ok()?;
                let a = iter.next()?.trim().parse::<f32>().ok()?;
                return Some(CssColor::Rgba{r, g, b, a});
            }

            if s.starts_with("hsl(") && s.ends_with(")") {
                let s = &s[4..s.len() - 1];
                let mut iter = s.split(',');
                let h = iter.next()?.trim().parse::<f32>().ok()?;
                let s = iter.next()?.trim().parse::<f32>().ok()?;
                let l = iter.next()?.trim().parse::<f32>().ok()?;
                return Some(CssColor::Hsl{h, s, l});
            }

            if s.starts_with("hsla(") && s.ends_with(")") {
                let s = &s[5..s.len() - 1];
                let mut iter = s.split(',');
                let h = iter.next()?.trim().parse::<f32>().ok()?;
                let s = iter.next()?.trim().parse::<f32>().ok()?;
                let l = iter.next()?.trim().parse::<f32>().ok()?;
                let a = iter.next()?.trim().parse::<f32>().ok()?;
                return Some(CssColor::Hsla{h, s, l, a});
            }

            if s.starts_with("#") {
                let s = &s[1..];
                if s.len() == 3 {
                    let mut iter = s.chars();
                    let r = iter.next()?.to_digit(16)? * 17;
                    let g = iter.next()?.to_digit(16)? * 17;
                    let b = iter.next()?.to_digit(16)? * 17;
                    return Some(CssColor::Hex([r as u8, g as u8, b as u8]));
                } else if s.len() == 6 {
                    let mut iter = s.chars();
                    let r = iter.next()?.to_digit(16)? * 16 + iter.next()?.to_digit(16)?;
                    let g = iter.next()?.to_digit(16)? * 16 + iter.next()?.to_digit(16)?;
                    let b = iter.next()?.to_digit(16)? * 16 + iter.next()?.to_digit(16)?;
                    return Some(CssColor::Hex([r as u8, g as u8, b as u8]));
                } else if s.len() == 4 {
                    let mut iter = s.chars();
                    let r = iter.next()?.to_digit(16)? * 17;
                    let g = iter.next()?.to_digit(16)? * 17;
                    let b = iter.next()?.to_digit(16)? * 17;
                    let a = iter.next()?.to_digit(16)? * 17;
                    return Some(CssColor::Hexa([r as u8, g as u8, b as u8, a as u8]));
                } else if s.len() == 8 {
                    let mut iter = s.chars();
                    let r = iter.next()?.to_digit(16)? * 16 + iter.next()?.to_digit(16)?;
                    let g = iter.next()?.to_digit(16)? * 16 + iter.next()?.to_digit(16)?;
                    let b = iter.next()?.to_digit(16)? * 16 + iter.next()?.to_digit(16)?;
                    let a = iter.next()?.to_digit(16)? * 16 + iter.next()?.to_digit(16)?;
                    return Some(CssColor::Hexa([r as u8, g as u8, b as u8, a as u8]));
                }
            }

            if let Some(color) = StandardCssColors::from_css_string_checked(s) {
                return Some(color.into());
            }

            None
        }
    }

    /// Standard CSS colors.
    /// 
    /// Example:
    /// ```rust
    /// let color: CssColor = css_named::Black.into();
    /// ```
    pub mod css_named {
        pub use super::StandardCssColorsL1::*;
        pub use super::StandardCssColorsL2::*;
        // TODO add L3
        // TODO add L4
    }

    /// Standard CSS colors.
    pub enum StandardCssColors {
        L1(StandardCssColorsL1),
        L2(StandardCssColorsL2),
        // TODO add L3
        // TODO add L4
    }

    impl StandardCssColors {
        pub fn to_css_rgba(&self) -> CssColor {
            match self {
                StandardCssColors::L1(color) => color.to_css_rgba(),
                StandardCssColors::L2(color) => color.to_css_rgba(),
                // TODO add L3
                // TODO add L4
            }
        }
        pub fn from_css_string_checked(s: &str) -> Option<Self> {
            if let Some(color) = StandardCssColorsL1::from_css_string_checked(s) {
                return Some(StandardCssColors::L1(color));
            }
            if let Some(color) = StandardCssColorsL2::from_css_string_checked(s) {
                return Some(StandardCssColors::L2(color));
            }
            // TODO add L3
            // TODO add L4
            None
        }
    }

    impl Into<CssColor> for StandardCssColors {
        fn into(self) -> CssColor {
            match self {
                StandardCssColors::L1(color) => color.into(),
                StandardCssColors::L2(color) => color.into(),
                // TODO add L3
                // TODO add L4
            }
        }
    }

    /// Standard CSS colors defined in CSS Level 1.
    /// 
    /// See <https://developer.mozilla.org/en-US/docs/Web/CSS/named-color>
    pub enum StandardCssColorsL1 {
        Black,
        Silver,
        Gray,
        White,
        Maroon,
        Red,
        Purple,
        Fuchsia,
        Green,
        Lime,
        Olive,
        Yellow,
        Navy,
        Blue,
        Teal,
        Aqua,
    }

    impl StandardCssColorsL1 {
        pub fn to_css_rgba(&self) -> CssColor {
            use StandardCssColorsL1::*;
            match self {
                Black => CssColor::Hex([0x00, 0x00, 0x00]),
                Silver => CssColor::Hex([0xC0, 0xC0, 0xC0]),
                Gray => CssColor::Hex([0x80, 0x80, 0x80]),
                White => CssColor::Hex([0xFF, 0xFF, 0xFF]),
                Maroon => CssColor::Hex([0x80, 0x00, 0x00]),
                Red => CssColor::Hex([0xFF, 0x00, 0x00]),
                Purple => CssColor::Hex([0x80, 0x00, 0x80]),
                Fuchsia => CssColor::Hex([0xFF, 0x00, 0xFF]),
                Green => CssColor::Hex([0x00, 0x80, 0x00]),
                Lime => CssColor::Hex([0x00, 0xFF, 0x00]),
                Olive => CssColor::Hex([0x80, 0x80, 0x00]),
                Yellow => CssColor::Hex([0xFF, 0xFF, 0x00]),
                Navy => CssColor::Hex([0x00, 0x00, 0x80]),
                Blue => CssColor::Hex([0x00, 0x00, 0xFF]),
                Teal => CssColor::Hex([0x00, 0x80, 0x80]),
                Aqua => CssColor::Hex([0x00, 0xFF, 0xFF]),
            }
        }

        pub fn from_css_string_checked(s: &str) -> Option<Self> {
            use StandardCssColorsL1::*;
            match s {
                "black" => Some(Black),
                "silver" => Some(Silver),
                "gray" => Some(Gray),
                "white" => Some(White),
                "maroon" => Some(Maroon),
                "red" => Some(Red),
                "purple" => Some(Purple),
                "fuchsia" => Some(Fuchsia),
                "green" => Some(Green),
                "lime" => Some(Lime),
                "olive" => Some(Olive),
                "yellow" => Some(Yellow),
                "navy" => Some(Navy),
                "blue" => Some(Blue),
                "teal" => Some(Teal),
                "aqua" => Some(Aqua),
                _ => None,
            }
        }
    }

    impl Into<CssColor> for StandardCssColorsL1 {
        fn into(self) -> CssColor {
            CssColor::Named(StandardCssColors::L1(self))
        }
    }

    /// Standard CSS colors defined in CSS Level 2.
    /// 
    /// See <https://developer.mozilla.org/en-US/docs/Web/CSS/named-color>
    pub enum StandardCssColorsL2 {
        Orange,
    }

    impl StandardCssColorsL2 {
        pub fn to_css_rgba(&self) -> CssColor {
            use StandardCssColorsL2::*;
            match self {
                Orange => CssColor::Hex([0xFF, 0xA5, 0x00]),
            }
        }

        pub fn from_css_string_checked(s: &str) -> Option<Self> {
            use StandardCssColorsL2::*;
            match s {
                "orange" => Some(Orange),
                _ => None,
            }
        }
    }

    impl Into<CssColor> for StandardCssColorsL2 {
        fn into(self) -> CssColor {
            CssColor::Named(StandardCssColors::L2(self))
        }
    }
}

pub mod default_colors {

    use super::default_color_types::RgbaFColor;

    impl RgbaFColor {
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
}

/// Convert a color from RGB to HSL.
/// 
/// The input ranges are int the range [0, 1].
/// The output ranges are:
///   hue: [0, 360]
///   saturation: [0, 1]
///   lightness: [0, 1]
pub fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    // TODO written by copilot, to check
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / delta % 6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };

    let lightness = (max + min) / 2.0;

    let saturation = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * lightness - 1.0).abs())
    };

    (hue, saturation, lightness)
}
