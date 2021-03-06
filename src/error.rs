/*
 * --------------------
 * THIS FILE IS LICENSED UNDER MIT
 * THE FOLLOWING MESSAGE IS NOT A LICENSE
 *
 * <barrow@tilde.team> wrote this file.
 * by reading this text, you are reading "TRANS RIGHTS".
 * this file and the content within it is the gay agenda.
 * if we meet some day, and you think this stuff is worth it,
 * you can buy me a beer, tea, or something stronger.
 * -Ezra Barrow
 * --------------------
 */

use crate::impl_from;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// napchart-rs error type
#[derive(Debug)]
pub enum ErrorKind {
    /// Napchart's api returned a chartShape that we dont understand.
    /// This is either my bad or napchart.com's bad.
    InvalidChartShape(String),
    /// Napchart's api returned a chartColor that we dont understand.
    /// This is either my bad or napchart.com's bad.
    InvalidChartColor(String),
    /// Napchart's api returned an element in a lane > the number of lanes.
    /// This is napchart.com's bad.
    InvalidLane(usize, usize),
    /// You tried to add an element to a lane but the space was already taken.
    /// This is your bad.
    ElementOverlap((u16, u16), (u16, u16)),
    // /// You tried to access a vector out of its bounds.
    // /// e.0 is the length of the vec, e.1 is the index you gave.
    // /// This is your bad.
    // OutOfBounds(usize, usize),
    // /// Api returned a number that should have been a usize but wasn't (in an unhandleable way)
    // /// This is probably napchart.com's bad.
    // NotUsizeable,
    /// An error occurred in reqwest.
    ReqwestError(reqwest::Error),
    /// An error occurred in serde_json.
    SerdeError(serde_json::Error),
    /// An IO error occurred.
    IoError(std::io::Error),
    /// An error occurred parsing a time
    ChronoParseError(chrono::ParseError),
    /// An error occured parsing an RGB value
    RgbParseError(colorsys::ParseError),
    /// Attempted to set the tag on an unset custom color
    CustomColorUnset(usize),
}

pub(crate) type Result<T> = std::result::Result<T, ErrorKind>;

impl Error for ErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ReqwestError(e) => Some(e),
            Self::SerdeError(e) => Some(e),
            Self::IoError(e) => Some(e),
            _ => None,
        }
    }
}
impl Display for ErrorKind {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            ErrorKind::InvalidChartShape(s) => write!(
                fmt,
                "Chart shape \"{}\" not supported! this is almost certainly a bug!",
                s
            ),
            ErrorKind::InvalidChartColor(s) => write!(
                fmt,
                "Chart color \"{}\" not supported! this is almost certainly a bug!",
                s
            ),
            ErrorKind::InvalidLane(lane, max) => {
                write!(fmt, "Element's lane {} is invalid! (max {})!", lane, max)
            }
            ErrorKind::ElementOverlap((news, newe), (olds, olde)) => {
                write!(
                    fmt,
                    "New chart element from {} to {} overlaps with existing element from {} to {}",
                    news, newe, olds, olde
                )
            }
            _ => write!(fmt, "Some error has occurred"),
        }
    }
}

impl_from!(reqwest::Error, ErrorKind::ReqwestError);
impl_from!(serde_json::Error, ErrorKind::SerdeError);
impl_from!(std::io::Error, ErrorKind::IoError);
impl_from!(chrono::ParseError, ErrorKind::ChronoParseError);
impl_from!(colorsys::ParseError, ErrorKind::RgbParseError);

#[doc(hidden)]
#[macro_export]
macro_rules! impl_from {
    ($from:path, $to:expr) => {
        impl From<$from> for ErrorKind {
            fn from(e: $from) -> Self {
                $to(e)
            }
        }
    };
}
