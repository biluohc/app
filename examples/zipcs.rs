extern crate app;

use app::{App, Cmd, Opt, Args, OptValue, OptValueParse, OptTypo};

fn main() {
    Config::parse()
}
#[derive(Debug, Default)]
pub struct Config {
    zip: Zips,
    ping: Pings,
    url: Urls,
}
impl Config {
    pub fn parse() {
        let mut config = Self::default();
        config.zip.outdir = "./".to_owned();
        let mut list = false;
        let charsets = format!(
            "Sets the charset Zipcs using({})",
            "UTF_8, UTF_16BE, UTF_16LE, GBK, GB18030, HZ, BIG5"
                .replace("_", "")
                .to_lowercase()
        );
        let helper = {
            App::new("zipcs")
                .version("0.6.0")
                .author("Wspsxing", "biluohc@qq.com")
                .addr("Repo", "https://github.com/biluohc/zipcs")
                .desc("Useful tools collection.")
                .cmd(
                    Cmd::new("zip")
                        .short("z")
                        .sort_key("a0")
                        .desc("Unzip with charset setting.")
                        .opt(Opt::new("list", &mut list).short('l').long("list").help(
                            "Only list files from ZipArchives",
                        ))
                        .opt(
                            Opt::new("charset", &mut config.zip.charset)
                                .short('c')
                                .long("charset")
                                .help(&charsets),
                        )
                        .opt(
                            Opt::new("outdir", &mut config.zip.outdir)
                                .short('o')
                                .long("outdir")
                                .help("Sets Output directory"),
                        )
                        .args(Args::new("ZipArchive", &mut config.zip.zips).help(
                            "ZipArchive need to unzip",
                        )),
                )
                .cmd(
                    Cmd::new("ping")
                        .short("p")
                        .sort_key("a1")
                        .desc("ping domains/ips.")
                        .opt(
                            Opt::new("count", &mut config.ping.count)
                                .short('c')
                                .long("count")
                                .help("stop after sending count ECHO_REQUEST packets"),
                        )
                        .opt(Opt::new("_6", &mut config.ping._6).short('6').help(
                            "use IPV6",
                        ))
                        .opt(
                            Opt::new("only-line", &mut config.ping.only_line)
                                .short('l')
                                .long("only-line")
                                .help("print result only-line"),
                        )
                        .args(Args::new("Host/IP", &mut config.ping.hosts).help(
                            "Host or IP need to ping",
                        )),
                )
                .cmd(
                    Cmd::new("url")
                        .short("l")
                        .sort_key("a2")
                        .desc("Urls decoding/encoding.")
                        .opt(
                            Opt::new("encode", &mut config.url.is_encode)
                                .short('e')
                                .long("encode")
                                .help("encode(default is decode)"),
                        )
                        .opt(
                            Opt::new("plus", &mut config.url.is_plus)
                                .short('p')
                                .long("plus")
                                .help("replaces ' ' with '+'"),
                        )
                        .args(Args::new("Url", &mut config.url.strs).help(
                            "Url need to decode/encode",
                        )),
                )
                .parse_args()
        };

        if *helper.args_len() == 0 
        {
            helper.help_exit(0);
        }
        
        if list {
            config.zip.task = Task::LIST;
        }
        config
            .check_and_call(helper.current_cmd_str())
            .map_err(|e| helper.help_cmd_err_exit(helper.current_cmd_ref(), e, 1))
            .unwrap(); // map_err alrendy exit if it is err, so unwrap is safe.
    }
    fn check_and_call(self, cmd: Option<&str>) -> Result<(), String> {
        println!("Match Cmd: {:?}", cmd);
        match cmd {
            Some("zip") => {
                self.zip.check()?;
                self.zip.call()?;
            }
            Some("ping") => {
                self.ping.call();
            }
            Some("url") => {
                self.url.call();
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

/// Custom `OptValue` by impl `OptValueParse`
impl<'app, 's: 'app> OptValueParse<'app> for &'s mut CharSet {
    fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        Some("utf8".to_owned())
    }
    fn parse(&mut self, opt_name: &str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String> {
        if *count == 0 || typo.is_covered() || typo.is_multiple() {
            match CharSet::new(msg) {
                Err(_) => {
                    Err(format!(
                        "OPTION(<{}>) parse<CharSet> fails: \"{}\"",
                        opt_name,
                        msg
                    ))?;
                }
                Ok(o) => **self = o,}
        } else if typo.is_single() {
            Err(format!(
                "OPTION(<{}>) can only occurs once, but second: {:?}",
                opt_name,
                msg
            ))?;
        }
        Ok(())
    }
    /// env::arg could is `""`
    fn check(&self, opt_name: &str, optional: &bool, count: &usize, _: &OptTypo) -> Result<(), String> {
        if !optional && *count == 0 &&self.default().is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Urls {
    pub is_plus: bool,
    pub is_encode: bool,
    pub strs: Vec<String>,
}
impl Urls {
    fn call(self) {
        //do something
    }
}
#[derive(Debug, Default)]
pub struct Pings {
    pub _6: bool,
    pub only_line: bool,
    pub hosts: Vec<String>,
    pub count: u64,
}
impl Pings {
    fn call(self) {
        //do something
    }
}
#[derive(Debug, PartialEq)]
pub enum Task {
    LIST, // zipcs -l/--list
    UNZIP, // Extract files from archive with full paths
}
impl Default for Task {
    fn default() -> Task {
        Task::UNZIP
    }
}

#[derive(Debug, Default)]
pub struct Zips {
    pub charset: CharSet, //zip -cs/--charset   // utf-8
    pub outdir: String, //zipcs -o/--outdir   //./
    pub zips: Vec<String>, //zipcs [ZipArchive..]
    pub task: Task, // UNZIP
}
impl Zips {
    fn check(&self) -> Result<(), String> {
        //do something
        Ok(())
    }
    fn call(self) -> Result<(), String> {
        //do something
        Ok(())
    }
}
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CharSet {
    UTF_8,
    UTF_16BE,
    UTF_16LE,
    GBK,
    GB18030,
    HZ,
    BIG5_2003,
}
impl CharSet {
    pub fn new(name: &str) -> Result<Self, ()> {
        let cs = match name {
            "utf8" => CharSet::UTF_8,
            "utf16be" => CharSet::UTF_16BE,
            "utf16le" => CharSet::UTF_16LE,
            "gbk" => CharSet::GBK,
            "gb18030" => CharSet::GB18030,
            "hz" => CharSet::HZ,
            "big5" => CharSet::BIG5_2003,
            _ => return Err(()),
        };
        Ok(cs)
    }
    pub fn decode(&self, _: &[u8]) -> Result<String, std::borrow::Cow<'static, str>> {
        unimplemented!()
    }
    pub fn encode(&self, _: &str) -> Result<Vec<u8>, std::borrow::Cow<'static, str>> {
        unimplemented!()
    }
}
impl Default for CharSet {
    fn default() -> CharSet {
        CharSet::UTF_8
    }
}
