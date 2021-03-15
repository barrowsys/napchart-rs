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

mod raw;

pub mod error;
use error::*;

#[derive(PartialEq, Debug)]
pub struct Napchart {
    pub chartid: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub shape: ChartShape,
    pub lanes: Vec<ChartLane>,
    pub color_tags: HashMap<String, String>,
}
#[derive(PartialEq, Debug)]
pub enum ChartShape {
    Circle,
    Wide,
    Line,
}
#[derive(PartialEq, Debug)]
pub struct ChartLane {
    pub locked: bool,
    pub elements: Vec<ChartElement>,
}
#[derive(PartialEq, Debug)]
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
impl TryFrom<raw::Napchart> for Napchart {
    type Error = ErrorKind;
    fn try_from(raw: raw::Napchart) -> Result<Self> {
        Ok(Self {
            chartid: raw.chartid,
            title: raw.title,
            description: raw.description,
            shape: match raw.chartData.shape.as_str() {
                "circle" => ChartShape::Circle,
                "wide" => ChartShape::Wide,
                "line" => ChartShape::Line,
                _ => Err(ErrorKind::InvalidChartShape(raw.chartData.shape.clone()))?,
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
                        }
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
impl Napchart {
    pub fn get_from_server<'a, T: Into<&'a str>>(chartid: T) -> Result<Self> {
        let r: raw::Napchart = reqwest::blocking::get(format!(
            "https://thumb.napchart.com/api/get?chartid={}",
            chartid.into()
        ))?
        .json()?;
        r.try_into()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::iter::FromIterator;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
