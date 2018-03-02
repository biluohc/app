extern crate app;
use app::{app, args, opt};

use std::path::PathBuf;

fn main() {
    Config::parse_args().call()
}

impl Config {
    fn call(self) {
        println!("call:\n{:?}", self);
        //do some work
    }
    fn parse_args() -> Self {
        let mut config = Config::default();

        let helper = {
            app(
                "cp",
                "0.6.0",
                "Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.",
            ).addr(
                "Github",
                "https://github.com/biluohc/app/blob/master/examples/cp.rs",
            )
                .opt(opt(
                    "force",
                    &mut config.force,
                    Some('f'),
                    Some("force"),
                    "if an existing destination file cannot be opened, remove it and try again",
                ))
                .opt(opt(
                    "recursive",
                    &mut config.recursive,
                    Some('r'),
                    Some("recursive"),
                    "Recursively copy all content within a directory and its subdirectories",
                ))
                .args(args(
                    "SOURCE",
                    &mut config.source,
                    "CP the SOURCE(s) to DEST",
                ))
                .args(args("DEST", &mut config.dest, "DEST Path").len(1usize))
                .parse_args()
        };
        config
            .check()
            .map_err(|e| helper.help_err_exit(e, 1))
            .unwrap() // map_err alrendy exit if it is err, so unwrap is safe.
    }
    fn check(self) -> Result<Self, String> {
        println!("check:\n{:?}", self);
        for path in &self.source {
            if !path.as_path().exists() {
                return Err(format!("Args(SOURCE): {:?} is not exists", path));
            }
        }
        Ok(self)
    }
}

#[derive(Debug, Default)]
pub struct Config {
    pub recursive: bool,
    pub force: bool,
    pub source: Vec<PathBuf>,
    pub dest: Vec<PathBuf>,
}
