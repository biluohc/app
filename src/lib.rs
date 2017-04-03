
//! # [app](https://github.com/biluohc/app-rs)
//!  A easy-to-use command-line-parser.
//!

//! ## Usage
//!
//! On Cargo.toml:
//!
//! ```toml
//!  [dependencies]
//!  app = "^0.3.0"
//! ```
//! or
//!
//! ```toml
//!  [dependencies]
//!  app = { git = "https://github.com/biluohc/app-rs",branch = "master", version = "^0.3.0" }
//! ```
//!
//! ## Examples
//! * [fht2p](https://github.com/biluohc/app-rs/blob/master/examples/fht2p.rs)
//! * [zipcs](https://github.com/biluohc/zipcs)

#[macro_use]
extern crate stderr;
use stderr::Loger;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::collections::BTreeMap as Map;
use std::process::exit;
use std::fmt::Debug;
use std::env;

static mut HELP: bool = false;
static mut HELP_SUBCMD: bool = false;
static mut VERSION: bool = false;

/// application struct
#[derive(Debug,Default)]
pub struct App<'app> {
    name: &'app str,
    version: &'app str,
    authors: Vec<(&'app str, &'app str)>, // (name,email)
    addrs: Vec<(&'app str, &'app str)>, // (addr_name,addr)
    // env_vars
    current_exe: Option<String>,
    current_dir: Option<String>,
    home_dir: Option<String>,
    temp_dir: String,
    //commands
    main: Cmd<'app>,
    current_cmd: Option<&'app mut String>, //main is None
    sub_cmds: Map<String, Cmd<'app>>, // name,_
}

impl<'app> App<'app> {
    pub fn new<'s: 'app>(name: &'s str) -> Self {
        init!();
        let mut app = Self::default();
        app.name = name;
        app = unsafe {
            app.opt(Opt::new("help", &mut HELP)
                        .short("h")
                        .long("help")
                        .help("show the help message"))
        };
        app = unsafe {
            app.opt(Opt::new("version", &mut VERSION)
                        .short("v")
                        .long("version")
                        .help("show the version message"))
        };
        app
    }
    pub fn version<'s: 'app>(mut self, version: &'s str) -> Self {
        self.version = version;
        self
    }
    pub fn desc<'s: 'app>(mut self, desc: &'s str) -> Self {
        self.main.desc = desc;
        self
    }
    pub fn author<'s: 'app>(mut self, name: &'s str, email: &'s str) -> Self {
        self.authors.push((name, email));
        self
    }
    pub fn addr<'s: 'app>(mut self, name: &'s str, url: &'s str) -> Self {
        self.addrs.push((name, url));
        self
    }
    pub fn opt(mut self, opt: Opt<'app>) -> Self {
        self.main = self.main.opt(opt);
        self
    }
    pub fn args<'s: 'app, S>(mut self, name: S, value: &'s mut Vec<String>) -> Self
        where S: Into<String>
    {
        self.main.args = Some(value);
        self.main.args_name = name.into();
        self
    }
    pub fn cmd(mut self, mut cmd: Cmd<'app>) -> Self {
        let name = cmd.name.to_string();
        cmd = unsafe {
            cmd.opt(Opt::new("help", &mut HELP_SUBCMD)
                        .short("h")
                        .long("help")
                        .help("show the help message"))
        };
        if self.sub_cmds.insert(name.clone(), cmd).is_some() {
            panic!("sub_command: \"{}\" already defined", name);
        }
        if self.current_cmd.is_none() {
            panic!("current_cmd's value no defined");
        }
        self
    }
    pub fn current_cmd<'s: 'app>(mut self, value: &'s mut String) -> Self {
        self.current_cmd = Some(value);
        self
    }
}
impl<'app> App<'app> {
    /// `parse_string(std::env::args()[1..])` and `exit(1)` if parse fails.
    pub fn parse(&mut self) {
        let mut args = env::args();
        args.next();
        let args: Vec<String> = args.collect();
        if let Err(e) = self.parse_strings(args) {
            errln!("ERROR:\n  {}\n", e);
            if self.current_cmd_get().is_some() && self.current_cmd_get() != Some(&mut String::new()) {
                if let Some(ref s) = self.current_cmd_get() {
                    self.help_cmd(s);
                }
            } else {
                self.help();
            }
            exit(1);
        }
    }
    pub fn parse_strings(&mut self, args: Vec<String>) -> Result<(), String> {
        dbstln!("args: {:?}", args);
        self.current_exe = env::current_exe()
            .map(|s| s.to_string_lossy().into_owned())
            .ok();
        self.current_dir = env::current_dir()
            .map(|s| s.to_string_lossy().into_owned())
            .ok();
        self.home_dir = env::home_dir().map(|s| s.to_string_lossy().into_owned());
        self.temp_dir = env::temp_dir().to_string_lossy().into_owned();
        let commands: Vec<String> = self.sub_cmds.keys().map(|s| s.to_string()).collect();
        let (mut idx, mut sub_cmd_name) = (std::usize::MAX, "");
        'out: for i in 0..args.len() {
            for cmd in &commands {
                if args[i] == **cmd {
                    idx = i;
                    sub_cmd_name = args[i].as_str();
                    break 'out;
                }
            }
        }
        if let Some(ref mut s) = self.current_cmd {
            s.clear();
            s.push_str(sub_cmd_name);
        }
        if idx != std::usize::MAX {
            self.main.parse(&args[0..idx])?;
            self.sub_cmds
                .get_mut(sub_cmd_name)
                .unwrap()
                .parse(&args[idx + 1..])?;
        } else {
            self.main.parse(&args[..])?;
        }
        if unsafe { HELP } {
            self.help();
            exit(0);
        } else if unsafe { HELP_SUBCMD } {
            self.help_cmd(sub_cmd_name);
            exit(0);
        } else if unsafe { VERSION } {
            self.ver();
            exit(0);
        }
        // check main.args
        if let Some(ref s) = self.main.args {
            if s.is_empty() {
                return Err(format!("Arguments({}) missing", self.main.args_name));
            }
        }
        // check main.opts
        for (_, opt) in &self.main.opts {
            opt.value.inner.check(opt.name_get())?;
        }
        // check current_cmd
        if sub_cmd_name != "" {
            let cmd = self.sub_cmds.get(sub_cmd_name).unwrap();
            if let Some(ref s) = cmd.args {
                if s.is_empty() {
                    return Err(format!("Arguments({}) missing", cmd.args_name));
                }
            }
            for (_, opt) in &cmd.opts {
                opt.value.inner.check(opt.name_get())?;
            }
        }
        Ok(())
    }
    fn ver(&self) {
        println!("{}  {}", self.name.trim(), self.version.trim());
    }
    // NAME
    fn help_name(&self) -> String {
        format!("NAME:\n  {} - {}\n  {}\n",
                self.name.trim(),
                self.version.trim(),
                self.main.desc.trim())
    }
    fn help_global_opt(&self) -> String {
        // GLOBAL OPTIONS:
        let mut help = String::new();
        if !self.main.opts.is_empty() {
            help += &{
                         let mut tmp = "\nGLOBAL OPTIONS:\n".to_owned();
                         let mut vs: Vec<(String, &str)> = Vec::new();
                         let mut len = 0;
                         for (k, v) in &self.main.opts {
                             let s = v.short_get().unwrap_or_else(String::new);
                             let long = v.long_get().unwrap_or_else(String::new);
                             let tmp_ = if v.is_bool() {
                                 if s != "" && long != "" {
                                     format!("   {},{}  ", long, s)
                                 } else {
                                     format!("   {}{}  ", long, s)
                                 }
                             } else if s != "" && long != "" {
                        format!("   {} {},{} {}  ", long, k, s, k)
                    } else {
                        format!("   {}{} {}  ", s, long, k)
                    };
                             if tmp_.len() > len {
                                 len = tmp_.len();
                             }
                             vs.push((tmp_, v.help_get()));
                         }
                         for (k, v) in vs {
                             let mut tmp_ = k.clone();
                             for _ in tmp_.len()..len {
                                 tmp_.push(' ');
                             }
                             tmp += &format!("{}  {}\n", tmp_, v.trim());
                         }
                         tmp
                     }
        }
        help
    }
    pub fn help(&self) {
        // NAME
        let mut help = self.help_name();
        //Author
        if !self.authors.is_empty() {
            help += &{
                         let mut authors = String::new() + "\nAUTHOR:\n";
                         for &(author, email) in &self.authors {
                             authors += &format!("  {} <{}>\n", author, email);
                         }
                         authors
                     };
        }
        //ADDRESS
        if !self.addrs.is_empty() {
            help += &{
                         let mut authors = String::new() + "\nADDRESS:\n";
                         for &(author, email) in &self.addrs {
                             authors += &format!("  {}: {}\n", author, email);
                         }
                         authors
                     };
        }
        //USAGE
        //    zipcs [global options] [global arguments] command [command options] [arguments...]
        help += &{
                     let mut tmp = format!("\nUSAGE:\n  {}", self.name);
                     if !self.main.opts.is_empty() {
                         tmp += " [global options]";
                     }
                     if self.main.args.is_some() {
                         tmp += &format!(" [{}...]", self.main.args_name);
                     }
                     if !self.sub_cmds.is_empty() {
                         tmp += " command [command options] [arguments...]";
                     }
                     tmp + "\n"
                 };
        // GLOBAL OPTIONS:
        help += &self.help_global_opt();
        // SUBCOMMANDS
        if !self.sub_cmds.is_empty() {
            help += &{
                         let mut tmp = "\nCOMMANDS:\n".to_owned();
                         let mut vs: Vec<(String, &str)> = Vec::new();
                         let mut len = 0;
                         for (k, v) in &self.sub_cmds {
                             let tmp_ = format!("    {}  ", k);
                             if tmp_.len() > len {
                                 len = tmp_.len();
                             }
                             vs.push((tmp_, v.desc));
                         }
                         for (k, v) in vs {
                             let mut tmp_ = k.clone();
                             for _ in tmp_.len()..len {
                                 tmp_.push(' ');
                             }
                             tmp += &format!("{}  {}\n", tmp_, v.trim());
                         }
                         tmp
                     };
        }
        println!("{}", help.trim());
    }
    pub fn help_cmd(&self, sub_cmd_name: &str) {
        // NAME
        let mut help = self.help_name();
        //USAGE
        //    zipcs [global options] [global arguments] command [command options] [arguments...]
        help += &{
                     let mut tmp = format!("\nUSAGE:\n  {}", self.name);
                     if !self.main.opts.is_empty() {
                         tmp += " [global options]";
                     }
                     if self.main.args.is_some() {
                         tmp += &format!(" [{}...]", self.main.args_name);
                     }
                     if let Some(s) = self.sub_cmds.get(sub_cmd_name) {
                         if !s.opts.is_empty() && s.args.is_some() {
                             tmp += &format!(" {} [{} options] [{}...]", s.name, s.name, s.args_name);
                         } else if !s.opts.is_empty() && s.args.is_none() {
                    tmp += &format!(" {} [{} options]", s.name, s.name);
                } else if s.opts.is_empty() && s.args.is_some() {
                    tmp += &format!(" {} [{}...]", s.name, s.args_name);
                }
                     }
                     tmp + "\n"
                 };
        // GLOBAL OPTIONS:
        help += &self.help_global_opt();
        // SubCMD OPTIONS
        let cmd = self.sub_cmds.get(sub_cmd_name).unwrap();
        if !cmd.opts.is_empty() {
            help += &{
                         let mut tmp = "\nOPTIONS:\n".to_owned();
                         let mut vs: Vec<(String, &str)> = Vec::new();
                         let mut len = 0;
                         for (k, v) in &cmd.opts {
                             let s = v.short_get().unwrap_or_else(String::new);
                             let long = v.long_get().unwrap_or_else(String::new);
                             let tmp_ = if v.is_bool() {
                                 if s != "" && long != "" {
                                     format!("    {},{}  ", long, s)
                                 } else {
                                     format!("    {}{}  ", long, s)
                                 }
                             } else if s != "" && long != "" {
                        format!("    {} {},{} {}  ", long, k, s, k)
                    } else {
                        format!("    {}{} {}  ", s, long, k)
                    };
                             if tmp_.len() > len {
                                 len = tmp_.len();
                             }
                             vs.push((tmp_, v.help_get()));
                         }
                         for (k, v) in vs {
                             let mut tmp_ = k.clone();
                             for _ in tmp_.len()..len {
                                 tmp_.push(' ');
                             }
                             tmp += &format!("{}  {}\n", tmp_, v.trim());
                         }
                         tmp
                     }
        }
        println!("{}", help.trim());
    }
    pub fn current_cmd_get(&self) -> Option<&str> {
        if let Some(ref s) = self.current_cmd {
            Some(s)
        } else {
            None
        }
    }
    pub fn current_exe(&self) -> Option<&String> {
        self.current_exe.as_ref()
    }
    pub fn current_dir(&self) -> Option<&String> {
        self.current_dir.as_ref()
    }
    pub fn home_dir(&self) -> Option<&String> {
        self.home_dir.as_ref()
    }
    pub fn temp_dir(&self) -> &String {
        &self.temp_dir
    }
}

///Command strcut
#[derive(Debug,Default)]
pub struct Cmd<'app> {
    name: &'app str,
    desc: &'app str,
    opts: Map<String, Opt<'app>>,
    str_to_name: Map<String, String>, //-short/--long to name
    args_name: String,
    args: Option<&'app mut Vec<String>>,
}
impl<'app> Cmd<'app> {
    pub fn new<'s: 'app>(name: &'s str) -> Self {
        let mut c = Self::default();
        c.name = name;
        c
    }
    pub fn desc<'s: 'app>(mut self, desc: &'s str) -> Self {
        self.desc = desc;
        self
    }
    pub fn args<'s: 'app, S>(mut self, name: S, value: &'s mut Vec<String>) -> Self
        where S: Into<String>
    {
        self.args = Some(value);
        self.args_name = name.into();
        self
    }
    pub fn opt(mut self, opt: Opt<'app>) -> Self {
        let long = opt.long_get();
        let short = opt.short_get();
        let name = opt.name_get().to_string();
        if long.is_none() && short.is_none() {
            panic!("OPTION: \"{}\" don't have --{} and -{} all",
                   name,
                   name,
                   name);
        }
        if let Some(ref s) = long {
            if self.str_to_name
                   .insert(s.clone(), name.clone())
                   .is_some() {
                panic!("long: \"{}\" already defined", s);
            }
        }
        if let Some(ref s) = short {
            if self.str_to_name
                   .insert(s.clone(), name.clone())
                   .is_some() {
                panic!("short: \"{}\" already defined", s);
            }
        }
        if self.opts.insert(name.clone(), opt).is_some() {
            panic!("name: \"{}\" already defined", name);
        }
        self
    }
    fn parse(&mut self, args: &[String]) -> Result<(), String> {
        let mut i = 0;
        for _ in 0..args.len() {
            if i >= args.len() {
                break;
            }
            let arg = &args[i];
            // println!("i+1/args_len: {}/{}: {:?}", i + 1, args.len(), &args[i..]);
            match arg {
                s if s.starts_with("--") => {
                    if let Some(opt_name) = self.str_to_name.get(s.as_str()) {
                        let mut opt = self.opts.get_mut(opt_name).unwrap();
                        let opt_is_bool = opt.is_bool();
                        if !opt_is_bool && args.len() > i + 1 {
                            opt.parse(&args[i + 1])?;
                            i += 2;
                        } else if opt_is_bool {
                            opt.parse("")?;
                            i += 1;
                        } else {
                            return Err(format!("OPTION({})'s value missing", s));
                        }
                    } else {
                        return Err(format!("OPTION: \"{}\" not defined", s));
                    }
                }
                s if s.starts_with('-') => {
                    if let Some(opt_name) = self.str_to_name.get(s.as_str()) {
                        let mut opt = self.opts.get_mut(opt_name).unwrap();
                        let opt_is_bool = opt.is_bool();
                        if !opt_is_bool && args.len() > i + 1 {
                            opt.parse(&args[i + 1])?;
                            i += 2;
                        } else if opt_is_bool {
                            opt.parse("")?;
                            i += 1;
                        } else {
                            return Err(format!("OPTION({})'s value missing", s));
                        }
                    } else {
                        return Err(format!("OPTION: \"{}\" not defined", s));
                    }
                }
                s => {
                    if let Some(ref mut ss) = self.args {
                        ss.push(s.to_string());
                        i += 1;
                    } else {
                        return Err(format!("Argument: \"{}\" no need", s));
                    }
                }
            }
        }
        Ok(())
    }
}

/// Option struct
#[derive(Debug)]
pub struct Opt<'app> {
    name: &'app str,
    value: OptValue<'app>,
    short: Option<&'app str>,
    long: Option<&'app str>,
    help: &'app str,
}
impl<'app> Opt<'app> {
    pub fn new<'s: 'app, V>(name: &'app str, value: V) -> Self
        where V: OptValueParse<'app>
    {
        Opt {
            value: value.into_opt_value(),
            name: name,
            short: None,
            long: None,
            help: "",
        }
    }
    pub fn short(mut self, short: &'app str) -> Self {
        self.short = Some(short);
        self
    }
    pub fn long(mut self, long: &'app str) -> Self {
        self.long = Some(long);
        self
    }
    pub fn help(mut self, help: &'app str) -> Self {
        self.help = help;
        self
    }
    fn parse(&mut self, msg: &str) -> Result<(), String> {
        let name = self.name_get().to_string();
        self.value.inner.parse(name, msg)
    }
}

impl<'app> Opt<'app> {
    pub fn is_bool(&self) -> bool {
        self.value.inner.is_bool()
    }
    pub fn name_get(&self) -> &'app str {
        self.name
    }
    pub fn short_get(&self) -> Option<String> {
        self.short.map(|s| "-".to_owned() + s)
    }
    pub fn long_get(&self) -> Option<String> {
        self.long.map(|s| "--".to_owned() + s)
    }
    pub fn help_get(&self) -> &str {
        self.help
    }
}

/// `OptValue` struct
#[derive(Debug)]
pub struct OptValue<'app> {
    pub inner: Box<OptValueParse<'app> + 'app>,
}

/// ## You can use custom `OptValue` by `impl` it
///
/// ### Explain
/// 
/// * `into_opt_value(self)` convert it(`&mut T`)  to `OptValue`.
///
///
/// * `is_bool(&self)` like `--help/-h`,they not have value follows it.
///
///    so you should return `false` except value's type is `&mut bool`(it already defined).
///
///
/// * `parse(&mut self, opt_name: String, msg: &str)` maintains the value, and return message by `Result<(),String>`.
///
///   `opt_name` is current `Opt`'s name, `msg` is `&str` need to pasre.
///
/// * `check(&self, opt_name: &str)` check value  and return message by `Result<(),String>`.
///
/// ### Suggestion
/// 
/// * `T` is suitable for options with default values.
///
///     You can initialize it using the default value. 
/// 
/// * `Option<T>` is suitable for necessary options.
///
///     `app` will `check` them, is `value.is_none()`, `app` will `exit(1)` after print error and help message.
///
/// * `Vec<T>` is suitable a grout of comma-separated values of the same type.
///
///     `app` will `check` them, is `value.is_empty()`, `app` will `exit(1)` after print error and help message.
///
///     You can initialize it using the default value.
///
/// ```fuckrs
/// "80" -> vec[80]
/// ",80," -> vec[80]
/// ",80,," -> vec[80]
/// "8080,8000,80," -> Vec[8080,8000,80]
/// ```

pub trait OptValueParse<'app>: Debug {
    fn into_opt_value(self) -> OptValue<'app>;
    fn is_bool(&self) -> bool;
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String>;
    fn check(&self, opt_name: &str) -> Result<(), String>;
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut bool {
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        true
    }
    fn parse(&mut self, _: String, _: &str) -> Result<(), String> {
        **self = true;
        Ok(())
    }
    fn check(&self, _: &str) -> Result<(), String> {
        Ok(())
    }
}
impl<'app, 's: 'app> OptValueParse<'app> for &'s mut String {
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn parse(&mut self, _: String, msg: &str) -> Result<(), String> {
        **self = msg.to_string();
        Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_empty() {
            Err(format!("OPTION({})'s value missing", opt_name))
        } else {
            Ok(())
        }
    }
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut char {
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
        if msg.len() == 1 {
            **self = msg.chars().next().unwrap();
        } else {
            return Err(format!("OPTION({}) parse<char> fails: \"{}\"", opt_name, msg));
        }
        Ok(())
    }
    fn check(&self, _: &str) -> Result<(), String> {
        Ok(())
    }
}

macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl<'app, 's: 'app> OptValueParse<'app> for &'s mut $t {
        fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn parse(&mut self, opt_name : String, msg: &str) -> Result<(), String> {
        **self = msg.parse::<$t>()
                .map_err(|_| format!("OPTION({}) parse<{}> fails: \"{}\"", opt_name, stringify!($t),msg))?;
                Ok(())
    }
    fn check(&self, _ :  &str) -> Result<(), String> {
        Ok(())
    }
        }
    )*)
}

add_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }
add_impl! { IpAddr Ipv4Addr Ipv6Addr SocketAddr SocketAddrV4 SocketAddrV6 }

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Option<char> {
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
        if msg.len() == 1 {
            **self = Some(msg.chars().next().unwrap());
        } else {
            return Err(format!("OPTION({}) parse<char> fails: \"{}\"", opt_name, msg));
        }
        Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_none() {
            Err(format!("OPTION({})'s value missing", opt_name))
        } else {
            Ok(())
        }
    }
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Option<String> {
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn parse(&mut self, _: String, msg: &str) -> Result<(), String> {
        **self = Some(msg.to_string());
        Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_none() {
            Err(format!("OPTION({})'s value missing", opt_name))
        } else {
            Ok(())
        }
    }
}

macro_rules! add_option_impl {
    ($($t:ty)*) => ($(
impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Option<$t> {
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
        **self = Some(msg.parse::<$t>()
                          .map_err(|_| {
                                       format!("OPTION({}) parse<{}> fails: \"{}\"",
                                               opt_name,
                                               stringify!($t),
                                               msg)
                                   })?);
        Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_none() {
            Err(format!("OPTION({})'s value missing", opt_name))
        } else {
            Ok(())
        }
    }
}
    )*)
}

add_option_impl! { bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }
add_option_impl! { IpAddr Ipv4Addr Ipv6Addr SocketAddr SocketAddrV4 SocketAddrV6 }

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Vec<char> {
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn parse(&mut self, _: String, msg: &str) -> Result<(), String> {
        self.clear();
        for c in msg.chars() {
            self.push(c);
        }
        Ok(())
    }
    fn check(&self, _: &str) -> Result<(), String> {
        Ok(())
    }
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Vec<String> {
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn parse(&mut self, _: String, msg: &str) -> Result<(), String> {
        self.clear(); // What due to ?
        let _ = msg.split(',')
            .filter(|s| !s.is_empty())
            .map(|ss| self.push(ss.to_string()));
        Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_empty() {
            Err(format!("OPTION({})'s value missing", opt_name))
        } else {
            Ok(())
        }
    }
}

macro_rules! add_vec_impl {
    ($($t:ty)*) => ($(
        impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Vec<$t> {
        fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
                self.clear();
                let vs: Vec<&str> = msg.split(',').filter(|s| !s.is_empty()).collect();
                for str in &vs {
                    self.push(str.parse::<$t>()
                               .map_err(|_| {
                                            format!("OPTION({}) parse<Vec<{}>> fails: \"{}/{}\"",
                                                    opt_name,
                                                    stringify!($t),
                                                    str,
                                                    msg)
                                        })?)
                }
                Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_empty() {
          Err(format!("OPTION({})'s value missing", opt_name))
        } else {
            Ok(())
        }
    }
        }
    )*)
}
add_vec_impl! { bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }
add_vec_impl! { IpAddr Ipv4Addr Ipv6Addr SocketAddr SocketAddrV4 SocketAddrV6 }
