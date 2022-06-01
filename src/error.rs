// error.rs
//
// Copyright (c) 2020-2022  Douglas Lau
//
use std::fmt;
use std::io;

/// Enum of `semtext` errors
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// A [Glyph] must have a column width of 1 or 2
    ///
    /// [Glyph]: text/struct.Glyph.html
    InvalidGlyphWidth(usize),

    /// Invalid grid area layout
    InvalidGridArea(),

    /// I/O error
    Io(io::Error),
}

/// Result for `semtext` errors
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidGlyphWidth(w) => {
                write!(fmt, "Invalid glyph width: {}", w)
            }
            Error::InvalidGridArea() => {
                write!(fmt, "Invalid grid: all widgets must be rectangular")
            }
            Error::Io(ref err) => err.fmt(fmt),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Io(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
