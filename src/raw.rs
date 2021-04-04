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
#![allow(non_snake_case)]

// use crate::error::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::convert::{TryFrom, TryInto};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct ChartUploadRequest {
    pub(crate) chartData: ChartSchema,
    pub(crate) title: Option<String>,
    pub(crate) description: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct ChartCreationReturn {
    pub(crate) chartDocument: ChartDocument,
    pub(crate) publicLink: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct ChartDocument {
    pub(crate) chartData: ChartSchema,
    pub(crate) chartid: String,
    pub(crate) title: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) username: String,
    pub(crate) lastUpdated: String,
    pub(crate) isSnapshot: bool,
    pub(crate) isPrivate: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct ChartSchema {
    pub(crate) elements: Vec<ChartElement>,
    pub(crate) lanes: usize,
    pub(crate) lanesConfig: HashMap<usize, LaneConfig>,
    pub(crate) shape: String,
    pub(crate) colorTags: Vec<ColorTag>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct ChartElement {
    pub(crate) start: u16,
    pub(crate) end: u16,
    pub(crate) lane: usize,
    pub(crate) text: Option<String>,
    pub(crate) color: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct ColorTag {
    pub(crate) tag: String,
    pub(crate) color: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct LaneConfig {
    pub(crate) locked: bool,
}
