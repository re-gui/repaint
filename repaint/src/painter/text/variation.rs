use std::ops::RangeInclusive;


// https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_fonts/Variable_fonts_guide
#[derive(Debug, Clone, PartialEq)]
pub struct FontVariationParameter {
    pub axis: FontVariationAxis,
    pub value: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FontVariationAxis {
    pub tag: FontVariationAxisTag,
    pub range: RangeInclusive<f32>,
    pub default: f32,
    //pub hidden: bool,
    // TODO flags
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontVariationAxisTag {
    /// Weight - `wght`
    Weight,
    /// Width - `wdth`
    Width,
    /// Slant - `slnt`
    Slant,
    /// Optical size - `opsz`
    OpticalSize,
    /// Custom axis (4-byte uppercase ascii tag)
    ///
    /// For example, `FontVariationAxis::Custom(*b"XTRA")` for the `XTRA` axis.
    Custom([u8; 4]),
}

// TODO ord based on bytes

impl FontVariationAxisTag {
    pub fn tag(&self) -> [u8; 4] {
        match self {
            FontVariationAxisTag::Weight => *b"wght",
            FontVariationAxisTag::Width => *b"wdth",
            FontVariationAxisTag::Slant => *b"slnt",
            FontVariationAxisTag::OpticalSize => *b"opsz",
            FontVariationAxisTag::Custom(tag) => *tag,
        }
    }
    pub fn from_bytes(bytes: [u8; 4]) -> Self {
        // TODO use match
        if bytes == *b"wght" {
            FontVariationAxisTag::Weight
        } else if bytes == *b"wdth" {
            FontVariationAxisTag::Width
        } else if bytes == *b"slnt" {
            FontVariationAxisTag::Slant
        } else if bytes == *b"opsz" {
            FontVariationAxisTag::OpticalSize
        } else {
            FontVariationAxisTag::Custom(bytes)
        }
    }
    pub fn from_bytes_slice(bytes: &[u8]) -> Option<Self> { // TODO error
        if bytes.len() == 4 {
            let mut tag = [0; 4];
            tag.copy_from_slice(bytes);
            Some(FontVariationAxisTag::from_bytes(tag))
        } else {
            None
        }
    }
    pub fn from_str(s: &str) -> Option<Self> { // TODO error
        match s {
            "wght" => Some(FontVariationAxisTag::Weight),
            "wdth" => Some(FontVariationAxisTag::Width),
            "slnt" => Some(FontVariationAxisTag::Slant),
            "opsz" => Some(FontVariationAxisTag::OpticalSize),
            _ => {
                // TODO check length, non-ascii, uppercase, non-alpha, etc.
                let mut tag = [0; 4];
                tag.copy_from_slice(s.as_bytes());
                Some(FontVariationAxisTag::Custom(tag))
            }
        }
    }

    // TODO fix function that checks if Custom tag registered tag
}

impl From<FontVariationAxisTag> for [u8; 4] {
    fn from(axis: FontVariationAxisTag) -> Self {
        axis.tag()
    }
}

impl From<FontVariationAxisTag> for String {
    fn from(axis: FontVariationAxisTag) -> Self {
        String::from_utf8(axis.tag().to_vec()).unwrap()
    }
}

impl From<[u8; 4]> for FontVariationAxisTag {
    fn from(tag: [u8; 4]) -> Self {
        FontVariationAxisTag::from_bytes(tag)
    }
}