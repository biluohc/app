[![Build status](https://travis-ci.org/biluohc/app.svg?branch=master)](https://github.com/biluohc/app)
[![Latest version](https://img.shields.io/crates/v/app.svg)](https://crates.io/crates/app)
[![All downloads](https://img.shields.io/crates/d/app.svg)](https://crates.io/crates/app)
[![Downloads of latest version](https://img.shields.io/crates/dv/app.svg)](https://crates.io/crates/app)
[![Documentation](https://docs.rs/app/badge.svg)](https://docs.rs/app)

## [app](https://github.com/biluohc/app)

### A easy-to-use command-line-parser written for Rust.

### Usage
Cargo.toml

```toml
    [dependencies]
    app = "0.6.0"
```
### Or

```toml
    [dependencies]
    app = { git = "https://github.com/biluohc/app",branch = "master", version = "0.6.0" }
```

### Documentation
* Visit [Docs.rs](https://docs.rs/app/)
or
* Run `cargo doc --open` after modified the toml file.

### Examples
* [fht2p](https://github.com/biluohc/app/blob/master/examples/fht2p.rs): Options and Args

```rustful
    git clone https://github.com/biluohc/app
    cd app
    cargo run --example fht2p -- -h
```
* [cp](https://github.com/biluohc/app/blob/master/examples/cp.rs): Options and `Multi_Args`

```rustful
    git clone https://github.com/biluohc/app
    cd app
    cargo run --example cp
```

* [zipcs](https://github.com/biluohc/app/blob/master/examples/zipcs.rs): `Sub_Commands, OptValue and OptValueParse`

```rustful
    git clone https://github.com/biluohc/app
    cd app
    cargo run --example zipcs
```

* [`sort_key`](https://github.com/biluohc/app/blob/master/examples/sort_key.rs): Option's order in help message

```rustful
    git clone https://github.com/biluohc/app
    cd app
    cargo run --example http
```

* [cargo-http](https://github.com/biluohc/app/blob/master/examples/cargo-http.rs): Custom `Helps` and `cargo subcmd`

```rustful
    git clone https://github.com/biluohc/app
    cd app
    cargo run --example cargo-http
```
