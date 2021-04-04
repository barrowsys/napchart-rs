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

#[allow(warnings)]
fn main() {
    let client = BlockingClient::default();
    let mut new_chart = napchart::Napchart::default().lanes(1);
    let lane = new_chart.get_lane_mut(0).unwrap();
    lane.add_element(1, 72).unwrap();
    lane.add_element(470, 472).unwrap();
    lane.add_element(870, 873).unwrap();
    lane.add_element(1270, 1274).unwrap();
    // println!(
    //     "{:?}",
    //     client.create_snapshot(&new_chart, "title", "").unwrap()
    // );
}
// {
//     "chartDocument": {
//         "chartData": {
//             "lanes":1,
//             "shape":"circle",
//             "elements": [
//                 {"end":72,"lane":0,"text":"","color":"red","start":1},{"end":472,"lane":0,"text":"","color":"red","start":470},{"end":873,"lane":0,"text":"","color":"red","start":870},{"end":1274,"lane":0,"text":"","color":"red","start":1270}
//             ],
//             "colorTags":[],
//             "lanesConfig": {"0":{"locked":false}}
//         },
//         "chartid": "Ytce0GcXY",
//         "title":"test",
//         "description":"",
//         "username":"anonymous",
//         "lastUpdated":"2021-03-29T20:13:46.582Z",
//         "isSnapshot":true
//     },
//     "publicLink": "https://napchart.com/snapshot/Ytce0GcXY"
// }
