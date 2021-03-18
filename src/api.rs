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
use std::convert::TryInto;

#[derive(Deserialize)]
struct CreateResponse {
    chartid: String,
}

#[cfg(feature = "blocking")]
pub mod blocking {
    use crate::error::*;
    use crate::raw;
    use crate::Napchart;
    use std::convert::TryInto;
    pub fn get<'a, T: Into<&'a str>>(chartid: T) -> Result<Napchart> {
        let r: raw::Napchart = reqwest::blocking::get(format!(
            "https://thumb.napchart.com/api/get?chartid={}",
            chartid.into()
        ))?
        .json()?;
        r.try_into()
    }
    pub fn create(chart: &mut Napchart) -> Result<String> {
        let raw: raw::Napchart = chart.clone().try_into()?;
        let raw = raw.as_uploadable();
        println!("{:#?}", raw);
        let client = reqwest::blocking::Client::new();
        let res = serde_json::to_string_pretty(&raw)?;
        println!("{}", res);
        let res = client.post("https://thumb.napchart.com/alt/api/create").json(&raw).send()?;
        println!("{:?}", res.status());
        let res: super::CreateResponse = res.json()?;
        Ok(res.chartid.to_string())
    }
}
#[cfg(feature = "async")]
pub async fn get<'a, T: Into<&'a str>>(chartid: T) -> Result<Napchart> {
    let r: raw::Napchart = reqwest::get(format!(
        "https://thumb.napchart.com/api/get?chartid={}",
        chartid.into()
    ))
    .await?
    .json()
    .await?;
    r.try_into()
}
#[cfg(feature = "async")]
pub async fn create<'a, T: Into<&'a str>>(chart: &mut Napchart) -> Result<String> {
    Err(ErrorKind::NotImplemented)
}

