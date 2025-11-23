# blazon

<!-- just: blazon-md -->
[![Dependencies: 123](https://img.shields.io/badge/cargo%20tree-123-blue)](https://crates.io/crates/blazon)
[![Binary Size: 100MB](https://img.shields.io/badge/build%20size-100MB-blue)](https://crates.io/crates/blazon)<!-- /just: blazon-md -->
[![crates.io](https://img.shields.io/crates/v/blazon.svg)](https://crates.io/crates/blazon)
[![documentation](https://docs.rs/blazon/badge.svg)](https://docs.rs/blazon)
[![MIT licensed](https://img.shields.io/crates/l/blazon.svg)](https://github.com/lmmx/blazon/blob/master/LICENSE)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/lmmx/blazon/master.svg)](https://results.pre-commit.ci/latest/github/lmmx/blazon/master)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)

blazon is a fast minimal badge generator for Rust crate stats.

## Motivation

For when you want to add static badges to your crate that are rewritten by a hook, and need a
generator to drive the static badge generation.

This can help with providing metrics to aim to optimise such as:

- Crate dependency tree size
- Crate binary size
- Crate target dir size

Note that these metrics will vary by hardware but if executed on the same hardware it can be used to
provide a measure of progress. Alternatively put them in CI, run them one-off, however you like!

## Installation

Add blazon to your `Cargo.toml`:
```toml
[dependencies]
blazon = "0.1"
```

#### CLI Installation

- pre-built binary: `cargo binstall blazon` (requires [cargo-binstall][cargo-binstall]),
- build from source: `cargo install blazon --features cli`

[cargo-binstall]: https://github.com/cargo-bins/cargo-binstall

## License

This project is licensed under [MIT license](https://github.com/lmmx/blazon/blob/master/LICENSE)

at your option.
