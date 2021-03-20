# napchart-rs

[crates.io](https://crates.io/crates/napchart/)
[docs.rs](https://docs.rs/napchart)

a rust interface to the https://napchart.com API alpha. pretty barebones right now, as is the api.

# Usage

currently, the only thing implemented is downloading and parsing the /get endpoint with napchart::Napchart::get_from_server(url: &str)
