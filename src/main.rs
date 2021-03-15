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

use napchart::Napchart;

fn main() {
    let small_chart = Napchart::get_from_server("cse2j").unwrap();
    println!("small chart: {:#?}", small_chart);
    let big_chart = Napchart::get_from_server("bwul9").unwrap();
    println!("big chart: {:#?}", big_chart);
}
