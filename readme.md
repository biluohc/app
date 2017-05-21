[![Build status](https://travis-ci.org/biluohc/app.svg?branch=master)](https://github.com/biluohc/app)
[![Latest version](https://img.shields.io/crates/v/app.svg)](https://crates.io/crates/app)
[![All downloads](https://img.shields.io/crates/d/app.svg)](https://crates.io/crates/app)
[![Downloads of latest version](https://img.shields.io/crates/dv/app.svg)](https://crates.io/crates/app)
[![Documentation](https://docs.rs/app/badge.svg)](https://docs.rs/app)

# App

## A easy-to-use command-line-parser written for Rust.

## Usage
Cargo.toml

```toml
    [dependencies]  
    app = "0.5.6" 
```
## Or 

```toml
    [dependencies]  
    app = { git = "https://github.com/biluohc/app", branch = "master", version = "0.5.6" }
```

## Documentation  
* Visit [Docs.rs](https://docs.rs/app/)  
or 
* Run `cargo doc --open` after modified the toml file.

## Examples
* Options and Arguments: [fht2p](https://github.com/biluohc/app/blob/master/examples/fht2p.rs)
```
    git clone https://github.com/biluohc/app
    cd app-rs
    cargo run --example fht2p --release
```


* Sub_Commands: [zipcs](https://github.com/biluohc/zipcs)
```
    git clone https://github.com/biluohc/zipcs
    cd zipcs
    cargo run --release
```
