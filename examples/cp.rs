extern crate app;
use app::{App, Opt, Args};

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
            App::new("cp")
                .version("0.6.0")
                .author("Wspsxing", "biluohc@qq.com")
                .addr("Github",
                      "https://github.com/biluohc/app/blob/master/examples/cp.rs")
                .desc("Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.")
                .opt(Opt::new("force", &mut config.force)
                         .short("f")
                         .long("force")
                         .help("if an existing destination file cannot be opened, remove it and try again"))
                .opt(Opt::new("recursive", &mut config.recursive)
                         .short("r")
                         .long("recursive")
                         .help("Recursively copy all content within a directory and its subdirectories"))
                .args(Args::new("SOURCE", &mut config.source).help("CP the SOURCE(s) to DEST"))
                .args(Args::new("DEST", &mut config.dest)
                          .len(1usize)
                          .help("DEST Path"))
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

#[derive(Debug,Default)]
pub struct Config {
    pub recursive: bool,
    pub force: bool,
    pub source: Vec<PathBuf>,
    pub dest: Vec<PathBuf>,
}
