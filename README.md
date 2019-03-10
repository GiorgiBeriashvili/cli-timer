# cli-timer

[![LICENSE][License-Image]](#License "Project's LICENSE section")
[![Tokei.rs](https://tokei.rs/b1/github/0rganic/cli-timer)](https://github.com/0rganic/cli-timer "Project's total lines of code")
[![Crates.io](https://img.shields.io/crates/v/cli-timer.svg)](https://crates.io/crates/cli-timer "Package's crates.io page")
[![Docs.rs](https://docs.rs/cli-timer/badge.svg)](https://docs.rs/crate/cli-timer/0.3.31 "Package's docs.rs page")

[License-Image]: https://img.shields.io/badge/License-MIT_or_Apache_2.0-blue.svg

## Description

An interactive command-line interface timer application written in Rust.

<hr>

## Building

In order to build cli-timer, you need to have [Rust](https://www.rust-lang.org "Rust programming language's official website") programming language installed on your system. To install Rust (alongside Cargo, which comes bundled with Rust), it's best to follow the [official installation steps](https://www.rust-lang.org/tools/install "Official guide to install Rust").

Building is guaranteed to work with Rust version 1.33.0 (2aa4c46cf 2019-02-28).

```sh
# Clone the repository
git clone https://github.com/0rganic/cli-timer
cd cli-timer

# Compile the release version
cargo build --release

# Run the release version
cargo run --release

# To compile and run in the debug mode and to print the help information
cargo run -- -h
```

<hr>

## Changelog

All notable changes to this project will be documented in the [CHANGELOG.md](CHANGELOG.md "Project's CHANGELOG.md file") file.

<hr>

## License

cli-timer is licensed under either of the following, at your option:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE "Copy of the Apache license (version 2.0)"))
* MIT License ([LICENSE-MIT](LICENSE-MIT "Copy of the MIT license"))