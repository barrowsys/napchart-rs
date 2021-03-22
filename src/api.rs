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

//! The actual api clients
//!
//! if the "async" feature is enabled, then you can use napchart::api::AsyncClient  
//! if the "blocking" feature is enabled, then you can use napchart::api::BlockingClient

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
/// Asynchronous api client for <https://napchart.com>
pub struct AsyncClient {
    internal: reqwest::Client,
}
#[cfg(feature = "async")]
impl AsyncClient {
    /// Asynchronously downloads the napchart with the given id from napchart.com
    ///
    /// Uses the <https://thumb.napchart.com/api/get> endpoint
    pub async fn get<'a, T: Into<&'a str>>(&self, chartid: T) -> Result<Napchart> {
        self.internal
            .get("https://thumb.napchart.com/api/get")
            .query(&[("chartid", chartid.into())])
            .send()
            .await?
            .json::<raw::Napchart>()
            .await?
            .try_into()
    }
    /// Asynchronously creates a napchart on napchart.com and returns its id
    ///
    /// Uses the <https://thumb.napchart.com/alt/api/create> endpoint
    pub async fn create(&self, chart: &Napchart) -> Result<String> {
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
    /// Asynchronously creates a napchart on napchart.com and sets the chartid of the napchart
    /// struct to the assigned id
    ///
    /// Uses the <https://thumb.napchart.com/alt/api/create> endpoint
    pub async fn create_new(&self, chart: &mut Napchart) -> Result<()> {
        chart.chartid = Some(self.create(chart).await?);
        Ok(())
    }
}
#[cfg(feature = "async")]
impl Default for AsyncClient {
    fn default() -> Self {
        Self {
            internal: reqwest::Client::new(),
        }
    }
}

#[cfg(feature = "blocking")]
/// Synchronous api client for <https://napchart.com>
pub struct BlockingClient {
    internal: reqwest::blocking::Client,
}
impl BlockingClient {
    /// Synchronously downloads the napchart with the given id from napchart.com
    ///
    /// Uses the <https://thumb.napchart.com/api/get> endpoint
    pub fn get<'a, T: Into<&'a str>>(&self, chartid: T) -> Result<Napchart> {
        self.internal
            .get("https://thumb.napchart.com/api/get")
            .query(&[("chartid", chartid.into())])
            .send()?
            .json::<raw::Napchart>()?
            .try_into()
    }
    /// Synchronously creates a napchart on napchart.com and returns its id
    ///
    /// Uses the <https://thumb.napchart.com/alt/api/create> endpoint
    pub fn create(&self, chart: &Napchart) -> Result<String> {
        Ok(self
            .internal
            .post("https://thumb.napchart.com/alt/api/create")
            .json(&raw::Napchart::try_from(chart.clone())?.as_uploadable())
            .send()?
            .json::<CreateResponse>()?
            .chartid)
    }
    /// Synchronously creates a napchart on napchart.com and sets the chartid of the napchart
    /// struct to the assigned id
    ///
    /// Uses the <https://thumb.napchart.com/alt/api/create> endpoint
    pub fn create_new(&self, chart: &mut Napchart) -> Result<()> {
        chart.chartid = Some(self.create(chart)?);
        Ok(())
    }
}
impl Default for BlockingClient {
    fn default() -> Self {
        BlockingClient {
            internal: reqwest::blocking::Client::new(),
        }
    }
}
