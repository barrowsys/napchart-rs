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
use crate::{Napchart, RemoteNapchart};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use std::io::Write;

pub struct UploadBuilder<'c> {
    chart: &'c Napchart,
    title: Option<String>,
    description: Option<String>,
}
impl<'n> UploadBuilder<'n> {
    pub(crate) fn new(n: &'n Napchart) -> Self {
        Self {
            chart: n,
            title: None,
            description: None,
        }
    }
    pub fn title<T: AsRef<str>>(self, title: T) -> Self {
        assert!(title.as_ref().len() <= 100);
        Self {
            title: Some(title.as_ref().to_string()),
            ..self
        }
    }
    pub fn description<T: ToString>(self, description: T) -> Self {
        Self {
            description: Some(description.to_string()),
            ..self
        }
    }
    fn build(self) -> Result<raw::UploadPayload> {
        Ok(raw::UploadPayload {
            chartData: self.chart.clone().try_into()?,
            title: self.title,
            description: self.description,
        })
    }
}

#[cfg(feature = "blocking")]
/// Synchronous api client for <https://napchart.com>
pub struct BlockingClient {
    internal: reqwest::blocking::Client,
}
/// Functions using the api.napchart.com/v1 public api
impl BlockingClient {
    pub fn create_snapshot(&self, payload: UploadBuilder) -> Result<RemoteNapchart> {
        self.internal
            .post("https://api.napchart.com/v1/createSnapshot")
            .json(&payload.build()?)
            .send()?
            .json::<raw::ChartCreationReturn>()?
            .try_into()
    }
    pub fn get_chart<T: AsRef<str>>(&self, chartid: T) -> Result<RemoteNapchart> {
        self.internal
            .get(format!(
                "https://api.napchart.com/v1/getChart/{}",
                chartid.as_ref()
            ))
            .send()?
            .json::<raw::ChartCreationReturn>()?
            .try_into()
    }
}
/// Functions using the undocumented api.napchart.com api
impl BlockingClient {}
/// Functions using the old thumb.napchart.com beta api
impl BlockingClient {
    pub fn get_image<T: AsRef<str>, F: Write>(
        &self,
        chartid: T,
        dest: &mut F,
        size: (u32, u32),
        shape: Option<crate::ChartShape>,
    ) -> Result<()> {
        let mut req = self
            .internal
            .get("https://thumb.napchart.com/api/getImage")
            .query(&[
                ("chartid", chartid.as_ref()),
                ("width", &size.0.to_string()),
                ("height", &size.1.to_string()),
            ]);
        if shape.is_some() {
            req = req.query(&[("shape", "circle")]);
        }
        let resp = req.send()?.bytes()?;
        dest.write_all(&resp)?;
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
#[cfg(test)]
#[cfg(feature = "blocking")]
mod blocking_tests {
    use super::*;
    #[test]
    fn get_chart() {
        let client = BlockingClient::default();
        let rchart = client.get_chart("jex3y").unwrap();
        assert_eq!(rchart.chartid, "jex3y");
        assert_eq!(
            rchart.title,
            Some("timemachine simple test chart".to_string())
        );
        assert_eq!(rchart.description, None);
        assert_eq!(rchart.username, Some("barrow".to_string()));
        assert_eq!(rchart.is_snapshot, false);
        assert_eq!(
            rchart.public_link,
            Some("https://napchart.com/barrow/timemachine-simple-test-chart-jex3y".to_string())
        );
        let chart = rchart.chart;
        assert_eq!(chart.shape, crate::ChartShape::Circle);
        assert!(chart.color_tags.is_empty());
        assert_eq!(chart.lanes.len(), 1);
        let lane = chart.lanes.get(0).unwrap();
        assert!(!lane.locked);
        assert_eq!(lane.elements.len(), 2);
    }
    #[test]
    fn create_snapshot() {
        let client = BlockingClient::default();
        let mut lchart = Napchart::default().shape(crate::ChartShape::Circle);
        lchart
            .add_lane()
            .add_element(0, 8 * 60)
            .unwrap()
            .color(crate::ChartColor::Red);
        lchart
            .add_lane()
            .add_element(8 * 60, 16 * 60)
            .unwrap()
            .color(crate::ChartColor::Blue);
        let rchart = client
            .create_snapshot(lchart.upload().title("napchart simple test chart"))
            .unwrap();
        assert_eq!(rchart.title, Some("napchart simple test chart".to_string()));
        assert_eq!(rchart.description, None);
        assert_eq!(rchart.username, None);
        assert!(rchart.is_snapshot);
        assert_eq!(rchart.chart, lchart);
        let chart = rchart.chart;
        assert_eq!(chart.shape, crate::ChartShape::Circle);
        assert!(chart.color_tags.is_empty());
        assert_eq!(chart.lanes.len(), 2);
        let lane1 = chart.lanes.get(0).unwrap();
        let lane2 = chart.lanes.get(1).unwrap();
        assert!(!lane1.locked);
        assert!(!lane2.locked);
        assert_eq!(lane1.elements.len(), 1);
        assert_eq!(lane2.elements.len(), 1);
        let elem1 = lane1.elements_iter().next().unwrap();
        let elem2 = lane2.elements_iter().next().unwrap();
        assert_eq!(elem1.start, 0);
        assert_eq!(elem1.end, 8 * 60);
        assert_eq!(elem2.start, 8 * 60);
        assert_eq!(elem2.end, 16 * 60);
        let elemd1 = &elem1.data;
        let elemd2 = &elem2.data;
        assert_eq!(elemd1.text, None);
        assert_eq!(elemd2.text, None);
        assert_eq!(elemd1.color, crate::ChartColor::Red);
        assert_eq!(elemd2.color, crate::ChartColor::Blue);
    }
}

//TODO: replace asyncclient
//#[cfg(feature = "async")]
///// Asynchronous api client for <https://napchart.com>
//pub struct AsyncClient {
//    internal: reqwest::Client,
//}
//#[cfg(feature = "async")]
//impl AsyncClient {
//    /// Asynchronously downloads the napchart with the given id from napchart.com
//    ///
//    /// Uses the <https://thumb.napchart.com/api/get> endpoint
//    pub async fn get<T: AsRef<str>>(&self, chartid: T) -> Result<Napchart> {
//        self.internal
//            .get("https://thumb.napchart.com/api/get")
//            .query(&[("chartid", chartid.as_ref())])
//            .send()
//            .await?
//            .json::<raw::Napchart>()
//            .await?
//            .try_into()
//    }
//    /// Asynchronously creates a napchart on napchart.com and returns its id
//    ///
//    /// Uses the <https://thumb.napchart.com/alt/api/create> endpoint
//    pub async fn create(&self, chart: &Napchart) -> Result<String> {
//        Ok(self
//            .internal
//            .post("https://thumb.napchart.com/alt/api/create")
//            .json(&raw::Napchart::try_from(chart.clone())?.as_uploadable())
//            .send()
//            .await?
//            .json::<CreateResponse>()
//            .await?
//            .chartid)
//    }
//    /// Asynchronously creates a napchart on napchart.com and sets the chartid of the napchart
//    /// struct to the assigned id
//    ///
//    /// Uses the <https://thumb.napchart.com/alt/api/create> endpoint
//    pub async fn create_new(&self, chart: &mut Napchart) -> Result<()> {
//        chart.chartid = Some(self.create(chart).await?);
//        Ok(())
//    }
//    /// Asynchronously downloads an image representation of a napchart from napchart.com and saves
//    /// it to the given &mut File
//    ///
//    /// Uses the <https://thumb.napchart.com/api/getImage> endpoint
//    pub async fn get_image<T: AsRef<str>, F: Write>(
//        &self,
//        chartid: T,
//        dest: &mut F,
//        size: (u32, u32),
//        shape: Option<crate::ChartShape>,
//    ) -> Result<()> {
//        let mut req = self
//            .internal
//            .get("https://thumb.napchart.com/api/getImage")
//            .query(&[
//                ("chartid", chartid.as_ref()),
//                ("width", &size.0.to_string()),
//                ("height", &size.1.to_string()),
//            ]);
//        if shape.is_some() {
//            req = req.query(&[("shape", "circle")]);
//        }
//        let resp = req.send().await?.bytes().await?;
//        dest.write_all(&resp)?;
//        Ok(())
//    }
//}
// #[cfg(feature = "async")]
// impl Default for AsyncClient {
//     fn default() -> Self {
//         Self {
//             internal: reqwest::Client::new(),
//         }
//     }
// }

//TODO: replace all blockingclient functions
//#[cfg(feature = "blocking")]
///// Synchronous api client for <https://napchart.com>
//pub struct BlockingClientOld {
//    internal: reqwest::blocking::Client,
//}
//impl BlockingClientOld {
//    /// Synchronously downloads the napchart with the given id from napchart.com
//    ///
//    /// Uses the <https://thumb.napchart.com/api/get> endpoint
//    pub fn get<T: AsRef<str>>(&self, chartid: T) -> Result<Napchart> {
//        self.internal
//            .get("https://thumb.napchart.com/api/get")
//            .query(&[("chartid", chartid.as_ref())])
//            .send()?
//            .json::<raw::Napchart>()?
//            .try_into()
//    }
//    /// Synchronously creates a napchart on napchart.com and returns its id
//    ///
//    /// Uses the <https://thumb.napchart.com/alt/api/create> endpoint
//    pub fn create(&self, chart: &Napchart) -> Result<String> {
//        Ok(self
//            .internal
//            .post("https://thumb.napchart.com/alt/api/create")
//            .json(&raw::Napchart::try_from(chart.clone())?.as_uploadable())
//            .send()?
//            .json::<CreateResponse>()?
//            .chartid)
//    }
//    //TODO: Replace api.create_new
//    ///// Synchronously creates a napchart on napchart.com and sets the chartid of the napchart
//    ///// struct to the assigned id
//    /////
//    ///// Uses the <https://thumb.napchart.com/alt/api/create> endpoint
//    //pub fn create_new(&self, chart: &mut Napchart) -> Result<()> {
//    //    chart.chartid = Some(self.create(chart)?);
//    //    Ok(())
//    //}
//    /// Synchronously downloads an image representation of a napchart from napchart.com and saves
//    /// it to the given &mut File
//    ///
//    /// Uses the <https://thumb.napchart.com/api/getImage> endpoint
//    pub fn get_image<T: AsRef<str>, F: Write>(
//        &self,
//        chartid: T,
//        dest: &mut F,
//        size: (u32, u32),
//        shape: Option<crate::ChartShape>,
//    ) -> Result<()> {
//        let mut req = self
//            .internal
//            .get("https://thumb.napchart.com/api/getImage")
//            .query(&[
//                ("chartid", chartid.as_ref()),
//                ("width", &size.0.to_string()),
//                ("height", &size.1.to_string()),
//            ]);
//        if shape.is_some() {
//            req = req.query(&[("shape", "circle")]);
//        }
//        let resp = req.send()?.bytes()?;
//        dest.write_all(&resp)?;
//        Ok(())
//    }
//}

#[cfg(test)]
#[cfg(feature = "async")]
mod tests {
    use super::*;
    use tokio::task::spawn_blocking;
    // #[tokio::test]
    // async fn get_eq() {
    //     let bres = spawn_blocking(move || {
    //         let bclient = BlockingClient::default();
    //         bclient.get("bwul9").unwrap()
    //     })
    //     .await
    //     .unwrap();
    //     let aclient = AsyncClient::default();
    //     let ares = aclient.get("bwul9").await.unwrap();
    //     assert_eq!(ares, bres);
    // }
    // #[tokio::test]
    // async fn create_eq() {
    //     let mut achart = Napchart::default().title("test").description("");
    //     let lane = achart.add_lane();
    //     lane.add_element(1, 72).unwrap();
    //     lane.add_element(470, 472).unwrap();
    //     lane.add_element(870, 873).unwrap();
    //     lane.add_element(1270, 1274).unwrap();
    //     let bchart = achart.clone();
    //     let bres = spawn_blocking(move || {
    //         let bclient = BlockingClient::default();
    //         let bid = bclient.create(&bchart).unwrap();
    //         bclient.get(&bid).unwrap()
    //     })
    //     .await
    //     .unwrap();
    //     let aclient = AsyncClient::default();
    //     let aid = aclient.create(&achart).await.unwrap();
    //     let ares = aclient.get(&aid).await.unwrap();
    //     assert!(ares.chart_eq(&bres));
    // }
    // #[tokio::test]
    // async fn get_image_eq() {
    //     use std::io::Cursor;
    //     let bfile = spawn_blocking(move || {
    //         let bclient = BlockingClient::default();
    //         let mut bfile = Cursor::new(Vec::new());
    //         bclient
    //             .get_image("bwul9", &mut bfile, (600, 600), None)
    //             .unwrap();
    //         bfile
    //     })
    //     .await
    //     .unwrap();
    //     let aclient = AsyncClient::default();
    //     let mut afile = Cursor::new(Vec::new());
    //     aclient
    //         .get_image("bwul9", &mut afile, (600, 600), None)
    //         .await
    //         .unwrap();
    //     assert_eq!(afile, bfile);
    // }
}
