extern crate app;
use app::{App, Opt, Args};

use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

fn main() {
    Config::parse_args().call()
}

impl Config {
    fn call(self) {
        println!("{:?}", self);
        //do some work
    }
    fn parse_args() -> Self {
        let mut config = Config::default();
        let mut cp = false;
        let mut c_path: Option<String> = None;
        let helper = {
            App::new("fht2p")
                .version("0.6.0")
                .author("Wspsxing", "biluohc@qq.com")
                .addr("Github", "https://github.com/biluohc/fht2p")
                .desc("A HTTP Server for Static File written with Rust")
                .opt(Opt::new("cp", &mut cp).short('c').long("cp").help(
                    "Print the default config file",
                ))
                .opt(
                    Opt::new("config", &mut c_path)
                        .optional()
                        .short('C')
                        .long("config")
                        .help("Sets a custom config file"),
                )
                .opt(
                    Opt::new("keep_alive", &mut config.keep_alive)
                        .short('k')
                        .long("keep-alive")
                        .help("use keep-alive"),
                )
                .opt(
                    Opt::new("ip", &mut config.server.ip)
                        .short('i')
                        .long("ip")
                        .help("Sets listenning ip"),
                )
                .opt(
                    Opt::new("port", &mut config.server.port)
                        .short('p')
                        .long("port")
                        .help("Sets listenning port"),
                )
                .args(Args::new("PATH", &mut config.paths).help(
                    "Sets the path to share",
                ))
                .parse_args()
        };
        if cp {
            // Print the default config file
            println!("-cp/--cp");
            helper.exit(0);
        }
        if let Some(c) = c_path {
            // use custom config file
            println!("-c/--config: {:?}", c);
        }
        config
            .check()
            .map_err(|e| helper.help_err_exit(e, 1))
            .unwrap() // map_err alrendy exit if it is err, so unwrap is safe.

    }
    fn check(self) -> Result<Self, String> {
        for path in &self.paths {
            if !path.as_path().exists() {
                return Err(format!("Args(<PATH>): {:?} is not exists", path));
            }
        }
        Ok(self)
    }
}

#[derive(Debug, Clone)]
pub struct Server {
    pub ip: IpAddr,
    pub port: u16,
}
impl Default for Server {
    fn default() -> Server {
        Server {
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 8080,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub keep_alive: bool,
    pub server: Server,
    pub paths: Vec<PathBuf>,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            keep_alive: false,
            server: Server::default(),
            paths: vec![PathBuf::from("./")],
        }
    }
}
