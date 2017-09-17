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
    app = "0.6.1"
```
### Or

```toml
    [dependencies]
    app = { git = "https://github.com/biluohc/app",branch = "master", version = "0.6.1" }
```

### Documentation
* Visit [Docs.rs](https://docs.rs/app/)

Or

* Run `cargo doc --open` after modified the toml file.

### Examples

```bash
    git clone https://github.com/biluohc/app
```

* [fht2p](https://github.com/biluohc/app/blob/master/examples/fht2p.rs): Options and Args

```bash
    cargo run --example fht2p -- -h
```

* [cp](https://github.com/biluohc/app/blob/master/examples/cp.rs): Options and Multi Args

```bash
    cargo run --example cp
```

* [cpfn](https://github.com/biluohc/app/blob/master/examples/cpfn.rs): Options, Multi Args and the help funcions.

```bash
    cargo run --example cpfn
```

* [zipcs](https://github.com/biluohc/app/blob/master/examples/zipcs.rs): `Sub_Commands, OptValue and OptValueParse`

```bash
    cargo run --example zipcs
```

* [`http`](https://github.com/biluohc/app/blob/master/examples/http.rs): Option's order in help message

```bash
    cargo run --example http
```

* [cargo-http](https://github.com/biluohc/app/blob/master/examples/cargo-http.rs): Custom `Helps` and `cargo subcmd`

```bash
    cargo run --example cargo-http
```

## To Du

name | status | exapmle
 -|-|-|
Flag  |√|               `ls --help` , `cargo -V`
Option |√|              `http --port 8080` , `rustc -o filename`
Args  |√|               `rm Path1 Path2 Path3`
SubCMD |√|              `cargo run` , `cargo doc` 
Flags aggregation|√|    `ls -a -l` => `ls -al`
Multi Args |√|          `cp SOURCE1 SOURCE2 SOUCE3 DEST`
Optional for Option and Args |√| 
Dependencies and Conflicts between Options |x|
