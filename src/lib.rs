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

//! The `napchart` crate provides strongly-typed bindings to the <https://napchart.com> API.
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
//! let chart = client.get("3tbkt").unwrap();
//! assert_eq!(chart.get_id(), Some(&String::from("3tbkt")));
//! assert_eq!(chart.title, Some(String::from("State test chart")));
//! assert_eq!(chart.shape, napchart::ChartShape::Circle);
//! assert_eq!(chart.lanes.len(), 1);
//! ```
//!
//! # `/create`ing a new napchart
//! ```
//! use napchart::api::BlockingClient;
//! use napchart::Napchart;
//!
//! let client = BlockingClient::default();
//! let mut chart = Napchart::default()
//!     .title("lib.rs doctest")
//!     .description("https://crates.io/crates/napchart");
//! let lane = chart.add_lane();
//! lane.add_element(420, 1260)
//!     .unwrap()
//!     .text("Nighttime")
//!     .color("grey");
//! assert!(chart.get_id().is_none());
//! // client.create_new(&mut chart).unwrap();
//! // assert!(chart.get_id().is_some());
//! ```

use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::repeat;
use std::string::ToString;

mod raw;

pub mod api;

mod error;
pub use error::ErrorKind;
use error::Result;

#[derive(PartialEq, Debug, Clone)]
/// A napchart, as seen on <https://napchart.com/>
pub struct Napchart {
    chartid: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub shape: ChartShape,
    /// A vector of all the lanes in the chart.
    /// In circular and wide charts, lane 0 is the innermost and smallest.
    /// In line charts, lane 0 is at the bottom.
    pub lanes: Vec<ChartLane>,
    /// Keys are simple html color names, values are the associated tag.
    pub color_tags: HashMap<String, String>,
}
impl Napchart {
    /// Get the napchart.com ID of this chart, if set.
    /// This is set by the "get" and "create_new" api functions.  
    /// The ID directly gives you the URL to the napchart, as in https://napchart.com/idcode.
    pub fn get_id(&self) -> Option<&String> {
        self.chartid.as_ref()
    }
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
}
/// Builder functions to create new napcharts.
///
/// ```
/// # use napchart::Napchart;
/// let chart = Napchart::default()
///                 .title("Title1")
///                 .description("Cool Chart")
///                 .lanes(3);
/// assert_eq!(chart.title, Some("Title1".to_string()));
/// assert_eq!(chart.description, Some("Cool Chart".to_string()));
/// assert_eq!(chart.lanes.len(), 3);
/// ```
impl Napchart {
    /// Return Napchart with title set
    pub fn title<'a, T: Into<&'a str>>(self, title: T) -> Self {
        Self {
            title: Some(title.into().to_string()),
            ..self
        }
    }
    /// Return Napchart with description set
    pub fn description<'a, T: Into<&'a str>>(self, description: T) -> Self {
        Self {
            description: Some(description.into().to_string()),
            ..self
        }
    }
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
            chartid: None,
            title: None,
            description: None,
            shape: ChartShape::Circle,
            lanes: Vec::new(),
            color_tags: HashMap::new(),
        }
    }
}
/// The shape of a napchart
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
                color: String::from("red"),
            },
        });
        Ok(self.elements.last_mut().unwrap())
    }
    /// Get an iterator over the elements in the lane.
    pub fn elements_iter(&self) -> std::slice::Iter<ChartElement> {
        self.elements.iter()
    }
}

/// A single napchart element
#[derive(PartialEq, Debug, Clone)]
pub struct ChartElement {
    pub start: u16,
    pub end: u16,
    pub data: ElementData,
}
/// Additional metadata for a ChartElement
#[derive(PartialEq, Debug, Clone)]
pub struct ElementData {
    pub text: Option<String>,
    pub color: String,
}
impl ChartElement {
    /// &mut builder function to set the text of the element.
    /// Returns the reference for following functions.
    /// ```
    /// # use napchart::Napchart;
    /// let mut chart = Napchart::default();
    /// let mut lane = chart.add_lane();
    /// lane.add_element(0, 120)
    ///     .unwrap()
    ///     .text("Midnight Ritual");
    /// lane.add_element(120, 330)
    ///     .unwrap()
    ///     .text("SWS Core")
    ///     .color("blue");
    /// ```
    pub fn text<T: ToString>(&mut self, text: T) -> &mut Self {
        self.data.text = Some(text.to_string());
        self
    }
    /// &mut builder function to set the color of the element.
    /// Returns the reference for following functions.
    /// ```
    /// # use napchart::Napchart;
    /// let mut chart = Napchart::default();
    /// let mut lane = chart.add_lane();
    /// lane.add_element(0, 120)
    ///     .unwrap()
    ///     .color("grey");
    /// lane.add_element(120, 330)
    ///     .unwrap()
    ///     .color("blue")
    ///     .text("SWS Core");
    /// ```
    pub fn color<T: ToString>(&mut self, color: T) -> &mut Self {
        self.data.color = color.to_string();
        self
    }
}

impl TryFrom<Napchart> for raw::Napchart {
    type Error = ErrorKind;
    fn try_from(chart: Napchart) -> Result<raw::Napchart> {
        Ok(raw::Napchart {
            chartData: raw::ChartData {
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
                        color: l.data.color,
                        start: l.start,
                    })
                    .collect(),
                colorTags: chart
                    .color_tags
                    .into_iter()
                    .map(|(color, tag)| raw::ColorTag { tag, color })
                    .collect(),
            },
            chartid: chart.chartid.unwrap_or_default(),
            title: chart.title,
            description: chart.description,
        })
    }
}
impl TryFrom<raw::Napchart> for Napchart {
    type Error = ErrorKind;
    fn try_from(raw: raw::Napchart) -> Result<Napchart> {
        Ok(Napchart {
            chartid: Some(raw.chartid),
            title: raw.title,
            description: raw.description,
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
                            color: e.color.clone(),
                        },
                    });
                }
                vec
            },
            color_tags: {
                let mut map = HashMap::new();
                for colortag in raw.chartData.colorTags.iter() {
                    map.insert(colortag.color.clone(), colortag.tag.clone());
                }
                map
            },
        })
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }
}
