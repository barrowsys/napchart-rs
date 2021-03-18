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

use napchart::api::NapchartClient;

#[tokio::main]
async fn main() {
    let client = NapchartClient::new();
    let small_chart = client.get("cse2j");
    let big_chart = client.get("bwul9");
    println!("small chart: {:#?}", small_chart.await.unwrap());
    println!("big chart: {:#?}", big_chart.await.unwrap());
}
