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

use napchart::blocking;

fn main() {
    let mut small_chart = blocking::get("cse2j").unwrap();
    println!("small chart: {:#?}", small_chart);
    let big_chart = blocking::get("bwul9").unwrap();
    println!("big chart: {:#?}", big_chart);
    let mut new_chart = napchart::Napchart::blank().title("test").lanes(1).description("");
    let mut lane = new_chart.get_lane_mut(0).unwrap();
    lane.add_element(480, 500).unwrap();
    lane.add_element(470, 471).unwrap();
    lane.add_element(480, 500).unwrap_err();
    blocking::create(&mut new_chart).unwrap();
    // blocking::create(&mut small_chart).unwrap();
}
