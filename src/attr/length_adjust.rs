use std::str::FromStr;

use crate::error::Error;

use super::AsValue;

/// `lengthAdjust` attribute enumeration
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/lengthAdjust).
pub enum LengthAdjust {
    /// Spacing
    Spacing,
    /// Spacing and glyphs
    SpacingAndGlyphs,
}

impl FromStr for LengthAdjust {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "spacing" => Ok(Self::Spacing),
            "spacingAndGlyphs" => Ok(Self::SpacingAndGlyphs),
            _ => Err(Error::NoOptionFound),
        }
    }
}

impl AsValue for LengthAdjust {
    fn as_value(&self) -> &str {
        match self {
            Self::Spacing => "spacing",
            Self::SpacingAndGlyphs => "spacingAndGlyphs",
        }
    }
}

impl ToString for LengthAdjust {
    fn to_string(&self) -> String {
        self.as_value().to_string()
    }
}
