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
#![warn(missing_docs)]
// #![feature(external_doc)]
// #![doc(include = "../README.md")]
//! # napchart-rs
//!
//! [![GitHub last commit](https://img.shields.io/github/last-commit/barrowsys/napchart-rs)](https://github.com/barrowsys/napchart-rs)
//! [![Crates.io](https://img.shields.io/crates/v/napchart)](https://crates.io/crates/napchart/)
//! [![Docs.rs](https://docs.rs/napchart/badge.svg)](https://docs.rs/napchart)
//!
//! A strongly-typed rust interface to the <https://napchart.com> API.
//!
//! The public napchart api is pretty barebones right now, but this will let you use it!
//!
//! ## Usage
//!
//! Add to your Cargo.toml:
//! ```text
//! [dependencies]
//! napchart = "0.2"
//! ```
//!
//! ## Examples
//!
//! ### Creating a new napchart from scratch
//! Example: <https://napchart.com/snapshot/O6kunUfuL>
//! ```
//! use napchart::prelude::*;
//!
//! let mut chart = Napchart::default()
//!     .shape(ChartShape::Circle)
//!     .lanes(1);
//! let first_lane = chart.get_lane_mut(0).unwrap();
//! first_lane.add_element(0, 60).unwrap()
//!     .text("Hour One");
//! first_lane.add_element(180, 240).unwrap()
//!     .text("Hour Four");
//! let second_lane = chart.add_lane();
//! second_lane.add_element(0, 120).unwrap()
//!     .color(ChartColor::Blue);
//! second_lane.add_element(120, 240).unwrap()
//!     .color(ChartColor::Green)
//!     .text("Cool green time");
//! ```
//!
//! ### Downloading a napchart
//! Example Chart: <https://napchart.com/3tbkt>
//! ```
//! use napchart::api::BlockingClient;
//!
//! let client = BlockingClient::default();
//! let rchart = client.get_chart("3tbkt").unwrap();
//! assert_eq!(rchart.chartid, String::from("3tbkt"));
//! assert_eq!(rchart.title, Some(String::from("State test chart")));
//! assert_eq!(rchart.chart.shape, napchart::ChartShape::Circle);
//! assert_eq!(rchart.chart.lanes_len(), 1);
//! ```
//!
//! ### Uploading a napchart as a snapshot
//! Example Output: <https://napchart.com/snapshot/TpCfggr4i>
//! ```no_run
//! use napchart::prelude::*;
//! use napchart::api::BlockingClient;
//!
//! let client = BlockingClient::default();
//! let mut chart = Napchart::default();
//! let lane = chart.add_lane();
//! lane.add_element(420, 1260)
//!     .unwrap()
//!     .text("Nighttime")
//!     .color(ChartColor::Gray);
//! let upload_builder = chart.upload()
//!     .title("readme doctest")
//!     .description("https://crates.io/crates/napchart");
//! let remote_chart = client.create_snapshot(upload_builder).unwrap();
//! assert!(!remote_chart.chartid.is_empty());
//! ```

use chrono::prelude::*;
use colorsys::Rgb;
use noneifempty::NoneIfEmpty;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::repeat;
use std::str::FromStr;
use std::string::ToString;

pub mod api;

mod raw;

mod error;
pub use error::ErrorKind;
use error::Result;
// type StdResult<T, E> = std::result::Result<T, E>;

/// Contains aliases to the useful imports.
/// ```
/// use napchart::prelude::*;
/// let mut chart: Napchart = Napchart::default()
///     .shape(ChartShape::Wide);
/// let lane: &mut ChartLane = chart.add_lane();
/// let elem: &mut ChartElement = lane.add_element(0, 60).unwrap()
///     .color(ChartColor::Green);
/// ```
pub mod prelude {
    pub use super::ChartColor;
    pub use super::ChartElement;
    pub use super::ChartLane;
    pub use super::ChartShape;
    pub use super::ElementData;
    pub use super::Napchart;
    pub use super::RemoteNapchart;
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
/// The shape of a napchart
pub enum ChartShape {
    Circle,
    Wide,
    Line,
}
impl Default for ChartShape {
    fn default() -> Self {
        Self::Circle
    }
}
impl ToString for ChartShape {
    fn to_string(&self) -> String {
        match self {
            ChartShape::Circle => String::from("circle"),
            ChartShape::Wide => String::from("wide"),
            ChartShape::Line => String::from("line"),
        }
    }
}
impl FromStr for ChartShape {
    type Err = ErrorKind;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "circle" => Self::Circle,
            "wide" => Self::Wide,
            "line" => Self::Line,
            _ => return Err(ErrorKind::InvalidChartShape(s.to_string())),
        })
    }
}

// #[allow(missing_docs)]
// #[derive(Clone, Debug, PartialEq)]
// /// The tag associated with a color.
// /// Also holds the rgb value associated with a custom color.
// pub enum ColorTag {
//     Builtin(String),
//     Custom(String, Rgb),
// }
// impl ColorTag {
//     fn from_raw(s: String, c: Option<String>) -> Result<Self> {
//         if let Some(c) = c {
//             Ok(Self::Custom(s, Rgb::from_hex_str(&c)?))
//         } else {
//             Ok(Self::Builtin(s))
//         }
//     }
//     fn unwrap(self) -> (String, Option<Rgb>) {
//         match self {
//             Self::Builtin(s) => (s, None),
//             Self::Custom(s, c) => (s, Some(c)),
//         }
//     }
// }

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// The colors available for chart elements
pub enum ChartColor {
    Red,
    Blue,
    Brown,
    Green,
    Gray,
    Yellow,
    Purple,
    Pink,
    Custom0,
    Custom1,
    Custom2,
    Custom3,
}
impl ChartColor {
    /// True if a color is custom, false if it's one of the builtin colors
    /// ```
    /// # use napchart::*;
    /// assert!(!ChartColor::Blue.is_custom());
    /// assert!(ChartColor::Custom2.is_custom());
    /// ```
    pub fn is_custom(&self) -> bool {
        match self {
            ChartColor::Red
            | ChartColor::Blue
            | ChartColor::Brown
            | ChartColor::Green
            | ChartColor::Gray
            | ChartColor::Yellow
            | ChartColor::Purple
            | ChartColor::Pink => false,
            ChartColor::Custom0
            | ChartColor::Custom1
            | ChartColor::Custom2
            | ChartColor::Custom3 => true,
        }
    }
    /// True if a color is builtin, false if it's one of the custom colors
    /// ```
    /// # use napchart::*;
    /// assert!(ChartColor::Blue.is_builtin());
    /// assert!(!ChartColor::Custom2.is_builtin());
    /// ```
    pub fn is_builtin(&self) -> bool {
        match self {
            ChartColor::Red
            | ChartColor::Blue
            | ChartColor::Brown
            | ChartColor::Green
            | ChartColor::Gray
            | ChartColor::Yellow
            | ChartColor::Purple
            | ChartColor::Pink => true,
            ChartColor::Custom0
            | ChartColor::Custom1
            | ChartColor::Custom2
            | ChartColor::Custom3 => false,
        }
    }
    fn custom_index(&self) -> Option<usize> {
        match self {
            ChartColor::Custom0 => Some(0),
            ChartColor::Custom1 => Some(1),
            ChartColor::Custom2 => Some(2),
            ChartColor::Custom3 => Some(3),
            _ => None,
        }
    }
    fn from_index(i: usize) -> Self {
        assert!(i <= 3);
        match i {
            0 => ChartColor::Custom0,
            1 => ChartColor::Custom1,
            2 => ChartColor::Custom2,
            3 => ChartColor::Custom3,
            _ => unreachable!(),
        }
    }
}
impl Default for ChartColor {
    fn default() -> Self {
        Self::Red
    }
}
impl ToString for ChartColor {
    fn to_string(&self) -> String {
        match self {
            ChartColor::Red => String::from("red"),
            ChartColor::Blue => String::from("blue"),
            ChartColor::Brown => String::from("brown"),
            ChartColor::Green => String::from("green"),
            ChartColor::Gray => String::from("gray"),
            ChartColor::Yellow => String::from("yellow"),
            ChartColor::Purple => String::from("purple"),
            ChartColor::Pink => String::from("pink"),
            ChartColor::Custom0 => String::from("custom_0"),
            ChartColor::Custom1 => String::from("custom_1"),
            ChartColor::Custom2 => String::from("custom_2"),
            ChartColor::Custom3 => String::from("custom_3"),
        }
    }
}
impl FromStr for ChartColor {
    type Err = ErrorKind;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "red" => ChartColor::Red,
            "blue" => ChartColor::Blue,
            "brown" => ChartColor::Brown,
            "green" => ChartColor::Green,
            "gray" => ChartColor::Gray,
            "yellow" => ChartColor::Yellow,
            "purple" => ChartColor::Purple,
            "pink" => ChartColor::Pink,
            "custom_0" => ChartColor::Custom0,
            "custom_1" => ChartColor::Custom1,
            "custom_2" => ChartColor::Custom2,
            "custom_3" => ChartColor::Custom3,
            _ => return Err(ErrorKind::InvalidChartColor(s.to_string())),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
/// A napchart, as seen on <https://napchart.com/>
pub struct Napchart {
    /// The default shape of the napchart on napchart.com
    pub shape: ChartShape,
    lanes: Vec<ChartLane>,
    /// String tags associated with element colors.
    /// These are displayed in the inner area of a napchart,
    /// along with the accumulated amount of time each color takes up.
    color_tags: HashMap<ChartColor, String>,
    /// RGB values for the four custom colors.
    /// If a custom color is None, it is INVALID/Undefined Behavior to set a chart element to it.
    custom_colors: [Option<Rgb>; 4],
}
impl Default for Napchart {
    fn default() -> Self {
        Self {
            shape: Default::default(),
            lanes: Default::default(),
            color_tags: Default::default(),
            custom_colors: [None, None, None, None],
        }
    }
}
impl Napchart {
    /// Append a new blank lane to the chart and return a mutable reference to it.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    /// let mut lane = chart.add_lane();
    /// assert!(lane.is_empty());
    /// assert_eq!(chart.lanes_len(), 1);
    /// ```
    pub fn add_lane(&mut self) -> &mut ChartLane {
        self.lanes.push(ChartLane::default());
        self.lanes.last_mut().unwrap()
    }
    /// Get a reference to the given lane, or None if out of bounds.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    /// chart.add_lane();
    /// assert!(chart.get_lane(0).is_some());
    /// assert!(chart.get_lane(1).is_none());
    /// ```
    pub fn get_lane(&self, i: usize) -> Option<&ChartLane> {
        self.lanes.get(i)
    }
    /// Get a mutable reference to the given lane, or None if out of bounds.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    /// chart.add_lane();
    /// assert!(chart.get_lane_mut(0).is_some());
    /// assert!(chart.get_lane_mut(1).is_none());
    /// ```
    pub fn get_lane_mut(&mut self, i: usize) -> Option<&mut ChartLane> {
        self.lanes.get_mut(i)
    }
    /// Remove the given lane from the chart and return it, or None if out of bounds.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    /// chart.add_lane();
    /// let lane = chart.remove_lane(0);
    /// assert!(lane.is_some());
    /// assert_eq!(chart.lanes_len(), 0);
    /// ```
    pub fn remove_lane(&mut self, i: usize) -> Option<ChartLane> {
        if i < self.lanes.len() {
            Some(self.lanes.remove(i))
        } else {
            None
        }
    }
    /// Get the total number of lanes in the chart.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    /// assert_eq!(chart.lanes_len(), 0);
    /// chart.add_lane();
    /// assert_eq!(chart.lanes_len(), 1);
    /// chart.add_lane();
    /// assert_eq!(chart.lanes_len(), 2);
    /// chart.remove_lane(1);
    /// chart.remove_lane(0);
    /// assert_eq!(chart.lanes_len(), 0);
    /// ```
    pub fn lanes_len(&self) -> usize {
        self.lanes.len()
    }
    /// Create an UploadBuilder with a reference to this chart.
    /// ```
    /// # use napchart::*;
    /// # use napchart::api::mock::BlockingClient;
    /// let client = BlockingClient::default();
    /// let chart = Napchart::default();
    /// let upload: napchart::api::UploadBuilder = chart.upload().title("My Cool Chart");
    /// assert!(client.create_snapshot(upload).is_ok());
    /// ```
    pub fn upload(&self) -> api::UploadBuilder {
        api::UploadBuilder::new(self)
    }
}
/// Getters and setters for color tags and custom colors
impl Napchart {
    /// Get the text tag for a color.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    /// // Napcharts start out with no color tags
    /// assert!(chart.get_color_tag(ChartColor::Blue).is_none());
    ///
    /// chart.set_color_tag(ChartColor::Blue, "Core Sleep").unwrap();
    ///
    /// assert_eq!(chart.get_color_tag(ChartColor::Blue), Some("Core Sleep"));
    /// ```
    pub fn get_color_tag(&self, color: ChartColor) -> Option<&str> {
        self.color_tags.get(&color).map(|s| s.as_str())
    }
    /// Get an iterator over ChartColors and their tags.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    ///
    /// chart.set_color_tag(ChartColor::Blue, "Nap");
    /// chart.set_color_tag(ChartColor::Gray, "Core");
    /// let mut iter = chart.color_tags_iter();
    /// assert!(iter.next().is_some());
    /// assert!(iter.next().is_some());
    /// assert!(iter.next().is_none());
    /// ```
    pub fn color_tags_iter(&self) -> impl Iterator<Item = (&ChartColor, &String)> + '_ {
        self.color_tags.iter()
    }
    /// Set the text tag for a color, returning the value that was replaced.
    /// Returns ErrorKind::CustomColorUnset if you attempt to set the tag on an undefined custom
    /// color.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    ///
    /// let original: Option<String> = chart.set_color_tag(ChartColor::Blue, "Core Sleep").unwrap();
    /// assert!(original.is_none()); // Replaced nothing
    /// assert_eq!(chart.get_color_tag(ChartColor::Blue), Some("Core Sleep"));
    ///
    /// let second: Option<String> = chart.set_color_tag(ChartColor::Blue, "Nap").unwrap();
    /// assert_eq!(second, Some(String::from("Core Sleep")));
    /// assert_eq!(chart.get_color_tag(ChartColor::Blue), Some("Nap"));
    /// ```
    pub fn set_color_tag<T: ToString>(
        &mut self,
        color: ChartColor,
        tag: T,
    ) -> Result<Option<String>> {
        if let Some(index) = color.custom_index() {
            if self.custom_colors[index].is_none() {
                return Err(ErrorKind::CustomColorUnset(index));
            }
        }
        Ok(self.color_tags.insert(color, tag.to_string()))
    }
    /// Set the text tag for a color.
    /// This function does not check custom colors are valid!!
    /// It is invalid/undefined behavior to upload a napchart that uses a custom color without
    /// defining its colorvalue,, "uses" meaning "has a color tag" and/or "is set on an element".
    /// ```
    /// # use napchart::*;
    /// use colorsys::Rgb;
    /// let mut chart = Napchart::default();
    ///
    /// chart.set_custom_color(ChartColor::Custom0, Rgb::from_hex_str("DEDBEF").unwrap());
    /// let res = chart.set_color_tag(ChartColor::Custom0, "Dead Beef");
    /// assert!(res.is_ok());
    ///
    /// chart.remove_custom_color(ChartColor::Custom0);
    /// let res = chart.set_color_tag(ChartColor::Custom0, "Dead Beef");
    /// assert!(res.is_err());
    /// assert!(matches!(res.unwrap_err(), napchart::ErrorKind::CustomColorUnset(0)));
    /// ```
    pub fn set_color_tag_unchecked<T: ToString>(
        &mut self,
        color: ChartColor,
        tag: T,
    ) -> Option<String> {
        self.color_tags.insert(color, tag.to_string())
    }
    /// Removing the text tag for a color, returning the previous value.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    ///
    /// let original: Option<String> = chart.set_color_tag(ChartColor::Blue, "Core Sleep").unwrap();
    /// assert!(original.is_none()); // Replaced nothing
    /// assert_eq!(chart.get_color_tag(ChartColor::Blue), Some("Core Sleep"));
    ///
    /// let second: Option<String> = chart.set_color_tag(ChartColor::Blue, "Nap").unwrap();
    /// assert_eq!(second, Some(String::from("Core Sleep")));
    /// assert_eq!(chart.get_color_tag(ChartColor::Blue), Some("Nap"));
    /// ```
    pub fn remove_color_tag(&mut self, color: ChartColor) -> Option<String> {
        self.color_tags.remove(&color)
    }
    /// Gets the rgb value of a custom color.
    /// May only be called with CustomX ChartColors (asserts ChartColor::is_custom).
    /// ```
    /// # use napchart::*;
    /// use colorsys::Rgb;
    /// let mut chart = Napchart::default();
    ///
    /// assert!(chart.get_custom_color(ChartColor::Custom0).is_none());
    ///
    /// chart.set_custom_color(ChartColor::Custom0, Rgb::from_hex_str("DEDBEF").unwrap());
    /// assert_eq!(chart.get_custom_color(ChartColor::Custom0), Some(&Rgb::from((0xDE, 0xDB, 0xEF))));
    /// ```
    pub fn get_custom_color(&self, id: ChartColor) -> Option<&Rgb> {
        assert!(id.is_custom());
        let i = id.custom_index().unwrap();
        self.custom_colors[i].as_ref()
    }
    /// Get an iterator over custom color (as usize indexes) and their RGB values.
    /// ```
    /// # use napchart::*;
    /// use colorsys::Rgb;
    /// let mut chart = Napchart::default();
    ///
    /// chart.set_custom_color(ChartColor::Custom2, Rgb::from((0xDE, 0xDB, 0xEF)));
    /// chart.set_custom_color(ChartColor::Custom0, Rgb::from((0xB0, 0x0B, 0xE5)));
    /// let mut iter = chart.custom_colors_iter_index();
    /// assert_eq!(iter.next(), Some((0, &Rgb::from((0xB0, 0x0B, 0xE5)))));
    /// assert_eq!(iter.next(), Some((2, &Rgb::from((0xDE, 0xDB, 0xEF)))));
    /// assert!(iter.next().is_none());
    /// ```
    pub fn custom_colors_iter_index(&self) -> impl Iterator<Item = (usize, &Rgb)> + '_ {
        self.custom_colors
            .iter()
            .enumerate()
            .filter_map(|(u, c)| c.as_ref().map(|c| (u, c)))
    }
    /// Get an iterator over custom color (as ChartColors) and their RGB values.
    /// ```
    /// # use napchart::*;
    /// use colorsys::Rgb;
    /// let mut chart = Napchart::default();
    ///
    /// chart.set_custom_color(ChartColor::Custom2, Rgb::from((0xDE, 0xDB, 0xEF)));
    /// chart.set_custom_color(ChartColor::Custom0, Rgb::from((0xB0, 0x0B, 0xE5)));
    /// let mut iter = chart.custom_colors_iter_color();
    /// assert_eq!(iter.next(), Some((ChartColor::Custom0, &Rgb::from((0xB0, 0x0B, 0xE5)))));
    /// assert_eq!(iter.next(), Some((ChartColor::Custom2, &Rgb::from((0xDE, 0xDB, 0xEF)))));
    /// assert!(iter.next().is_none());
    /// ```
    pub fn custom_colors_iter_color(&self) -> impl Iterator<Item = (ChartColor, &Rgb)> + '_ {
        self.custom_colors
            .iter()
            .enumerate()
            .filter_map(|(u, c)| c.as_ref().map(|c| (u, c)))
            .map(|(u, c)| (ChartColor::from_index(u), c))
    }
    /// Sets the rgb value of a custom color, returning the previous value.
    /// May only be called with CustomX ChartColors (asserts ChartColor::is_custom).
    /// ```
    /// # use napchart::*;
    /// use colorsys::Rgb;
    /// let mut chart = Napchart::default();
    ///
    /// assert!(chart.get_custom_color(ChartColor::Custom0).is_none());
    ///
    /// chart.set_custom_color(ChartColor::Custom0, Rgb::from_hex_str("DEDBEF").unwrap());
    /// assert_eq!(chart.get_custom_color(ChartColor::Custom0), Some(&Rgb::from((0xDE, 0xDB, 0xEF))));
    /// ```
    pub fn set_custom_color(&mut self, id: ChartColor, color: Rgb) -> Option<Rgb> {
        assert!(id.is_custom());
        let i = id.custom_index().unwrap();
        self.custom_colors[i].replace(color)
    }
    /// Unsets the rgb value of a custom color, returning the previous value.
    /// May only be called with CustomX ChartColors (asserts ChartColor::is_custom).
    /// Also removes the color_tag associated with the custom color.
    /// (See [_unchecked](#method.remove_custom_color_unchecked))
    /// ```
    /// # use napchart::*;
    /// use colorsys::Rgb;
    /// let mut chart = Napchart::default();
    ///
    /// chart.set_custom_color(ChartColor::Custom0, Rgb::from_hex_str("DEDBEF").unwrap());
    /// chart.set_color_tag(ChartColor::Custom0, "Dead Beef").unwrap();
    ///
    /// chart.remove_custom_color(ChartColor::Custom0);
    ///
    /// assert!(chart.get_custom_color(ChartColor::Custom0).is_none());
    /// assert!(chart.get_color_tag(ChartColor::Custom0).is_none());
    /// ```
    pub fn remove_custom_color(&mut self, id: ChartColor) -> Option<Rgb> {
        assert!(id.is_custom());
        self.remove_color_tag(id.clone());
        self.remove_custom_color_unchecked(id)
    }
    /// Unsets the rgb value of a custom color, returning the previous value.
    /// May only be called with CustomX ChartColors (asserts ChartColor::is_custom).
    /// Does not remove a color_tag associated with the custom color.
    /// It is invalid/undefined behavior to upload a napchart that uses a custom color without
    /// defining its colorvalue,, "uses" meaning "has a color tag" and/or "is set on an element".
    /// ```
    /// # use napchart::*;
    /// use colorsys::Rgb;
    /// let mut chart = Napchart::default();
    ///
    /// chart.set_custom_color(ChartColor::Custom0, Rgb::from_hex_str("DEDBEF").unwrap());
    /// chart.set_color_tag(ChartColor::Custom0, "Dead Beef").unwrap();
    ///
    /// chart.remove_custom_color_unchecked(ChartColor::Custom0);
    ///
    /// assert!(chart.get_custom_color(ChartColor::Custom0).is_none());
    /// assert!(chart.get_color_tag(ChartColor::Custom0).is_some()); // UB if uploaded!
    /// ```
    pub fn remove_custom_color_unchecked(&mut self, id: ChartColor) -> Option<Rgb> {
        assert!(id.is_custom());
        let i = id.custom_index().unwrap();
        self.custom_colors[i].take()
    }
}
/// Builder functions to create new napcharts.
///
/// ```
/// # use napchart::*;
/// let chart = Napchart::default()
///                 .lanes(3)
///                 .shape(ChartShape::Wide);
/// assert_eq!(chart.lanes_len(), 3);
/// assert_eq!(chart.shape, ChartShape::Wide);
/// ```
impl Napchart {
    /// Return Napchart with shape set.
    /// ```
    /// # use napchart::*;
    /// let chart = Napchart::default();
    /// assert_eq!(chart.shape, ChartShape::Circle);
    ///
    /// let wide_chart = Napchart::default().shape(ChartShape::Wide);
    /// assert_eq!(wide_chart.shape, ChartShape::Wide);
    /// ```
    pub fn shape(self, shape: ChartShape) -> Self {
        Self { shape, ..self }
    }
    /// Return Napchart with a given number of empty lanes.
    /// ```
    /// # use napchart::*;
    /// let chart = Napchart::default();
    /// assert_eq!(chart.lanes_len(), 0);
    ///
    /// let chart2 = Napchart::default().lanes(3);
    /// assert_eq!(chart2.lanes_len(), 3);
    /// ```
    pub fn lanes(self, count: usize) -> Self {
        Self {
            lanes: repeat(ChartLane::default()).take(count).collect(),
            ..self
        }
    }
}

/// A napchart downloaded from <https://napchart.com>.
/// Includes extra metadata around the internal Napchart, such as the chart's ID, title, author, update time, etc.
#[derive(Debug)]
pub struct RemoteNapchart {
    /// The chart's ID code. Chartids are unique.
    /// Should be in one of the following formats:
    /// * 5 chars (`napchart.com/xxxxx`) (deprecated)
    /// * 6 chars (`napchart.com/xxxxxx`) (deprecated)
    /// * 9 chars snapshot (`napchart.com/snapshot/xxxxxxxxx`)
    /// * 9 chars user chart (`napchart.com/:user/xxxxxxxxx`)
    /// * 9 chars user chart with title (`napchart.com/:user/Some-title-here-xxxxxxxxx`)
    pub chartid: String,
    /// The title of the napchart, or None if empty
    pub title: Option<String>,
    /// The description of the napchart, or None if empty
    pub description: Option<String>,
    /// The user that saved this napchart, or None if anonymous
    pub username: Option<String>,
    /// The time that this chart was last saved
    pub last_updated: DateTime<Utc>,
    /// True if this napchart was saved as a snapshot
    pub is_snapshot: bool,
    /// The public link to this napchart.
    /// (Note: We should be able to generate this from the other metadata)
    pub public_link: Option<String>,
    /// The chart itself
    pub chart: Napchart,
}
impl RemoteNapchart {
    /// True if both RemoteNapcharts are the same, ignoring chartid, last_updated, and public_link.
    /// Used by the api tests to make sure BlockingClient and AsyncClient are doing the same thing.
    pub fn semantic_eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.description == other.description
            && self.username == other.username
            && self.is_snapshot == other.is_snapshot
            && self.chart == other.chart
    }
}

/// A single lane of a napchart
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChartLane {
    /// Whether the lane is locked on napchart.com.
    /// Has no effect in rust.
    pub locked: bool,
    elements: Vec<ChartElement>,
}
impl ChartLane {
    /// Clear all elements from the lane.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    /// let mut lane = chart.add_lane();
    /// lane.add_element(0, 60).unwrap();
    /// lane.add_element(60, 120).unwrap();
    /// lane.add_element(120, 180).unwrap();
    /// assert!(!lane.is_empty());
    /// lane.clear();
    /// assert!(lane.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.elements.clear();
    }
    /// True if the lane has no elements.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    /// let mut lane = chart.add_lane();
    /// assert!(lane.is_empty());
    /// lane.add_element(0, 60).unwrap();
    /// assert!(!lane.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
    /// The number of elements in the lane.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    /// let mut lane = chart.add_lane();
    /// assert_eq!(lane.elems_len(), 0);
    /// lane.add_element(0, 60).unwrap();
    /// lane.add_element(60, 120).unwrap();
    /// lane.add_element(120, 180).unwrap();
    /// assert_eq!(lane.elems_len(), 3);
    /// ```
    pub fn elems_len(&self) -> usize {
        self.elements.len()
    }
    /// Add a new element to the lane.
    /// Start and end must both be between [0 and 1440).
    /// Error if the new element would overlap with the existing elements.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    /// let mut lane = chart.add_lane();
    /// assert!(lane.add_element(0, 30).is_ok());
    /// assert!(lane.add_element(15, 45).is_err());
    /// assert!(lane.add_element(30, 60).is_ok());
    /// assert_eq!(lane.elems_len(), 2);
    /// ```
    pub fn add_element(&mut self, start: u16, end: u16) -> Result<&mut ChartElement> {
        assert!(start <= 1440);
        assert!(end <= 1440);
        // Turns self.elements into a vec of (start, end, index),
        // splitting midnight-crossers in two.
        let mut elems: Vec<(u16, u16, usize)> = Vec::new();
        for (i, e) in self.elements.iter().enumerate() {
            if e.start < e.end {
                elems.push((e.start, e.end, i));
            } else {
                elems.push((e.start, 1440, i));
                elems.push((0, e.end, i));
            }
        }
        for e in elems.into_iter() {
            // If the new element starts or ends within any of the current elements
            if (start >= e.0 && start < e.1) || (end > e.0 && end <= e.1) {
                // Error out
                let e = &self.elements[e.2];
                return Err(ErrorKind::ElementOverlap((start, end), (e.start, e.end)));
            }
        }
        // Otherwise, add the element...
        self.elements.push(ChartElement {
            start,
            end,
            ..Default::default()
        });
        // ...and return it
        Ok(self.elements.last_mut().unwrap())
    }
    /// Get an iterator over the elements in the lane.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    /// let mut lane = chart.add_lane();
    /// lane.add_element(0, 60).unwrap();
    /// lane.add_element(60, 120).unwrap();
    /// lane.add_element(120, 180).unwrap();
    /// let mut iter = lane.elems_iter();
    /// assert_eq!(iter.next().unwrap().get_position(), (0, 60));
    /// assert_eq!(iter.next().unwrap().get_position(), (60, 120));
    /// assert_eq!(iter.next().unwrap().get_position(), (120, 180));
    /// ```
    pub fn elems_iter(&self) -> std::slice::Iter<ChartElement> {
        self.elements.iter()
    }
}

/// A single element in a napchart.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChartElement {
    start: u16,
    end: u16,
    /// Additional metadata for the element.
    pub data: ElementData,
}
impl ChartElement {
    /// Returns the position of the element as (start, end),
    /// where start and end are minutes past midnight.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    /// let mut lane = chart.add_lane();
    /// let elem = lane.add_element(0, 60).unwrap();
    /// assert_eq!(elem.get_position(), (0, 60));
    /// ```
    pub fn get_position(&self) -> (u16, u16) {
        (self.start, self.end)
    }
    // unsafe fn set_position(&mut self, start: u16, end: u16) {
    //     self.start = start;
    //     self.end = end;
    // }
    /// &mut builder function to set the text of an element.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    /// let mut lane = chart.add_lane();
    /// let elem = lane.add_element(0, 60).unwrap().text("Hour One");
    /// assert_eq!(elem.data.text, Some(String::from("Hour One")));
    /// ```
    pub fn text<T: ToString>(&mut self, text: T) -> &mut Self {
        self.data.text = Some(text.to_string());
        self
    }
    /// &mut builder function to set the color of an element.
    /// ```
    /// # use napchart::*;
    /// let mut chart = Napchart::default();
    /// let mut lane = chart.add_lane();
    /// let elem = lane.add_element(0, 60).unwrap().color(ChartColor::Blue);
    /// assert_eq!(elem.data.color, ChartColor::Blue);
    /// ```
    pub fn color(&mut self, color: ChartColor) -> &mut Self {
        self.data.color = color;
        self
    }
}

/// Additional metadata for an element.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ElementData {
    /// The text description attached to the element
    pub text: Option<String>,
    /// The element's color
    pub color: ChartColor,
}

impl TryFrom<Napchart> for raw::ChartSchema {
    type Error = ErrorKind;
    fn try_from(chart: Napchart) -> Result<raw::ChartSchema> {
        let custom_colors = chart.custom_colors;
        Ok(raw::ChartSchema {
            lanes: chart.lanes.len(),
            shape: chart.shape.to_string(),
            // Iter<chart lane>         -> map to
            // Iter<raw::LaneConfig>    -> enumerate to
            // Iter<(index, raw::LaneConfig)>
            lanes_config: chart
                .lanes
                .iter()
                .map(|l| raw::LaneConfig { locked: l.locked })
                .enumerate()
                .collect(),
            // Iter<chart lane>                         -> enumerate to
            // Iter<(lane index, chart lane)>           -> map to
            // Iter<(
            //     Iter<chart element>,
            //     Repeat<lane index>
            // )>                                       -> map(zip) to
            // Iter<Iter<(chart element, lane index)>>  -> flatten to
            // Iter<(chart element, lane index)>        -> map to
            // Iter<raw::ChartElement>
            elements: chart
                .lanes
                .into_iter()
                .enumerate()
                .map(|(i, l)| (l.elements.into_iter(), repeat(i)))
                .flat_map(|(s, o)| s.zip(o))
                .map(|(e, i)| raw::ChartElement {
                    start: e.start,
                    end: e.end,
                    lane: i,
                    text: e.data.text,
                    color: e.data.color.to_string(),
                })
                .collect(),
            // Iter<(ChartColor, String)>               -> map to
            // Iter<(ChartColor, String, ColorValue)>   -> map to
            // Iter<ColorTag>
            color_tags: chart
                .color_tags
                .into_iter()
                .map(|(color, tag)| {
                    let rgb = color
                        .custom_index()
                        .map(|i| custom_colors[i].as_ref().map(Rgb::to_css_string))
                        .flatten();
                    (color, tag, rgb)
                })
                .map(|(color, tag, rgb)| raw::ColorTag {
                    tag,
                    color: color.to_string(),
                    color_value: rgb,
                })
                .collect(),
        })
    }
}

impl TryFrom<raw::ChartCreationReturn> for RemoteNapchart {
    type Error = ErrorKind;
    fn try_from(raw: raw::ChartCreationReturn) -> Result<RemoteNapchart> {
        Ok(RemoteNapchart {
            chartid: raw.chart_document.chartid,
            title: raw.chart_document.title.none_if_empty(),
            description: raw.chart_document.description.none_if_empty(),
            username: if &raw.chart_document.username == "anonymous" {
                None
            } else {
                Some(raw.chart_document.username)
            },
            last_updated: raw.chart_document.last_updated.parse()?,
            is_snapshot: raw.chart_document.is_snapshot,
            public_link: raw.public_link.none_if_empty(),
            chart: Napchart {
                shape: raw.chart_document.chart_data.shape.parse()?,
                lanes: {
                    // Initialize lanes vector with capacity
                    let mut vec = Vec::with_capacity(raw.chart_document.chart_data.lanes);
                    // Initialize each lane with its LaneConfig and an empty elements vec
                    for i in 0..raw.chart_document.chart_data.lanes {
                        vec.push(ChartLane {
                            locked: raw
                                .chart_document
                                .chart_data
                                .lanes_config
                                .get(&i)
                                .map(|c| c.locked)
                                .unwrap_or(false),
                            elements: vec![],
                        });
                    }
                    for e in raw.chart_document.chart_data.elements.into_iter() {
                        // Get the element's lane out of the vec as an Option<Lane>
                        // Map the Option<Lane> to a Option<Vec<Element>>
                        // Map the Option<Lane> to a Result<Lane, ErrorKind::InvalidLane>
                        let lane = vec.get_mut(e.lane).map(|l| &mut l.elements).ok_or(
                            ErrorKind::InvalidLane(e.lane, raw.chart_document.chart_data.lanes),
                        )?;
                        lane.push(ChartElement {
                            start: e.start,
                            end: e.end,
                            data: ElementData {
                                text: e.text.none_if_empty(),
                                color: e.color.parse()?,
                            },
                        });
                    }
                    vec
                },
                custom_colors: {
                    let mut r = [None, None, None, None];
                    // Iter<ColorTag>                       -> map to
                    // Iter<(color name, Opt<rgb str>)>     -> parse to
                    // Iter<Opt<(ChartColor, Opt<&str>)>>   -> filter to
                    // Iter<(ChartColor, Opt<&str>)>        -> map to
                    // Iter<(ChartColor, Opt<Res<Rgb>>)>    -> transpose to
                    // Iter<(ChartColor, Res<Opt<Rgb>>)>    -> ok to
                    // Iter<(ChartColor, Opt<Opt<Rgb>>)>    -> flatten to
                    // Iter<(ChartColor, Opt<Rgb>)>         -> foreach
                    raw.chart_document
                        .chart_data
                        .color_tags
                        .iter()
                        .filter_map(|tag| {
                            Some((
                                tag.color.parse::<ChartColor>().ok()?,
                                tag.color_value.as_deref(),
                            ))
                        })
                        .map(|(color, color_str)| {
                            (
                                color,
                                color_str.map(Rgb::from_hex_str).transpose().ok().flatten(),
                            )
                        })
                        .for_each(|(color, color_value)| match color {
                            ChartColor::Custom0 => r[0] = color_value,
                            ChartColor::Custom1 => r[1] = color_value,
                            ChartColor::Custom2 => r[2] = color_value,
                            ChartColor::Custom3 => r[3] = color_value,
                            _ => {}
                        });
                    r
                },
                // Iter<ColorTag>                           -> map to
                // Iter<(color string, tag>                 -> parse to
                // Iter<(Result<color>, tag)>               -> prop error
                // Iter<Result<(color, tag)>>               -> collect to
                // Result<HashMap<ChartColor, String>>      -> prop error
                // HashMap<ChartColor, tag>
                color_tags: raw
                    .chart_document
                    .chart_data
                    .color_tags
                    .into_iter()
                    .map(|tag| Ok((tag.color.parse()?, tag.tag)))
                    .collect::<Result<HashMap<ChartColor, String>>>()?,
            },
        })
    }
}
