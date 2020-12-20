// glyph.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::{Error, Result, Screen};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Inner enum for glyphs
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum GlyphInner {
    /// Character glyph
    Char(char),
    /// String glyph
    Str(String),
}

/// Printable glyph
///
/// A glyph can be made from a `char` or `&str`:
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use semtext::text::IntoGlyph;
///
/// let glyph_char = '🦀'.into_glyph()?;
/// let glyph_str = "a\u{308}".into_glyph()?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Glyph {
    /// Inner glyph value (char or String)
    inner: GlyphInner,
    /// Width in text cells (must be either 1 or 2)
    width: usize,
}

/// Trait to convert into a Glyph
///
/// This is used instead of TryFrom to avoid error conversion nonsense
pub trait IntoGlyph {
    fn into_glyph(self) -> Result<Glyph>;
}

impl IntoGlyph for char {
    /// Create a Glyph from a `char`
    fn into_glyph(self) -> Result<Glyph> {
        if let Some(width) = self.width() {
            if width == 1 || width == 2 {
                let inner = GlyphInner::Char(self);
                return Ok(Glyph { inner, width });
            }
        }
        Err(Error::InvalidGlyph())
    }
}

impl IntoGlyph for &str {
    /// Create a Glyphn from a `&str`
    fn into_glyph(self) -> Result<Glyph> {
        let width = self.width();
        if width == 1 || width == 2 {
            let inner = GlyphInner::Str(self.to_string());
            return Ok(Glyph { inner, width });
        }
        Err(Error::InvalidGlyph())
    }
}

impl Glyph {
    /// Get the glyph width.
    ///
    /// The width must be either 1 or 2 (checked on construction).
    pub fn width(&self) -> usize {
        self.width
    }

    /// Print glyph to the screen
    pub fn print(&self, screen: &mut Screen) -> Result<()> {
        match &self.inner {
            GlyphInner::Char(ch) => screen.print_char(*ch)?,
            GlyphInner::Str(st) => screen.print_str(&st)?,
        }
        Ok(())
    }
}
