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

//! Structs in this module directly map to the json representation of napcharts
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct Napchart {
    pub(crate) chartData: ChartData,
    pub(crate) chartid: String,
    pub(crate) title: Option<String>,
    pub(crate) description: Option<String>,
}
#[derive(Serialize, PartialEq, Debug)]
pub(crate) struct UploadableChart<'a> {
    pub(crate) chartData: &'a ChartData,
    pub(crate) metaInfo: UploadableMetadata<'a>,
}
#[derive(Serialize, PartialEq, Debug)]
pub(crate) struct UploadableMetadata<'a> {
    pub(crate) title: &'a Option<String>,
    pub(crate) description: &'a Option<String>,
}
impl Napchart {
    pub(crate) fn as_uploadable(&'_ self) -> UploadableChart<'_> {
        UploadableChart {
            chartData: &self.chartData,
            metaInfo: UploadableMetadata {
                title: &self.title,
                description: &self.description,
            },
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct ChartData {
    pub(crate) lanes: usize,
    pub(crate) shape: String,
    pub(crate) elements: Vec<ChartElement>,
    pub(crate) colorTags: Vec<ColorTag>,
    pub(crate) lanesConfig: HashMap<usize, LaneConfig>,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct ChartElement {
    pub(crate) end: u16,
    pub(crate) lane: usize,
    pub(crate) text: String,
    pub(crate) color: String,
    pub(crate) start: u16,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct LaneConfig {
    pub(crate) locked: bool,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct ColorTag {
    pub(crate) tag: String,
    pub(crate) color: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::iter::FromIterator;
    #[test]
    fn test_json_1() {
        let chart_str = r#"{"chartData":{"lanes":1,"shape":"circle","elements":[{"end":480,"lane":0,"text":"","color":"red","start":0}],"colorTags":[],"lanesConfig":{"1":{"locked":false}}},"chartid":"cse2j","title":"small test chart","description":null}"#;
        let chart = Napchart {
            chartid: "cse2j".to_owned(),
            title: Some("small test chart".to_owned()),
            description: None,
            chartData: ChartData {
                lanes: 1,
                shape: "circle".to_owned(),
                elements: vec![ChartElement {
                    start: 0,
                    end: 480,
                    lane: 0,
                    text: "".to_owned(),
                    color: "red".to_owned(),
                }],
                colorTags: vec![],
                lanesConfig: HashMap::from_iter(vec![(1, LaneConfig { locked: false })]),
            },
        };
        let chart2: Napchart = serde_json::from_str(chart_str).unwrap();
        assert_eq!(chart, chart2);
    }
    #[test]
    fn test_json_2() {
        let chart_str = r#"{"chartData":{"lanes":2,"shape":"circle","elements":[{"end":30,"lane":0,"text":"","color":"brown","start":1380},{"end":1110,"lane":0,"text":"hello","color":"brown","start":1020},{"end":390,"lane":0,"text":"","color":"yellow","start":300},{"end":600,"lane":0,"text":"","color":"red","start":580},{"end":820,"lane":0,"text":"","color":"red","start":800},{"end":1080,"lane":1,"text":"","color":"pink","start":1050},{"end":720,"lane":1,"text":"","color":"pink","start":690},{"end":360,"lane":1,"text":"","color":"pink","start":330},{"end":1440,"lane":1,"text":"","color":"gray","start":1410}],"colorTags":[{"tag":"SWS cores","color":"brown"}],"lanesConfig":{"0":{"locked":true},"1":{"locked":false}}},"chartid":"bwul9","title":"big test chart","description":"description1"}"#;
        let chart = Napchart {
            chartid: "bwul9".to_owned(),
            title: Some("big test chart".to_owned()),
            description: Some("description1".to_owned()),
            chartData: ChartData {
                lanes: 2,
                shape: "circle".to_owned(),
                elements: vec![
                    ChartElement {
                        end: 30,
                        lane: 0,
                        text: "".to_owned(),
                        color: "brown".to_owned(),
                        start: 1380,
                    },
                    ChartElement {
                        end: 1110,
                        lane: 0,
                        text: "hello".to_owned(),
                        color: "brown".to_owned(),
                        start: 1020,
                    },
                    ChartElement {
                        end: 390,
                        lane: 0,
                        text: "".to_owned(),
                        color: "yellow".to_owned(),
                        start: 300,
                    },
                    ChartElement {
                        end: 600,
                        lane: 0,
                        text: "".to_owned(),
                        color: "red".to_owned(),
                        start: 580,
                    },
                    ChartElement {
                        end: 820,
                        lane: 0,
                        text: "".to_owned(),
                        color: "red".to_owned(),
                        start: 800,
                    },
                    ChartElement {
                        end: 1080,
                        lane: 1,
                        text: "".to_owned(),
                        color: "pink".to_owned(),
                        start: 1050,
                    },
                    ChartElement {
                        end: 720,
                        lane: 1,
                        text: "".to_owned(),
                        color: "pink".to_owned(),
                        start: 690,
                    },
                    ChartElement {
                        end: 360,
                        lane: 1,
                        text: "".to_owned(),
                        color: "pink".to_owned(),
                        start: 330,
                    },
                    ChartElement {
                        end: 1440,
                        lane: 1,
                        text: "".to_owned(),
                        color: "gray".to_owned(),
                        start: 1410,
                    },
                ],
                colorTags: vec![ColorTag {
                    tag: "SWS cores".to_owned(),
                    color: "brown".to_owned(),
                }],
                lanesConfig: HashMap::from_iter(vec![
                    (0, LaneConfig { locked: true }),
                    (1, LaneConfig { locked: false }),
                ]),
            },
        };
        let chart2: Napchart = serde_json::from_str(chart_str).unwrap();
        assert_eq!(chart, chart2);
    }
}
