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
//TODO: More docs
//! Api clients for napchart.com

use crate::error::*;
use crate::raw;
use crate::{Napchart, RemoteNapchart};
use std::convert::TryInto;

/// Builder struct for adding submittable metadata to a &Napchart
/// ```
/// # use napchart::*;
/// # use napchart::api::mock::BlockingClient;
/// let client = BlockingClient::default();
/// let chart = Napchart::default();
/// let upload = chart.upload();
/// assert!(client.create_snapshot(upload).is_ok());
/// ```
pub struct UploadBuilder<'c> {
    chart: &'c Napchart,
    title: Option<String>,
    description: Option<String>,
}
impl<'c> UploadBuilder<'c> {
    pub(crate) fn new(n: &'c Napchart) -> Self {
        Self {
            chart: n,
            title: None,
            description: None,
        }
    }
    /// Builder function to set the title of a napchart on upload.
    /// ```
    /// # use napchart::*;
    /// # use napchart::api::mock::BlockingClient;
    /// let client = BlockingClient::default();
    /// let chart = Napchart::default();
    /// let upload = chart.upload().title("My Cool Chart");
    /// assert!(client.create_snapshot(upload).is_ok());
    /// ```
    pub fn title<T: AsRef<str>>(self, title: T) -> Self {
        assert!(title.as_ref().len() <= 100);
        Self {
            title: Some(title.as_ref().to_string()),
            ..self
        }
    }
    /// Builder function to set the description of a napchart on upload.
    /// ```
    /// # use napchart::*;
    /// # use napchart::api::mock::BlockingClient;
    /// let client = BlockingClient::default();
    /// let chart = Napchart::default();
    /// let upload = chart.upload().description("This is my super cool chart. Please appreciate it");
    /// assert!(client.create_snapshot(upload).is_ok());
    /// ```
    pub fn description<T: AsRef<str>>(self, description: T) -> Self {
        Self {
            description: Some(description.as_ref().to_string()),
            ..self
        }
    }
    fn build(self) -> Result<raw::ChartUploadRequest> {
        Ok(raw::ChartUploadRequest {
            chart_data: self.chart.clone().try_into()?,
            title: self.title,
            description: self.description,
        })
    }
}

#[doc(hidden)]
pub mod mock {
    #[derive(Default)]
    pub struct BlockingClient {}
    impl BlockingClient {
        #[allow(dead_code)]
        pub fn create_snapshot(
            &self,
            payload: super::UploadBuilder,
        ) -> Result<(), crate::ErrorKind> {
            let _ = payload.build()?;
            Ok(())
        }
    }
}

/// Blocking <https://napchart.com> API client.
/// Uses reqwest::blocking::Client internally.
#[derive(Default)]
pub struct BlockingClient {
    internal: reqwest::blocking::Client,
}
impl BlockingClient {
    /// Uploads a napchart (prepared as an UploadBuilder) to <https://napchart.com> and returns a
    /// RemoteNapchart
    pub fn create_snapshot(&self, payload: UploadBuilder) -> Result<RemoteNapchart> {
        self.internal
            .post("https://api.napchart.com/v1/createSnapshot")
            .json(&payload.build()?)
            .send()?
            .json::<raw::ChartCreationReturn>()?
            .try_into()
    }
    /// Downloads a napchart with the given chartid from <https://napchart.com> and returns it as a
    /// RemoteNapchart
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

/// Async <https://napchart.com> API client.
/// Uses reqwest::Client internally.
#[derive(Default)]
pub struct AsyncClient {
    internal: reqwest::Client,
}
impl AsyncClient {
    /// Uploads a napchart (prepared as an UploadBuilder) to <https://napchart.com> and returns a
    /// RemoteNapchart
    pub async fn create_snapshot(&self, payload: UploadBuilder<'_>) -> Result<RemoteNapchart> {
        self.internal
            .post("https://api.napchart.com/v1/createSnapshot")
            .json(&payload.build()?)
            .send()
            .await?
            .json::<raw::ChartCreationReturn>()
            .await?
            .try_into()
    }
    /// Downloads a napchart with the given chartid from <https://napchart.com> and returns it as a
    /// RemoteNapchart
    pub async fn get_chart<T: AsRef<str>>(&self, chartid: T) -> Result<RemoteNapchart> {
        self.internal
            .get(format!(
                "https://api.napchart.com/v1/getChart/{}",
                chartid.as_ref()
            ))
            .send()
            .await?
            .json::<raw::ChartCreationReturn>()
            .await?
            .try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::task::spawn_blocking;
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
        assert!(!rchart.is_snapshot);
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
    fn get_custom_colors() {
        let client = BlockingClient::default();
        let rchart = client.get_chart("oo81DYL84").unwrap();
        let chart = rchart.chart;
        assert_eq!(chart.color_tags.len(), 12);
        let custom_0 = chart.color_tags.get(&crate::ChartColor::Custom0).unwrap();
        println!("{:#?}", chart);
        assert_eq!(custom_0, "custom_0 tag");
        assert_eq!(
            chart.custom_colors[0],
            Some(colorsys::Rgb::from((0x50, 0x81, 0x4a)))
        );
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
        let elem1 = lane1.elems_iter().next().unwrap();
        let elem2 = lane2.elems_iter().next().unwrap();
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
    #[tokio::test]
    async fn get_chart_eq() {
        let bres = spawn_blocking(move || {
            let bclient = BlockingClient::default();
            bclient.get_chart("bwul9").unwrap()
        })
        .await
        .unwrap();
        let aclient = AsyncClient::default();
        let ares = aclient.get_chart("bwul9").await.unwrap();
        assert!(ares.semantic_eq(&bres));
    }
    #[tokio::test]
    async fn create_snapshot_eq() {
        let mut achart = Napchart::default();
        let lane = achart.add_lane();
        lane.add_element(1, 72).unwrap();
        lane.add_element(470, 472).unwrap();
        lane.add_element(870, 873).unwrap();
        lane.add_element(1270, 1274).unwrap();
        let bchart = achart.clone();
        let aup = achart
            .upload()
            .title("create_snapshot equivalence test")
            .description("");
        let (brchart1, brchart2) = spawn_blocking(move || {
            let bup = bchart
                .upload()
                .title("create_snapshot equivalence test")
                .description("");
            let bclient = BlockingClient::default();
            let brchart1 = bclient.create_snapshot(bup).unwrap();
            let brchart2 = bclient.get_chart(&brchart1.chartid).unwrap();
            (brchart1, brchart2)
        })
        .await
        .unwrap();
        let aclient = AsyncClient::default();
        let archart1 = aclient.create_snapshot(aup).await.unwrap();
        let archart2 = aclient.get_chart(&archart1.chartid).await.unwrap();
        assert!(archart1.semantic_eq(&brchart1));
        assert!(archart2.semantic_eq(&brchart2));
        // assert_eq!(ares, bres);
    }
}
