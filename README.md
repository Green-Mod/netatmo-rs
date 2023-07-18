# netatmo-rs

This repo is the fork of [`netatmo-rs`](https://github.com/lukaspustina/netatmo-rs), which is a simple [Rust](https://rust-lang.org) library to talk to [Netatmo's API](https://dev.netatmo.com/resources/technical/introduction).

## So why the fork?

The original library is not maintained anymore, and I needed to add some features to it. Also I revamped the code to use the latest Rust features and libraries.

This fork has the following changes:
1. Uses `thiserror` crate for error handling.
2. Uses `reqwest`'s async client for making requests.
3. Drops the support to [Client credentials grant type](https://dev.netatmo.com/apidocumentation/oauth#client-credential), which is not supported by Netatmo anymore.

This library assumes that the user already has an Access Token for the Netatmo API. If you don't have one, you can get it by following the steps [here](https://dev.netatmo.com/apidocumentation/oauth#authorization-code).

## Development

### Run Examples

```bash
NETATMO_ACCESS_TOKEN=xxxx NETATMO_DEVICE_ID=xxxx cargo run --example get_station_data
```

## Todos

1. Semantic transformation of results -> use enums, timezone etc instead of Strings and int values.
