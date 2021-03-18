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

use crate::error::*;
use crate::raw;
use crate::Napchart;
use serde::Deserialize;
use std::convert::{TryFrom, TryInto};

#[derive(Deserialize)]
struct CreateResponse {
    chartid: String,
}

#[cfg(feature = "async")]
pub struct NapchartClient {
    internal: reqwest::Client,
}
#[cfg(feature = "async")]
impl NapchartClient {
    pub async fn get<'a, T: Into<&'a str>>(&self, chartid: T) -> Result<Napchart> {
        self.internal
            .get(format!(
                "https://thumb.napchart.com/api/get?chartid={}",
                chartid.into()
            ))
            .send()
            .await?
            .json::<raw::Napchart>()
            .await?
            .try_into()
    }
    pub async fn create<'a, T: Into<&'a str>>(&self, chart: &mut Napchart) -> Result<String> {
        Ok(self
            .internal
            .post("https://thumb.napchart.com/alt/api/create")
            .json(&raw::Napchart::try_from(chart.clone())?.as_uploadable())
            .send()
            .await?
            .json::<CreateResponse>()
            .await?
            .chartid)
    }
}
#[cfg(feature = "async")]
impl Default for NapchartClient {
    fn default() -> Self {
        NapchartClient {
            internal: reqwest::Client::new(),
        }
    }
}

#[cfg(feature = "blocking")]
pub mod blocking {
    use super::*;
    pub struct NapchartClient {
        internal: reqwest::blocking::Client,
    }
    impl NapchartClient {
        pub fn get<'a, T: Into<&'a str>>(&self, chartid: T) -> Result<Napchart> {
            self.internal
                .get(format!(
                    "https://thumb.napchart.com/api/get?chartid={}",
                    chartid.into()
                ))
                .send()?
                .json::<raw::Napchart>()?
                .try_into()
        }
        pub fn create(&self, chart: &mut Napchart) -> Result<String> {
            Ok(self
                .internal
                .post("https://thumb.napchart.com/alt/api/create")
                .json(&raw::Napchart::try_from(chart.clone())?.as_uploadable())
                .send()?
                .json::<CreateResponse>()?
                .chartid)
        }
    }
    impl Default for NapchartClient {
        fn default() -> Self {
            NapchartClient {
                internal: reqwest::blocking::Client::new(),
            }
        }
    }
}
