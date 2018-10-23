# create-protocols-plugin
A Rust cargo subcommand that will create initial code for the protocols crate. 

[![Build Status](https://travis-ci.org/zutils/signals.svg?branch=master)](https://travis-ci.org/zutils/create-protocols-plugin)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/zutils/create-protocols-plugin/blob/master/LICENSE-MIT)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)


## Installation
cargo install create-protocols-plugin

## Usage
cargo create-protocols-plugin <Crate Name> --protocol <.proto File>

# What does it do?
This crate will create you a crate with default functions to get started using the protocols library.

# Future Work
Build a library based on a Cap'n Proto schema.
Build a library based on flatbuffer schema.


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
