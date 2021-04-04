# napchart-rs

[![GitHub last commit](https://img.shields.io/github/last-commit/barrowsys/napchart-rs)](https://github.com/barrowsys/napchart-rs)
[![Crates.io](https://img.shields.io/crates/v/napchart)](https://crates.io/crates/napchart/)
[![Docs.rs](https://docs.rs/napchart/badge.svg)](https://docs.rs/napchart)

A strongly-typed rust interface to the <https://napchart.com> API.

The public napchart api is pretty barebones right now, but this will let you use it!

## Usage

Add to your Cargo.toml:
```text
[dependencies]
napchart = "0.1.5"
```

## Examples

### Creating a new napchart from scratch
Example: <https://napchart.com/snapshot/O6kunUfuL>
```
use napchart::prelude::*;

let mut chart = Napchart::default()
    .shape(ChartShape::Circle)
    .lanes(1);
let first_lane = chart.get_lane_mut(0).unwrap();
first_lane.add_element(0, 60).unwrap()
    .text("Hour One");
first_lane.add_element(180, 240).unwrap()
    .text("Hour Four");
let second_lane = chart.add_lane();
second_lane.add_element(0, 120).unwrap()
    .color(ChartColor::Blue);
second_lane.add_element(120, 240).unwrap()
    .color(ChartColor::Green)
    .text("Cool green time");
```

### Downloading a napchart
Example Chart: <https://napchart.com/3tbkt>
```
use napchart::api::BlockingClient;

let client = BlockingClient::default();
let rchart = client.get_chart("3tbkt").unwrap();
assert_eq!(rchart.chartid, String::from("3tbkt"));
assert_eq!(rchart.title, Some(String::from("State test chart")));
assert_eq!(rchart.chart.shape, napchart::ChartShape::Circle);
assert_eq!(rchart.chart.lanes_len(), 1);
```

### Uploading a napchart as a snapshot
Example Output: <https://napchart.com/snapshot/TpCfggr4i>
```
use napchart::prelude::*;
use napchart::api::BlockingClient;

let client = BlockingClient::default();
let mut chart = Napchart::default();
let lane = chart.add_lane();
lane.add_element(420, 1260)
    .unwrap()
    .text("Nighttime")
    .color(ChartColor::Gray);
let upload_builder = chart.upload()
    .title("readme doctest")
    .description("https://crates.io/crates/napchart");
let remote_chart = client.create_snapshot(upload_builder).unwrap();
assert!(!remote_chart.chartid.is_empty());
```
