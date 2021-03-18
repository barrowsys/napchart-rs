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

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::iter::{repeat, once};
use std::string::ToString;

mod raw;

pub mod api;

pub mod error;
use error::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Napchart {
    pub chartid: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub shape: ChartShape,
    pub lanes: Vec<ChartLane>,
    pub color_tags: HashMap<String, String>,
}
impl Napchart {
    pub fn blank() -> Self {
        Self {
            chartid: None,
            title: None,
            description: None,
            shape: ChartShape::Circle,
            lanes: Vec::new(),
            color_tags: HashMap::new(),
        }
    }
    pub fn title<'a, T: Into<&'a str>>(self, title: T) -> Self {
        Self {
            title: Some(title.into().to_string()),
            ..self
        }
    }
    pub fn description<'a, T: Into<&'a str>>(self, description: T) -> Self {
        Self {
            description: Some(description.into().to_string()),
            ..self
        }
    }
    pub fn shape(self, shape: ChartShape) -> Self {
        Self { shape, ..self }
    }
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
    pub fn new_lane(&mut self) {
        self.lanes.push(ChartLane {
            locked: false,
            elements: Vec::new(),
        });
    }
    pub fn get_lane_mut(&mut self, i: usize) -> Option<&mut ChartLane> {
        self.lanes.get_mut(i)
    }
}
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
#[derive(PartialEq, Debug, Clone)]
pub struct ChartLane {
    pub locked: bool,
    elements: Vec<ChartElement>,
}
impl ChartLane {
    pub fn lock(&mut self) {
        self.locked = true;
    }
    pub fn unlock(&mut self) {
        self.locked = false;
    }
    pub fn sort(&mut self) {
        self.elements.sort_unstable_by_key(|e| e.end);
    }
    pub fn clear(&mut self) {
        self.elements.clear();
    }
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
            start: start,
            end: end,
            data: ElementData {
                text: None,
                color: String::from("red"),
            }
        });
        Ok(self.elements.last_mut().unwrap())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ChartElement {
    pub start: u16,
    pub end: u16,
    pub data: ElementData,
}
#[derive(PartialEq, Debug, Clone)]
pub struct ElementData {
    pub text: Option<String>,
    pub color: String,
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
