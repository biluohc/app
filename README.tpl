[![Build status](https://travis-ci.org/biluohc/app.svg?branch=master)](https://github.com/biluohc/app)
[![Latest version](https://img.shields.io/crates/v/app.svg)](https://crates.io/crates/app)
[![All downloads](https://img.shields.io/crates/d/app.svg)](https://crates.io/crates/app)
[![Downloads of latest version](https://img.shields.io/crates/dv/app.svg)](https://crates.io/crates/app)
[![Documentation](https://docs.rs/app/badge.svg)](https://docs.rs/app)

{{readme}}

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