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
#![allow(missing_docs)]
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
type StdResult<T, E> = std::result::Result<T, E>;

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
    pub color_tags: HashMap<ChartColor, String>,
    pub custom_colors: [Option<Rgb>; 4],
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
    // TODO: Add Documentation
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
    // TODO: Add Documentation
    pub fn clear_custom_color(&mut self, id: ChartColor) -> Option<Rgb> {
        assert!(id.is_custom());
        let i = id.custom_index().unwrap();
        self.custom_colors[i].take()
    }
    // TODO: Add Documentation
    pub fn set_custom_color(&mut self, id: ChartColor, color: Rgb) -> Option<Rgb> {
        assert!(id.is_custom());
        let i = id.custom_index().unwrap();
        let old = self.custom_colors[i].take();
        self.custom_colors[i] = Some(color);
        old
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
            // Iter of chart lane -> map to
            // Iter of raw::LaneConfig -> enumerate to
            // Iter of (index, raw::LaneConfig)
            lanesConfig: chart
                .lanes
                .iter()
                .map(|l| raw::LaneConfig { locked: l.locked })
                .enumerate()
                .collect(),
            // Iter<chart lane>                             -> enumerate to
            // Iter<(lane index, chart lane)>               -> map to
            // Iter<chart element>, Repeat<lane index>      -> zip to
            // Iter<Iter<(chart element, lane index)>>      -> flatten to
            // Iter<(chart element, lane index)>            -> map to
            // Iter<raw::ChartElement>
            elements: chart
                .lanes
                .into_iter()
                .enumerate()
                .flat_map(|(i, l)| l.elements.into_iter().zip(repeat(i)))
                .map(|(e, i)| raw::ChartElement {
                    start: e.start,
                    end: e.end,
                    lane: i,
                    text: e.data.text,
                    color: e.data.color.to_string(),
                })
                .collect(),
            colorTags: chart
                .color_tags
                .into_iter()
                .map(|(color, tag)| {
                    let custom_color = color
                        .custom_index()
                        .map(|i| custom_colors[i].as_ref().map(Rgb::to_css_string))
                        .flatten();
                    (color, tag, custom_color)
                })
                .map(|(color, tag, custom_color)| raw::ColorTag {
                    tag,
                    color: color.to_string(),
                    colorValue: custom_color,
                })
                .collect(),
        })
    }
}

impl TryFrom<raw::ChartCreationReturn> for RemoteNapchart {
    type Error = ErrorKind;
    fn try_from(raw: raw::ChartCreationReturn) -> Result<RemoteNapchart> {
        Ok(RemoteNapchart {
            chartid: raw.chartDocument.chartid,
            title: raw.chartDocument.title.none_if_empty(),
            description: raw.chartDocument.description.none_if_empty(),
            username: if &raw.chartDocument.username == "anonymous" {
                None
            } else {
                Some(raw.chartDocument.username)
            },
            last_updated: raw.chartDocument.lastUpdated.parse()?,
            is_snapshot: raw.chartDocument.isSnapshot,
            public_link: raw.publicLink.none_if_empty(),
            chart: Napchart {
                shape: raw.chartDocument.chartData.shape.parse()?,
                lanes: {
                    // Initialize lanes vector with capacity
                    let mut vec = Vec::with_capacity(raw.chartDocument.chartData.lanes);
                    // Initialize each lane with its LaneConfig and an empty elements vec
                    for i in 0..raw.chartDocument.chartData.lanes {
                        vec.push(ChartLane {
                            locked: raw
                                .chartDocument
                                .chartData
                                .lanesConfig
                                .get(&i)
                                .map(|c| c.locked)
                                .unwrap_or(false),
                            elements: vec![],
                        });
                    }
                    for e in raw.chartDocument.chartData.elements.into_iter() {
                        // Get the element's lane out of the vec as an Option<Lane>
                        // Map the Option<Lane> to a Option<Vec<Element>>
                        // Map the Option<Lane> to a Result<Lane, ErrorKind::InvalidLane>
                        let lane = vec.get_mut(e.lane).map(|l| &mut l.elements).ok_or(
                            ErrorKind::InvalidLane(e.lane, raw.chartDocument.chartData.lanes),
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
                    raw.chartDocument
                        .chartData
                        .colorTags
                        .iter()
                        .filter_map(|tag| {
                            tag.color
                                .parse::<ChartColor>()
                                .ok()
                                .map(|color| (color, tag.colorValue.as_deref()))
                        })
                        .map(|(color, color_str)| {
                            (
                                color,
                                color_str
                                    .map(Rgb::from_hex_str)
                                    .map(StdResult::ok)
                                    .flatten(),
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
                // Iter<color string>                       -> parse to
                // Iter<Result<ChartColor>>                 -> ok to
                // Iter<Option<ChartColor>>                 -> map to
                // Iter<Option<(ChartColor, tag string)>>   -> filter to
                // Iter<(ChartColor, tag string)>           -> collect to
                // HashMap<ChartColor, String>
                color_tags: raw
                    .chartDocument
                    .chartData
                    .colorTags
                    .into_iter()
                    .map(|tag| Ok((tag.color.parse()?, tag.tag)))
                    .collect::<Result<HashMap<ChartColor, String>>>()?,
            },
        })
    }
}
