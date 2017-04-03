# App-rs

## A easy-to-use command-line-parser written for Rust.

## Usage
Cargo.toml

```toml
    [dependencies]  
    app = "^0.4.0" 
```
## Or 

```toml
    [dependencies]  
    app = { git = "https://github.com/biluohc/app-rs",branch = "master", version = "^0.4.0" }
```

## Documentation  
* Visit [Docs.rs](https://docs.rs/app/)  
or 
* Run `cargo doc --open` after modified the toml file.

## Examples
* [fht2p](https://github.com/biluohc/app-rs/blob/master/examples/fht2p.rs)
```
    git clone https://github.com/biluohc/app-rs
    cd app-rs
    cargo run --example fht2p --release
```


* [zipcs](https://github.com/biluohc/zipcs)
```
    git clone https://github.com/biluohc/zipcs
    cd zipcs
    cargo run --release
```