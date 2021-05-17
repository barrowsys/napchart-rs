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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChartUploadRequest {
    pub(crate) chart_data: ChartSchema,
    pub(crate) title: Option<String>,
    pub(crate) description: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChartCreationReturn {
    pub(crate) chart_document: ChartDocument,
    pub(crate) public_link: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChartDocument {
    pub(crate) chart_data: ChartSchema,
    pub(crate) chartid: String,
    pub(crate) title: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) username: String,
    pub(crate) last_updated: String,
    pub(crate) is_snapshot: bool,
    pub(crate) is_private: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChartSchema {
    pub(crate) elements: Vec<ChartElement>,
    pub(crate) lanes: usize,
    pub(crate) lanes_config: HashMap<usize, LaneConfig>,
    pub(crate) shape: String,
    pub(crate) color_tags: Vec<ColorTag>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChartElement {
    pub(crate) start: u16,
    pub(crate) end: u16,
    pub(crate) lane: usize,
    pub(crate) text: Option<String>,
    pub(crate) color: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ColorTag {
    pub(crate) tag: String,
    pub(crate) color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) color_value: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LaneConfig {
    pub(crate) locked: bool,
}
