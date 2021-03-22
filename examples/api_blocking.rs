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

use napchart::api::BlockingClient;
use std::fs::File;

fn main() {
    let client = BlockingClient::default();
    let small_chart = client.get("cse2j").unwrap();
    println!("small chart: {:#?}", small_chart);
    let big_chart = client.get("bwul9").unwrap();
    println!("big chart: {:#?}", big_chart);
    let mut new_chart = napchart::Napchart::default()
        .title("test")
        .lanes(1)
        .description("");
    let lane = new_chart.get_lane_mut(0).unwrap();
    lane.add_element(1, 72).unwrap();
    lane.add_element(470, 472).unwrap();
    lane.add_element(870, 873).unwrap();
    lane.add_element(1270, 1274).unwrap();
    let chartid = client.create(&new_chart).unwrap();
    println!("https://napchart.com/{}", chartid);
    let mut file = File::create("/tmp/napchart_3tbkt.png").unwrap();
    client.get_image("3tbkt", &mut file, (600, 600), None).unwrap();
}
