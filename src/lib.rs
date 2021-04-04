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
// #![deny(missing_docs)]
#![allow(unused_imports)]
#![allow(dead_code)]
//! The `napchart` crate provides strongly-typed bindings to the <https://napchart.com> API.
//!
//! [![GitHub last commit](https://img.shields.io/github/last-commit/barrowsys/napchart-rs)](https://github.com/barrowsys/napchart-rs)
//! [![Crates.io](https://img.shields.io/crates/v/napchart)](https://crates.io/crates/napchart/)
//! [![Docs.rs](https://docs.rs/napchart/badge.svg)](https://docs.rs/napchart)
//!
//! By default, only the api [BlockingClient](api::BlockingClient) is enabled with `default-features = ["blocking"]`.
//! An [AsyncClient](api::AsyncClient) is also provided with crate feature `async`.
//! The following examples will use the blocking client but the process is the same with async
//! except with `.await` thrown in before `.unwrap()`.
//!
//! # `/get`ing a napchart
//! Link to chart: <https://napchart.com/3tbkt>.
//! ```
//! use napchart::api::BlockingClient;
//!
//! let client = BlockingClient::default();
//! let rchart = client.get_chart("3tbkt").unwrap();
//! assert_eq!(rchart.chartid, String::from("3tbkt"));
//! assert_eq!(rchart.title, Some(String::from("State test chart")));
//! assert_eq!(rchart.chart.shape, napchart::ChartShape::Circle);
//! assert_eq!(rchart.chart.lanes.len(), 1);
//! ```
//!
//! # `/create`ing a new napchart
//! ```
//! use napchart::api::BlockingClient;
//! use napchart::Napchart;
//! use napchart::ChartColor;
//!
//! let client = BlockingClient::default();
//! let mut chart = Napchart::default();
//! let lane = chart.add_lane();
//! lane.add_element(420, 1260)
//!     .unwrap()
//!     .text("Nighttime")
//!     .color(ChartColor::Gray);
//! let upload_builder = chart.upload()
//!     .title("lib.rs doctest")
//!     .description("https://crates.io/crates/napchart");
//! let remote_chart = client.create_snapshot(upload_builder).unwrap();
//! assert!(!remote_chart.chartid.is_empty());
//! ```
//!
//! # Getting an image for a napchart
//! ```
//! use napchart::api::BlockingClient;
//! use std::fs::File;
//!
//! let client = BlockingClient::default();
//! let mut file = File::create("/tmp/napchart_3tbkt.png").unwrap();
//! client.get_image("3tbkt", &mut file, (600, 600), None).unwrap();
//! ```

use chrono::prelude::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::repeat;
use std::str::FromStr;
use std::string::ToString;

mod raw;

pub mod api;

mod error;
pub use error::ErrorKind;
use error::Result;

#[derive(PartialEq, Debug, Clone)]
/// A napchart, as seen on <https://napchart.com/>
pub struct Napchart {
    /// The default shape of the napchart on napchart.com
    pub shape: ChartShape,
    /// A vector of all the lanes in the chart.
    /// In circular and wide charts, lane 0 is the innermost and smallest.
    /// In line charts, lane 0 is at the bottom.
    pub lanes: Vec<ChartLane>,
    /// Keys are simple html color names, values are the associated tag.
    pub color_tags: HashMap<ChartColor, String>,
}
impl Napchart {
    //TODO: Replace Napchart.get_id
    // #[allow(non_autolinks)]
    // /// Get the napchart.com ID of this chart, if set.
    // /// This is set by the "get" and "create_new" api functions.
    // /// The ID directly gives you the URL to the napchart, as in https://napchart.com/idcode.
    // pub fn get_id(&self) -> Option<&String> {
    //     self.chartid.as_ref()
    // }
    // TODO: Replace Napchart.chart_eq
    // /// Check if two napcharts are equal, ignoring chartid.
    // /// Used by the API tests to compare the result of create() for both clients
    // pub fn chart_eq(&self, other: &Napchart) -> bool {
    //     self.title == other.title
    //         && self.description == other.description
    //         && self.shape == other.shape
    //         && self.lanes == other.lanes
    //         && self.color_tags == other.color_tags
    // }
    /// Append a new blank lane to the chart and returns a mutable reference to it.
    pub fn add_lane(&mut self) -> &mut ChartLane {
        self.lanes.push(ChartLane {
            locked: false,
            elements: Vec::new(),
        });
        self.lanes.last_mut().unwrap()
    }
    /// Get a mutable reference to the given lane
    pub fn get_lane_mut(&mut self, i: usize) -> Option<&mut ChartLane> {
        self.lanes.get_mut(i)
    }
    pub fn upload(&self) -> api::UploadBuilder {
        api::UploadBuilder::new(self)
    }
    ////TODO: should this be public?
    //fn sort_all_lanes_in_place(&mut self) {
    //    self.lanes.iter_mut().for_each(|l| l.sort_in_place());
    //}
}
/// Builder functions to create new napcharts.
///
/// ```
/// # use napchart::*;
/// let chart = Napchart::default()
///                 .lanes(3)
///                 .shape(ChartShape::Wide);
/// assert_eq!(chart.lanes.len(), 3);
/// assert_eq!(chart.shape, ChartShape::Wide);
/// ```
impl Napchart {
    /// Return Napchart with shape set
    pub fn shape(self, shape: ChartShape) -> Self {
        Self { shape, ..self }
    }
    /// Return Napchart with a given number of empty lanes
    pub fn lanes(self, count: usize) -> Self {
        Self {
            lanes: repeat(ChartLane {
                locked: false,
                elements: Vec::new(),
            })
            .take(count)
            .collect(),
            ..self
        }
    }
}
impl Default for Napchart {
    fn default() -> Self {
        Self {
            shape: ChartShape::Circle,
            lanes: Vec::new(),
            color_tags: HashMap::new(),
        }
    }
}
#[derive(PartialEq, Debug, Clone)]
pub struct RemoteNapchart {
    pub chartid: String, //TODO: Replace this with an Option<ChartId>
    /// The title of the napchart, or None if empty
    pub title: Option<String>,
    /// The description of the napchart, or None if empty
    pub description: Option<String>,
    /// The username that saved this napchart, or None if anonymous
    pub username: Option<String>,
    /// The time that this chart was last saved
    pub last_updated: DateTime<Utc>,
    /// True if this napchart was saved as a snapshot
    pub is_snapshot: bool,
    /// The public link to this napchart as given by the API
    /// We should be able to generate this from the other metadata,
    /// but we'll keep it for now to test against.
    pub(crate) public_link: Option<String>,
    pub chart: Napchart,
}
////TODO: is this really the best way to handle ChartId?
//#[derive(PartialEq, Debug, Clone)]
//pub enum ChartId {
//    CompatFiveChar(String),
//    CompatSixChar(String),
//    Snapshot(String),
//    UserChart(String),
//    Unhandled(String),
//}
// impl ToString for ChartId {
//     fn to_string(&self) -> String {
//         use ChartId::*;
//         match self {
//             CompatFiveChar(s) => s.to_string(),
//             CompatSixChar(s) => s.to_string(),
//             Snapshot(s) => s.to_string(),
//             UserChart(s) => s.to_string(),
//             Unhandled(s) => s.to_string(),
//         }
//     }
// }
#[allow(missing_docs)]
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum ChartColor {
    Red,
    Blue,
    Brown,
    Green,
    Gray,
    Yellow,
    Purple,
    Pink,
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
            _ => return Err(ErrorKind::InvalidChartColor(s.to_string())),
        })
    }
}
/// The shape of a napchart
#[allow(missing_docs)]
#[derive(PartialEq, Debug, Clone)]
pub enum ChartShape {
    Circle,
    Wide,
    Line,
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
/// A single lane of a napchart
#[derive(PartialEq, Debug, Clone)]
pub struct ChartLane {
    /// Whether the lane is locked on napchart.com.
    /// Has no effect on struct functionality.
    pub locked: bool,
    elements: Vec<ChartElement>,
}
impl ChartLane {
    /// Set locked to true (has no effect until uploaded)
    pub fn lock(&mut self) {
        self.locked = true;
    }
    /// Set locked to false (has no effect until uploaded)
    pub fn unlock(&mut self) {
        self.locked = false;
    }
    /// Clear all elements from the lane
    pub fn clear(&mut self) {
        self.elements.clear();
    }
    /// Attempt to add an element to the lane.
    /// If the element would overlap with an existing element, the function fails.
    /// Returns a mutable reference to the new element.
    pub fn add_element(&mut self, start: u16, end: u16) -> Result<&mut ChartElement> {
        assert!(start <= 1440);
        assert!(end <= 1440);
        let mut elems: Vec<(u16, u16, usize)> = Vec::new();
        for (i, e) in self.elements.iter().enumerate() {
            if e.start < e.end {
                elems.push((e.start, e.end, i));
            } else {
                elems.push((e.start, 1440, i));
                elems.push((0, e.end, i));
            }
        }
        println!("{:#?}", elems);
        for e in elems.into_iter() {
            if (start >= e.0 && start < e.1) || (end > e.0 && end <= e.1) {
                let e = &self.elements[e.2];
                return Err(ErrorKind::ElementOverlap((start, end), (e.start, e.end)));
            }
        }
        self.elements.push(ChartElement {
            start,
            end,
            data: ElementData {
                text: None,
                color: ChartColor::default(),
            },
        });
        Ok(self.elements.last_mut().unwrap())
    }
    /// Get an iterator over the elements in the lane.
    pub fn elements_iter(&self) -> std::slice::Iter<ChartElement> {
        self.elements.iter()
    }
    ////TODO: should this be public?
    //fn sort_in_place(&mut self) {
    //    self.elements.sort_unstable_by_key(|elem| elem.end);
    //}
    //TODO: should this be public?
    fn semantic_eq(&self, other: &Self) -> bool {
        let locked_eq = self.locked == other.locked;
        let selems: std::collections::HashSet<&ChartElement> = self.elements_iter().collect();
        let oelems: std::collections::HashSet<&ChartElement> = other.elements_iter().collect();
        let elems_eq = selems == oelems;
        locked_eq && elems_eq
    }
}

/// A single napchart element
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct ChartElement {
    /// The start position of the element as minutes past midnight
    pub start: u16,
    /// The end position of the element as minutes past midnight
    pub end: u16,
    /// Additional element metadata
    pub data: ElementData,
}
/// Additional metadata for a ChartElement
#[derive(PartialEq, Eq, Debug, Clone, Default, Hash)]
pub struct ElementData {
    /// The text annotation on an element
    pub text: Option<String>,
    /// The element's color as a string, e.g. "red" "blue" "green"
    pub color: ChartColor,
}
impl ChartElement {
    /// &mut builder function to set the text of the element.
    /// Returns the reference for following functions.
    /// ```
    /// # use napchart::{Napchart, ChartColor};
    /// let mut chart = Napchart::default();
    /// let mut lane = chart.add_lane();
    /// lane.add_element(0, 120)
    ///     .unwrap()
    ///     .text("Midnight Ritual");
    /// lane.add_element(120, 330)
    ///     .unwrap()
    ///     .text("SWS Core")
    ///     .color(ChartColor::Blue);
    /// ```
    pub fn text<T: ToString>(&mut self, text: T) -> &mut Self {
        self.data.text = Some(text.to_string());
        self
    }
    /// &mut builder function to set the color of the element.
    /// Returns the reference for following functions.
    /// ```
    /// # use napchart::{Napchart, ChartColor};
    /// let mut chart = Napchart::default();
    /// let mut lane = chart.add_lane();
    /// lane.add_element(0, 120)
    ///     .unwrap()
    ///     .color(ChartColor::Gray);
    /// lane.add_element(120, 330)
    ///     .unwrap()
    ///     .color(ChartColor::Blue)
    ///     .text("SWS Core");
    /// ```
    pub fn color(&mut self, color: ChartColor) -> &mut Self {
        self.data.color = color;
        self
    }
}

impl TryFrom<Napchart> for raw::ChartSchema {
    type Error = ErrorKind;
    fn try_from(chart: Napchart) -> Result<raw::ChartSchema> {
        Ok(raw::ChartSchema {
            lanes: chart.lanes.len(),
            shape: chart.shape.to_string(),
            lanesConfig: chart
                .lanes
                .iter()
                .enumerate()
                .map(|(i, l)| (i, raw::LaneConfig { locked: l.locked }))
                .collect(),
            elements: chart
                .lanes
                .into_iter()
                .enumerate()
                .flat_map(|(i, l)| l.elements.into_iter().zip(repeat(i)))
                .map(|(l, i)| raw::ChartElement {
                    end: l.end,
                    lane: i,
                    text: l.data.text.unwrap_or_default(),
                    color: l.data.color.to_string(),
                    start: l.start,
                })
                .collect(),
            colorTags: chart
                .color_tags
                .into_iter()
                .map(|(color, tag)| raw::ColorTag {
                    tag,
                    color: color.to_string(),
                })
                .collect(),
        })
    }
}
// TODO: Replace TryFrom<raw::Napchart> for Napchart
impl TryFrom<raw::ChartDocument> for RemoteNapchart {
    type Error = ErrorKind;
    fn try_from(raw: raw::ChartDocument) -> Result<RemoteNapchart> {
        Ok(RemoteNapchart {
            chartid: raw.chartid, //TODO: parse chartid
            title: raw
                .title
                .and_then(|t| if t.is_empty() { None } else { Some(t) }),
            description: raw
                .description
                .and_then(|t| if t.is_empty() { None } else { Some(t) }),
            username: if &raw.username == "anonymous" {
                None
            } else {
                Some(raw.username)
            },
            last_updated: raw.lastUpdated.parse()?,
            is_snapshot: raw.isSnapshot,
            public_link: None,
            chart: Napchart {
                shape: match raw.chartData.shape.as_str() {
                    "circle" => ChartShape::Circle,
                    "wide" => ChartShape::Wide,
                    "line" => ChartShape::Line,
                    _ => return Err(ErrorKind::InvalidChartShape(raw.chartData.shape.clone())),
                },
                lanes: {
                    let mut vec = Vec::with_capacity(raw.chartData.lanes);
                    for i in 0..raw.chartData.lanes {
                        vec.push(ChartLane {
                            locked: raw
                                .chartData
                                .lanesConfig
                                .get(&i)
                                .map(|c| c.locked)
                                .unwrap_or(false),
                            elements: vec![],
                        });
                    }
                    for e in raw.chartData.elements.iter() {
                        let lane = &mut vec
                            .get_mut(e.lane)
                            .map(|l| &mut l.elements)
                            .ok_or(ErrorKind::InvalidLane(e.lane, raw.chartData.lanes))?;
                        lane.push(ChartElement {
                            start: e.start,
                            end: e.end,
                            data: ElementData {
                                text: if e.text.is_empty() {
                                    None
                                } else {
                                    Some(e.text.clone())
                                },
                                color: e.color.parse()?,
                            },
                        });
                    }
                    vec
                },
                color_tags: {
                    raw.chartData
                        .colorTags
                        .into_iter()
                        .filter_map(|tag| {
                            tag.color.parse().ok().map(|color| (color, tag.tag.clone()))
                        })
                        .collect()
                },
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn apply_permutations<E, R, F: Fn(&Vec<E>) -> R>(mut elems: Vec<E>, f: F) -> Vec<R> {
        fn heap_permutation<E, R, F: Fn(&Vec<E>) -> R>(
            elems: &mut Vec<E>,
            size: usize,
            f: &F,
            res: &mut Vec<R>,
        ) {
            if size == 1 {
                res.push(f(elems));
            } else {
                for i in 0..size {
                    heap_permutation(elems, size - 1, f, res);
                    if size % 2 == 1 {
                        elems.swap(0, size - 1);
                    } else {
                        elems.swap(i, size - 1);
                    }
                }
            }
        }
        let mut res = Vec::new();
        let size = elems.len();
        heap_permutation(&mut elems, size, &f, &mut res);
        res
    }
    #[test]
    fn lane_semantic_eq() {
        let elems = vec![
            (0, 8 * 60),
            (8 * 60, 16 * 60),
            (16 * 60, 19 * 60),
            (21 * 60, (24 * 60) - 1),
        ];
        let lanes = apply_permutations(elems, |v| {
            let mut lane1 = ChartLane {
                locked: false,
                elements: Vec::with_capacity(4),
            };
            for e in v.iter() {
                lane1.add_element(e.0, e.1).unwrap();
            }
            lane1
        });
        for lanei in lanes.iter() {
            for lanej in lanes.iter() {
                assert!(lanei.semantic_eq(lanej));
            }
        }
    }
    #[test]
    fn lane_semantic_neq() {
        let elems = vec![
            (0, 8 * 60),
            (8 * 60, 16 * 60),
            (16 * 60, 19 * 60),
            (21 * 60, (24 * 60) - 1),
        ];
        let mut lanes = Vec::new();
        for i in 1..4 {
            let mut lane = ChartLane {
                locked: false,
                elements: Vec::new(),
            };
            for j in elems.iter().take(i) {
                lane.add_element(j.0, j.1).unwrap();
            }
            lanes.push(lane);
        }
        for i in 1..4 {
            let mut lane = ChartLane {
                locked: false,
                elements: Vec::new(),
            };
            for j in 0..i {
                lane.add_element(elems[3 - j].0, elems[3 - j].1).unwrap();
            }
            lanes.push(lane);
        }
        for lanei in lanes.iter() {
            for lanej in lanes.iter() {
                if lanei != lanej {
                    println!("{:?} != {:?}", lanei, lanej);
                    assert!(!lanei.semantic_eq(lanej));
                }
            }
        }
    }
    //TODO: Redo lib.rs tests
    // #[test]
    // fn title_builder() {
    //     let nc = Napchart {
    //         title: Some(String::from("Test Title")),
    //         ..Default::default()
    //     };
    //     let nc2 = Napchart::default().title("Test Title");
    //     assert_eq!(nc, nc2);
    // }
    // #[test]
    // fn description_builder() {
    //     let nc = Napchart {
    //         description: Some(String::from("Test Description")),
    //         ..Default::default()
    //     };
    //     let nc2 = Napchart::default().description("Test Description");
    //     assert_eq!(nc, nc2);
    // }
    #[test]
    fn shape_builder() {
        let nc = Napchart {
            shape: ChartShape::Wide,
            ..Default::default()
        };
        let nc2 = Napchart::default().shape(ChartShape::Wide);
        assert_eq!(nc, nc2);
    }
    #[test]
    fn lanes_builder() {
        let mut nc = Napchart::default();
        nc.add_lane();
        nc.add_lane();
        let nc2 = Napchart::default().lanes(2);
        assert_eq!(nc, nc2);
    }
}
