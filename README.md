# 50 Projects in Rust

<img height="200" width="200" align=center src="https://cdn.simpleicons.org/rust/ce412b">

This is a collection of 50 rust projects based on the Youtube Playlist by [Akhil Sharma](https://www.youtube.com/playlist?list=PL5dTjWUk_cPYuhHm9_QImW7_u4lr5d6zO).

I make slight deviations and improvements to my liking as I go along. Eventually I deviate completely when there are projects that I am not interested in (such as the Solana and Ethereum-related projects, I'm really only interested in Bitcoin).

The ones that are completely new are annotated.

## Here is the list:

1. [compress](./compress) - Takes in a file and compresses it using [flake2](https://docs.rs/flate2/latest/flate2/).
2. [decompress](./decompress/) - Takes in a zip file and decompresses it.
3. [read_csv](./read_csv/) - Reads csv and outputs to stdout.
4. [read_json](./read_json/) - Reads in JSON using [serde](https://serde.rs/).
5. [write_json](./write_json) - Writes JSON to stdout using [serde](https://serde.rs/).
6. [get_request](./get_request) - A simple Get request to an API. Prints out Status, Headers, and Body. Uses [reqwest](https://docs.rs/reqwest/latest/reqwest/) and [error_chain](https://docs.rs/error-chain/latest/error_chain/).
7. [async](./async_await/) - Similar to 6, except uses async await in typical fashion found in Javascript/Typescript. Also encorporates [anyhow](https://docs.rs/anyhow/latest/anyhow/) and [thiserror](https://docs.rs/thiserror/latest/thiserror/) instead of [error-chain](https://docs.rs/error-chain/latest/error_chain/)
8. [api_calls](./api_calls/) - Simple API call to Github which gets stargazer info for a rust-related repo.
9. [auth](./auth/) - Simple authentication using [reqwest](https://docs.rs/reqwest/latest/reqwest/)
