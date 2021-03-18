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

#[derive(Debug)]
pub enum ErrorKind {
    NotImplemented,
    InvalidChartShape(String),
    InvalidLane(usize, usize),
    ElementOverlap((u16, u16), (u16, u16)),
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
}

pub type Result<T> = std::result::Result<T, ErrorKind>;

impl Error for ErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ReqwestError(e) => Some(e),
            _ => None,
        }
    }
}
impl Display for ErrorKind {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            ErrorKind::NotImplemented => write!(fmt, "Not implemented! This is probably a bug."),
            ErrorKind::InvalidChartShape(s) => write!(
                fmt,
                "Chart shape \"{}\" not supported! this is almost certainly a bug!",
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
