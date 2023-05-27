
use std::cmp::Ordering;

use super::*;

mod variation;

pub use variation::*;

/// The **weight** of a font.
///
/// The weight of a font describes how bold or light the font appears and it is maily used in the
/// [`FontStyle`] to describe the style of a font.
///
/// In this crate, there is a correspondence between the font weight and the
/// skia [`SkFontStyle::Weight`](https://api.skia.org/classSkFontStyle.html)'s number:
///
/// | Weight | Number |
/// | ------ | ------ |
/// | [`Invisible`] | `0` |
/// | [`Thin`] | `100` |
/// | [`ExtraLight`] | `200` |
/// | [`Light`] | `300` |
/// | [`Normal`] | `400` |
/// | [`Medium`] | `500` |
/// | [`SemiBold`] | `600` |
/// | [`Bold`] | `700` |
/// | [`ExtraBold`] | `800` |
/// | [`Black`] | `900` |
/// | [`ExtraBlack`] | `1000` |
/// | [`Custom(weight)`] | `weight` |
///
/// The [`Custom(weight)`] variant allows to use any weight number
/// outside the common values.
///
/// # References
/// - <https://api.skia.org/classSkFontStyle.html>
///
/// [`Invisible`]: FontWeight::Invisible
/// [`Thin`]: FontWeight::Thin
/// [`ExtraLight`]: FontWeight::ExtraLight
/// [`Light`]: FontWeight::Light
/// [`Normal`]: FontWeight::Normal
/// [`Medium`]: FontWeight::Medium
/// [`SemiBold`]: FontWeight::SemiBold
/// [`Bold`]: FontWeight::Bold
/// [`ExtraBold`]: FontWeight::ExtraBold
/// [`Black`]: FontWeight::Black
/// [`ExtraBlack`]: FontWeight::ExtraBlack
/// [`Custom(weight)`]: FontWeight::Custom
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum FontWeight {
    /// `0`
    Invisible,
    /// `100`
    Thin,
    /// `200`
    ExtraLight,
    /// `300`
    Light,
    /// `400`
    Normal,
    /// `500`
    Medium,
    /// `600`
    SemiBold,
    /// `700`
    Bold,
    /// `800`
    ExtraBold,
    /// `900`
    Black,
    /// `1000`
    ExtraBlack,
    /// Custom weight.
    ///
    /// This variant allows to use any weight number outside the common values.
    Custom(i32),
}

impl PartialOrd for FontWeight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.to_number().cmp(&other.to_number()))
    }
}

impl Ord for FontWeight {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_number().cmp(&other.to_number())
    }
}

/// The default font weight is [`Normal`](FontWeight::Normal).
impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Normal
    }
}

impl FontWeight {
    /// Returns the [`FontWeight`] corresponding to the given number.
    ///
    /// [`FontWeight::Custom`] is returned if the number is not one of the common values.
    pub fn from_number(weight: i32) -> FontWeight {
        match weight {
            0 => FontWeight::Invisible,
            100 => FontWeight::Thin,
            200 => FontWeight::ExtraLight,
            300 => FontWeight::Light,
            400 => FontWeight::Normal,
            500 => FontWeight::Medium,
            600 => FontWeight::SemiBold,
            700 => FontWeight::Bold,
            800 => FontWeight::ExtraBold,
            900 => FontWeight::Black,
            1000 => FontWeight::ExtraBlack,
            _ => FontWeight::Custom(weight),
        }
    }

    /// Fixes the font weight.
    ///
    /// If the weight is [`Custom(weight)`], it is converted to the corresponding
    /// common value if possible.
    pub fn fix(self) -> Self {
        Self::from_number(self.to_number())
    }

    /// Returns the number corresponding to the [`FontWeight`].
    pub fn to_number(&self) -> i32 {
        match self {
            FontWeight::Invisible => 0,
            FontWeight::Thin => 100,
            FontWeight::ExtraLight => 200,
            FontWeight::Light => 300,
            FontWeight::Normal => 400,
            FontWeight::Medium => 500,
            FontWeight::SemiBold => 600,
            FontWeight::Bold => 700,
            FontWeight::ExtraBold => 800,
            FontWeight::Black => 900,
            FontWeight::ExtraBlack => 1000,
            FontWeight::Custom(weight) => *weight,
        }
    }

    pub fn is_bold(&self) -> bool {
        self > &FontWeight::Normal
    }
}

/// Converts a number to a [`FontWeight`].
///
/// See [`FontWeight::from_number`].
impl From<i32> for FontWeight {
    fn from(weight: i32) -> FontWeight {
        FontWeight::from_number(weight)
    }
}

/// Converts a string to a [`FontWeight`].
///
/// This trait implementation uses the [`FromStr`] trait, see the [`FromStr`] implementation for
/// [`FontWeight`].
impl From<&'static str> for FontWeight {
    fn from(value: &'static str) -> Self {
        if let Ok(weight) = value.parse::<FontWeight>() {
            weight
        } else {
            FontWeight::Normal
        }
    }
}

/// Converts a [`FontWeight`] to a string.
impl ToString for FontWeight {
    fn to_string(&self) -> String {
        match self {
            FontWeight::Invisible => "invisible",
            FontWeight::Thin => "thin",
            FontWeight::ExtraLight => "extra-light",
            FontWeight::Light => "light",
            FontWeight::Normal => "normal",
            FontWeight::Medium => "medium",
            FontWeight::SemiBold => "semi-bold",
            FontWeight::Bold => "bold",
            FontWeight::ExtraBold => "extra-bold",
            FontWeight::Black => "black",
            FontWeight::ExtraBlack => "extra-black",
            FontWeight::Custom(weight) => return weight.to_string(),
        }.to_string()
    }
}

/// Parses a string to a [`FontWeight`].
///
/// # Examples
/// ```
/// use repaint::FontWeight;
/// use std::str::FromStr;
/// assert_eq!("bold".parse::<FontWeight>(), Ok(FontWeight::Bold));
/// assert_eq!("100".parse::<FontWeight>(), Ok(FontWeight::Thin));
/// assert_eq!("42".parse::<FontWeight>(), Ok(FontWeight::Custom(42)));
/// assert_eq!("foo".parse::<FontWeight>(), Err(()));
/// ```
impl FromStr for FontWeight {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(number) = s.parse::<i32>() {
            Ok(number.into())
        } else {
            match s.to_lowercase().as_str() {
                "invisible" => Ok(FontWeight::Invisible),
                "thin" => Ok(FontWeight::Thin),
                "extra-light" => Ok(FontWeight::ExtraLight),
                "light" => Ok(FontWeight::Light),
                "normal" => Ok(FontWeight::Normal),
                "medium" => Ok(FontWeight::Medium),
                "semi-bold" => Ok(FontWeight::SemiBold),
                "bold" => Ok(FontWeight::Bold),
                "extra-bold" => Ok(FontWeight::ExtraBold),
                "black" => Ok(FontWeight::Black),
                "extra-black" => Ok(FontWeight::ExtraBlack),
                _ => Err(()),
            }
        }
    }
}

/// The font width.
///
/// The font width is also known as the font stretch and describes the relative change from the
/// normal aspect ratio as specified by the font designer. It is mainly used in [`FontStyle`].
///
/// The font width is a number between 1 and 9, where 1 is the most condensed and 9 the most
/// expanded. The default font width is [`Normal`](FontWidth::Normal).
/// 
/// # References
/// - <https://api.skia.org/classSkFontStyle.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum FontWidth {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
    Custom(i32),
}
// TODO conversions to/from number and string

/// The default font width is [`Normal`](FontWidth::Normal).
impl Default for FontWidth {
    fn default() -> Self {
        FontWidth::Normal
    }
}

/// The font slant.
///
/// The font slant is also known as the font slope and describes the angle of the glyphs in the
/// font. It is mainly used in [`FontStyle`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, PartialOrd, Ord)]
pub enum FontSlant {
    Upright,
    Italic,
    Oblique,
}
// TODO conversions to/from number and string

impl FontSlant {
    pub fn is_italic(&self) -> bool {
        self > &FontSlant::Upright
    }
}

/// The default font slant is [`Upright`](FontSlant::Upright).
impl Default for FontSlant {
    fn default() -> Self {
        FontSlant::Upright
    }
}

/// The font style.
///
/// TODO ...
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontStyle {
    /// The font weight.
    pub weight: FontWeight,
    /// The font width.
    pub width: FontWidth,
    /// The font slant.
    pub slant: FontSlant,
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle {
            weight: Default::default(),
            width: Default::default(),
            slant: Default::default(),
        }
    }
}

// TODO move elsewhere
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalizedString { // TODO or font name?
    pub string: String,
    pub language: String,
}

pub trait FontManager {
    type Iter: Iterator<Item = String>;

    fn families(&self) -> Self::Iter;
}

// https://api.skia.org/classSkTypeface.html#a1cd98b2bdf57d80e06917c24c5526b59
pub trait Typeface: Sized {
    //type GlyphId; // TODO a trait for glyph id such as hashable, copy, etc.

    //fn style(&self) -> FontStyle;
    //fn is_fixed_pitch(&self) -> bool;
    //fn glyph_id(&self, c: char) -> Self::GlyphId;
    // TODO ...

    fn style(&self) -> FontStyle;

    /// Returns `true` if the typeface **claims** to be fixed pitch.
    fn is_fixed_pitch(&self) -> bool;

    // https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_fonts/Variable_fonts_guide
    fn design_parameters(&self) -> Vec<FontVariationParameter>; // TODO avoid vec, use iterator or cow or something similar

    fn with_parameters(self, parameters: &[FontVariationParameter]) -> Option<Self>; // TODO error type

    fn family_name(&self) -> String; // TODO replace with Cow, Cow<'static, str>, str or something similar to avoid allocation

    fn enumerate_family_names(&self) -> Vec<LocalizedString>;
}

pub trait Glyph {
    // TODO 
}

pub trait Font {
    type Typeface: Typeface;
    //type Glyph: Glyph;

    //fn typeface(&self) -> &Self::Typeface;
    //fn glyph(&self, c: char) -> Self::Glyph;
    // TODO ...
}

pub trait TextBlob {
    // TODO ...
}

pub trait WithText<'context>: BasicPainter<'context> {
    type Typeface: Typeface;
    type Font: Font<Typeface = Self::Typeface>;
    type TextBlob: TextBlob;
    type FontManager: FontManager;

    fn font_manager(&self) -> &Self::FontManager;

    fn typeface(&mut self, family_name: &str, style: FontStyle) -> Self::Typeface;
    // TODO a method to load from file with `read: impl Read` parameter

    fn font(&mut self, typeface: &Self::Typeface, size: f32) -> Self::Font;

    fn make_text_blob(&mut self, text: &str, font: &Self::Font) -> Self::TextBlob;

    fn draw_text_blob(&mut self, text_blob: &Self::TextBlob, pos: Vec2f64, style: PaintStyle<Self::NativeColor>);
}