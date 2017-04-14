
//! # [app](https://github.com/biluohc/app-rs)
//!  A easy-to-use command-line-parser.
//!

//! ## Usage
//!
//! On Cargo.toml:
//!
//! ```toml
//!  [dependencies]
//!  app = "0.5.4"
//! ```
//! or
//!
//! ```toml
//!  [dependencies]
//!  app = { git = "https://github.com/biluohc/app-rs",branch = "master", version = "0.5.4" }
//! ```
//!
//! ## Examples
//! * [fht2p](https://github.com/biluohc/app-rs/blob/master/examples/fht2p.rs)
//! * [zipcs](https://github.com/biluohc/zipcs)

#[macro_use]
extern crate stderr;
use stderr::Loger;
extern crate term;
mod ovp;
pub use ovp::{OptValue, OptValueParse};

use std::collections::BTreeMap as Map;
use std::fmt::{self, Display};
use std::default::Default;
use std::io::prelude::*;
use std::process::exit;
use std::env;

const ERROR_LINE_NUM: usize = 1; // for print error with color(Red)
static mut HELP: bool = false;
static mut HELP_SUBCMD: bool = false;
static mut VERSION: bool = false;
static OPTIONAL: &'static str = "(optional)";

/// **Application**
#[derive(Debug,Default)]
pub struct App<'app> {
    //commands
    main: Cmd<'app>,
    current_cmd: Option<&'app mut Option<String>>, //main is None
    sub_cmds: Map<Option<String>, Cmd<'app>>, // name,_
    helper: Helper,
}

/// **`Helper`**
#[derive(Debug,Default)]
pub struct Helper {
    // info
    name: String,
    version: String,
    authors: Vec<(String, String)>, // (name,email)
    addrs: Vec<(String, String)>, // (addr_name,addr)
    desc: String,
    // env_vars
    current_exe: Option<String>,
    current_dir: Option<String>,
    home_dir: Option<String>,
    temp_dir: String,
    //  current_cmd
    current_cmd: Option<String>, //main is None
    // -v/--version, -h/--help
    ver: String, // "name version"
    helps: Map<Option<String>, String>, // None is main
}

impl Helper {
    /// name
    pub fn name(&self) -> &String {
        &self.name
    }
    /// version
    pub fn version(&self) -> &String {
        &self.version
    }
    /// description
    pub fn desc(&self) -> &String {
        &self.desc
    }
    /// name, email
    pub fn authors(&self) -> &Vec<(String, String)> {
        &self.authors
    }
    /// url_name, url
    pub fn addrs(&self) -> &Vec<(String, String)> {
        &self.addrs
    }
}

impl Helper {
    pub fn current_cmd(&self) -> Option<&String> {
        self.current_cmd.as_ref()
    }
    pub fn current_cmd_str(&self) -> Option<&str> {
        self.current_cmd.as_ref().map(|s| s.as_str())
    }
    pub fn current_cmd_ref(&self) -> &Option<String> {
        &self.current_cmd
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

impl Helper {
    /// `format!("{}  {}", self.name(), self.version())`
    pub fn ver(&self) -> &String {
        &self.ver
    }
    /// print ver(`self.ver`) message and exit with the `status`
    pub fn ver_exit(&self, status: i32) {
        println!("{}", self.ver().trim());
        exit(status);
    }
    /// `format!("ERROR:\n  {}\n\n", error)`
    pub fn err<E>(&self, error: E) -> String
        where E: AsRef<str> + Display
    {
        format!("ERROR:\n  {}\n\n", error)
    }
    /// print error(`self.err(error)`) message to `stderr` and exit with the `status`
    pub fn err_exit<E>(&self, error: E, status: i32)
        where E: AsRef<str> + Display
    {
        self.err_line_print(&self.err(error), ERROR_LINE_NUM);
        exit(status);
    }
    /// print error message line(2) with Red color(fg)
    #[inline]
    pub fn err_line_print(&self, msg: &str, line_num: usize) {
        for (i, line) in msg.trim().lines().enumerate() {
            if i == line_num {
                let mut t = term::stderr().unwrap();
                t.fg(term::color::RED).unwrap();
                writeln!(t, "{}", line).unwrap();
                t.reset().unwrap();
            } else {
                errln!("{}", line);
            }
        }
    }
    /// all Command's help message(`main` and `sub_cmd`)
    pub fn helps(&self) -> &Map<Option<String>, String> {
        &self.helps
    }
    /// main's help mesage
    pub fn help(&self) -> &String {
        &self.helps[&None]
    }
    /// print main's help message and exit with the `status`
    pub fn help_exit(&self, status: i32) {
        println!("{}", self.help().trim());
        exit(status);
    }
    /// `self.err(error) + self.help()`
    pub fn help_err<E>(&self, error: E) -> String
        where E: AsRef<str> + Display
    {
        self.err(error) + &self.helps[&None]
    }
    /// print error and help message(`self.help_err(error)`) to `stderr` and exit with the `status`
    pub fn help_err_exit<E>(&self, error: E, status: i32)
        where E: AsRef<str> + Display
    {
        self.err_line_print(&self.help_err(error), ERROR_LINE_NUM);
        exit(status);
    }
    /// get sub_command's help message
    pub fn help_cmd(&self, cmd_name: &Option<String>) -> &String {
        &self.helps[cmd_name]
    }
    /// print sub_command's help message and exit with the `status`
    pub fn help_cmd_exit(&self, cmd_name: &Option<String>, status: i32) {
        println!("{}", &self.helps[cmd_name].trim());
        exit(status);
    }
    /// `self.err(error) + self.help_cmd(cmd_name)`
    pub fn help_cmd_err<E>(&self, cmd_name: &Option<String>, error: E) -> String
        where E: AsRef<str> + Display
    {
        self.err(error) + &self.helps[cmd_name]
    }
    /// print error and sub_command's help message to `stderr`,s exit with the `status`
    pub fn help_cmd_err_exit<E>(&self, cmd_name: &Option<String>, error: E, status: i32)
        where E: AsRef<str> + Display
    {
        self.err_line_print(&self.help_cmd_err(cmd_name, error), ERROR_LINE_NUM);
        exit(status);
    }
    fn init_ver(&mut self, ver: String) {
        self.ver = ver;
    }
    fn init_help(&mut self, cmd_name: Option<String>, help: String) {
        if self.helps.insert(cmd_name.clone(), help).is_some() {
            panic!("{:?}'s help already insert", cmd_name);
        }
    }
}

impl<'app> App<'app> {
    /// name
    pub fn new<S>(name: S) -> Self
        where S: Into<String>
    {
        init!();
        let mut app = Self::default();
        app.helper.name = name.into();
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
    /// version
    pub fn version<S>(mut self, version: S) -> Self
        where S: Into<String>
    {
        self.helper.version = version.into();
        self
    }
    /// discription
    pub fn desc<'s: 'app>(mut self, desc: &'s str) -> Self {
        self.helper.desc = desc.to_string();
        self.main.desc = desc;
        self
    }
    /// name, email
    pub fn author<S>(mut self, name: S, email: S) -> Self
        where S: Into<String>
    {
        self.helper.authors.push((name.into(), email.into()));
        self
    }
    /// url_name, url
    pub fn addr<S>(mut self, name: S, url: S) -> Self
        where S: Into<String>
    {
        self.helper.addrs.push((name.into(), url.into()));
        self
    }
    /// add a `Opt`
    pub fn opt(mut self, opt: Opt<'app>) -> Self {
        self.main = self.main.opt(opt);
        self
    }
    /// get arguments, `App` will update the value
    pub fn args<'s: 'app, S>(mut self, name: S, value: &'s mut Vec<String>) -> Self
        where S: Into<String>
    {
        self.main.args = Some(value);
        self.main.args_name = name.into();
        self
    }
    /// set the `Cmd`s `args_optional` as `true`(default is `false`),
    ///
    /// `App` will will not check it's Args and create help message with tag of `optional` if it is `true`.
    pub fn args_optional(mut self) -> Self {
        self.main.args_optional = true;
        self
    }
    /// arguments's help message
    pub fn args_help<'s: 'app>(mut self, help: &'s str) -> Self {
        self.main.args_help = help;
        self
    }
    /// give a function let `App` to check Arguments
    pub fn args_check<Checker: StringsCheck>(mut self, checker: Checker) -> Self {
        self.main.args_check = checker.into_strings_check();
        self
    }
    /// add a sub_command
    pub fn cmd(mut self, mut cmd: Cmd<'app>) -> Self {
        let name = cmd.name.to_string();
        cmd = unsafe {
            cmd.opt(Opt::new("help", &mut HELP_SUBCMD)
                        .short("h")
                        .long("help")
                        .help("show the help message"))
        };
        if self.sub_cmds.insert(Some(name.clone()), cmd).is_some() {
            panic!("sub_command: \"{}\" already defined", name);
        }
        self
    }
    #[doc(hidden)]
    pub fn current_cmd<'s: 'app>(mut self, value: &'s mut Option<String>) -> Self {
        self.current_cmd = Some(value);
        self
    }
}
impl<'app> App<'app> {
    /// `parse(std::env::args()[1..])` and `exit(1)` if parse fails.
    pub fn parse_args(self) -> Helper {
        let args: Vec<String> = env::args().skip(1).collect();
        self.parse(&args[..])
    }
    /// `parse(&[String])` and `exit(1)` if parse fails.
    pub fn parse(mut self, args: &[String]) -> Helper {
        if let Err(e) = self.parse_strings(args) {
            if e == String::new() {
                panic!("App::parse_strings()->Err(String::new())");
            }
            self.helper
                .help_cmd_err_exit(self.helper.current_cmd_ref(), e, 1);
        }
        self.into_helper()
    }
    fn parse_strings(&mut self, args: &[String]) -> Result<(), String> {
        dbstln!("args: {:?}", args);
        {
            let ver = self.ver();
            self.helper.init_ver(ver);
            let help = self.help();
            self.helper.init_help(None, help);
        }
        for sub_cmd_name in self.sub_cmds.keys() {
            let help = self.help_cmd(sub_cmd_name);
            self.helper.init_help(sub_cmd_name.clone(), help);
        }
        self.helper.current_exe = env::current_exe()
            .map(|s| s.to_string_lossy().into_owned())
            .ok();
        self.helper.current_dir = env::current_dir()
            .map(|s| s.to_string_lossy().into_owned())
            .ok();
        self.helper.home_dir = env::home_dir().map(|s| s.to_string_lossy().into_owned());
        self.helper.temp_dir = env::temp_dir().to_string_lossy().into_owned();
        let mut idx = std::usize::MAX;
        {
            let commands: Vec<&Option<String>> = self.sub_cmds.keys().collect();
            'out: for (i, arg) in args.iter().enumerate() {
                for cmd in &commands {
                    let arg = Some(arg.to_string());
                    if arg == **cmd {
                        idx = i;
                        self.helper.current_cmd = arg;
                        break 'out;
                    }
                }
            }
        }
        if let Some(ref mut s) = self.current_cmd {
            **s = self.helper.current_cmd().cloned();
        }
        // -h/--help
        if let Some(s) = strings_idx(&args[..], "-h", "--help") {
            if idx != std::usize::MAX && idx < s {
                self.helper
                    .help_cmd_exit(self.helper.current_cmd_ref(), 0);
            } else {
                self.helper.help_exit(0);
            }
        }
        // -v/--version
        if let Some(s) = strings_idx(&args[..], "-v", "--version") {
            if idx >= s {
                self.helper.ver_exit(0);
            }
        }
        fn strings_idx(ss: &[String], msg0: &str, msg1: &str) -> Option<usize> {
            for (idx, arg) in ss.iter().enumerate() {
                if arg == msg0 || arg == msg1 {
                    return Some(idx);
                }
            }
            None
        }
        if idx != std::usize::MAX {
            self.main.parse(&args[0..idx])?;
            self.sub_cmds
                .get_mut(self.helper.current_cmd_ref())
                .unwrap()
                .parse(&args[idx + 1..])?;
        } else {
            self.main.parse(&args[..])?;
        }
        // check main.args
        if let Some(ref s) = self.main.args {
            if !self.main.args_optional && s.is_empty() {
                return Err(format!("Arguments({}) missing", self.main.args_name));
            }
        }
        // check main.opts
        for opt in self.main.opts.values() {
            if !opt.is_optional() {
                opt.value.as_ref().check(opt.name_get())?;
            }
        }
        // check current_cmd
        if self.helper.current_cmd().is_some() {
            let cmd = self.sub_cmds
                .get_mut(self.helper.current_cmd_ref())
                .unwrap();
            if let Some(ref s) = cmd.args {
                if !cmd.args_optional && s.is_empty() {
                    return Err(format!("Arguments({}) missing", cmd.args_name));
                }
            }
            for opt in cmd.opts.values() {
                if !opt.is_optional() {
                    opt.value.as_ref().check(opt.name_get())?;
                }
            }
        }
        // No input Args
        if args.is_empty() && self.main.args.is_none() {
            if self.sub_cmds.is_empty() {
                return Err("OPTION missing".to_owned());
            } else {
                return Err("OPTION/COMMAND missing".to_owned());
            }
        }
        // Args checker
        if !self.main.args_optional {
            if let Some(ref s) = self.main.args {
                self.main
                    .args_check
                    .call(&s[..], &self.main.args_name)?;
            }
        }
        if self.helper.current_cmd().is_some() {
            let cmd = self.sub_cmds
                .get_mut(self.helper.current_cmd_ref())
                .unwrap();
            if !cmd.args_optional {
                if let Some(ref s) = cmd.args {
                    cmd.args_check.call(&s[..], &cmd.args_name)?;
                }
            }
        }
        Ok(())
    }
    fn ver(&self) -> String {
        format!("{}  {}",
                self.helper.name.trim(),
                self.helper.version.trim())
    }
    // INFO
    fn help_info(&self) -> String {
        format!("INFO:\n  {} - {}\n  {}\n",
                self.helper.name.trim(),
                self.helper.version.trim(),
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
                             let default_or_optional = if v.is_optional() {
                                 OPTIONAL.to_owned()
                             } else {
                                 v.value
                                     .as_ref()
                                     .default()
                                     .map(|s| format!("[{}]", s))
                                     .unwrap_or_else(String::new)
                             };
                             dbstln!("GLOBAL--{}:  {:?}", k, default_or_optional);
                             let s = v.short_get().unwrap_or_else(String::new);
                             let long = v.long_get().unwrap_or_else(String::new);
                             let tmp_ = if v.is_bool() {
                                 if s != "" && long != "" {
                                     format!("   {}, {}  ", s, long)
                                 } else {
                                     format!("   {}{}  ", long, s)
                                 }
                             } else if s != "" && long != "" {
                        format!("   {}, {} <{}>{}  ", s, long, k, default_or_optional)
                    } else {
                        format!("   {}{} <{}>{}  ", s, long, k, default_or_optional)
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
    fn help_args(&self) -> String {
        let tmp_raw = "\nARGS:\n";
        let mut tmp = tmp_raw.to_string();
        let mut vs: Vec<(String, &str)> = Vec::new();
        let mut len = 0;
        if !self.main.args_help.is_empty() {
            let optional = if self.main.args_optional {
                OPTIONAL.to_owned()
            } else {
                String::new()
            };
            let tmp_main = format!("   <{}>{}  ", self.main.args_name, optional);
            len = tmp_main.len();
            vs.push((tmp_main, self.main.args_help))
        }
        for v in self.sub_cmds.values() {
            if !v.args_help.is_empty() {
                let optional = if self.main.args_optional {
                    OPTIONAL.to_owned()
                } else {
                    String::new()
                };
                let tmp_ = format!("   <{}>{}  ", v.args_name, optional);
                if tmp_.len() > len {
                    len = tmp_.len();
                }
                vs.push((tmp_, v.args_help));
            }
        }
        for (k, v) in vs {
            let mut tmp_ = k.clone();
            for _ in tmp_.len()..len {
                tmp_.push(' ');
            }
            tmp += &format!("{}  {}\n", tmp_, v.trim());
        }
        if tmp.as_str() == tmp_raw {
            String::new()
        } else {
            tmp
        }
    }
    fn help(&self) -> String {
        // INFO
        let mut help = self.help_info();
        //Author
        if !self.helper.authors.is_empty() {
            help += &{
                         let mut authors = String::new() + "\nAUTHOR:\n";
                         for &(ref author, ref email) in &self.helper.authors {
                             authors += &format!("  {} <{}>\n", author, email);
                         }
                         authors
                     };
        }
        //ADDRESS
        if !self.helper.addrs.is_empty() {
            help += &{
                         let mut authors = String::new() + "\nADDRESS:\n";
                         for &(ref author, ref email) in &self.helper.addrs {
                             authors += &format!("  {}: {}\n", author, email);
                         }
                         authors
                     };
        }
        //USAGE
        //    zipcs [global options] [global arguments] command [command options] [arguments...]
        help += &{
                     let mut tmp = format!("\nUSAGE:\n  {}", self.helper.name);
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
                             let tmp_ = format!("    {}  ", k.as_ref().unwrap());
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
        // args_help
        help += &self.help_args();
        help
    }
    fn help_cmd(&self, sub_cmd_name: &Option<String>) -> String {
        // INFO
        let mut help = self.help_info();
        //USAGE
        //    zipcs [global options] [global arguments] command [command options] [arguments...]
        help += &{
                     let mut tmp = format!("\nUSAGE:\n  {}", self.helper.name);
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
        let cmd = &self.sub_cmds[sub_cmd_name];
        if !cmd.opts.is_empty() {
            help += &{
                         let mut tmp = "\nOPTIONS:\n".to_owned();
                         let mut vs: Vec<(String, &str)> = Vec::new();
                         let mut len = 0;
                         for (k, v) in &cmd.opts {
                             let default_or_optional = if v.is_optional() {
                                 OPTIONAL.to_owned()
                             } else {
                                 v.value
                                     .as_ref()
                                     .default()
                                     .map(|s| format!("[{}]", s))
                                     .unwrap_or_else(String::new)
                             };
                             dbstln!("CMD_{:?}--{}:  {:?}", sub_cmd_name, k, default_or_optional);
                             let s = v.short_get().unwrap_or_else(String::new);
                             let long = v.long_get().unwrap_or_else(String::new);
                             let tmp_ = if v.is_bool() {
                                 if s != "" && long != "" {
                                     format!("    {}, {}  ", s, long)
                                 } else {
                                     format!("    {}{}  ", s, long)
                                 }
                             } else if s != "" && long != "" {
                        format!("    {}, {} <{}>{}  ", s, long, k, default_or_optional)
                    } else {
                        format!("    {}{} <{}>{}  ", s, long, k, default_or_optional)
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
        // args_help
        help += &self.help_args();
        help
    }
    pub fn helper(&self) -> &Helper {
        &self.helper
    }
    pub fn into_helper(self) -> Helper {
        self.helper
    }
}

/// **Command**
#[derive(Debug,Default)]
pub struct Cmd<'app> {
    name: &'app str,
    desc: &'app str,
    opts: Map<String, Opt<'app>>,
    str_to_name: Map<String, String>, //-short/--long to name
    args_name: String,
    args_help: &'app str,
    args: Option<&'app mut Vec<String>>,
    args_optional: bool,
    args_check: StringsChecker,
}
impl<'app> Cmd<'app> {
    /// name
    pub fn new<'s: 'app>(name: &'s str) -> Self {
        let mut c = Self::default();
        c.name = name;
        c
    }
    /// description
    pub fn desc<'s: 'app>(mut self, desc: &'s str) -> Self {
        self.desc = desc;
        self
    }
    /// get arguments
    pub fn args<'s: 'app, S>(mut self, name: S, value: &'s mut Vec<String>) -> Self
        where S: Into<String>
    {
        self.args = Some(value);
        self.args_name = name.into();
        self
    }
    /// set the `Cmd`s `args_optional` as `true`(default is `false`),
    ///
    /// `App` will will not check it's Args and create help message with tag of `optional` if it is `true`.
    pub fn args_optional(mut self) -> Self {
        self.args_optional = true;
        self
    }
    /// arguments's help message
    pub fn args_help<'s: 'app>(mut self, help: &'s str) -> Self {
        self.args_help = help;
        self
    }
    pub fn args_check<Checker: StringsCheck>(mut self, checker: Checker) -> Self {
        self.args_check = checker.into_strings_check();
        self
    }
    /// add `Opt`
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

/// **Option**
#[derive(Debug)]
pub struct Opt<'app> {
    name: &'app str,
    value: OptValue<'app>,
    optional: bool,
    short: Option<&'app str>,
    long: Option<&'app str>,
    help: &'app str,
}
impl<'app> Opt<'app> {
    ///**name and value, `App` will maintain the value(`&mut T`).**
    ///
    ///for example,
    ///
    ///* follows's charset is `Opt`'s Name
    ///
    ///* h, v and cs is `Opt`'s short
    ///
    ///* help, version and charset is `Opt`'s long
    ///
    ///* help is `Opt`'s help message
    ///
    ///```frs
    ///--charset charset,-cs charset         sets the charset Zipcs using
    ///--help,-h                             show the help message
    ///--version,-v                          show the version message
    ///```
    pub fn new<'s: 'app, V>(name: &'app str, value: V) -> Self
        where V: OptValueParse<'app>
    {
        Opt {
            value: value.into_opt_value(),
            name: name,
            optional: false,
            short: None,
            long: None,
            help: "",
        }
    }
    /// set `Opt`s optional as `true`(default is `false`),
    ///
    /// `App` will will not check it's value and create help message without default's value if it is `true`.
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
    /// short
    pub fn short(mut self, short: &'app str) -> Self {
        self.short = Some(short);
        self
    }
    /// long
    pub fn long(mut self, long: &'app str) -> Self {
        self.long = Some(long);
        self
    }
    /// help message
    pub fn help(mut self, help: &'app str) -> Self {
        self.help = help;
        self
    }
    fn parse(&mut self, msg: &str) -> Result<(), String> {
        let name = self.name_get().to_string();
        self.value.as_mut().parse(name, msg)
    }
}

impl<'app> Opt<'app> {
    pub fn value(&self) -> &'app OptValue {
        &self.value
    }
    pub fn value_mut(&mut self) -> &'app mut OptValue {
        &mut self.value
    }
    pub fn is_optional(&self) -> bool {
        self.optional
    }
    pub fn is_bool(&self) -> bool {
        self.value.as_ref().is_bool()
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

/// **`ArgumentsCheck`**
///
/// `&[String]` is the Arguments, `args_name` is args's name
pub struct StringsChecker {
    pub fn_: Box<Fn(&[String], &str) -> Result<(), String>>,
}

fn strings_checker_default(_: &[String], _: &str) -> Result<(), String> {
    Ok(())
}
impl Default for StringsChecker {
    fn default() -> StringsChecker {
        StringsChecker { fn_: Box::new(strings_checker_default) }
    }
}

impl fmt::Debug for StringsChecker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StringsCheck {{ fn_: _ }}")
    }
}

impl StringsChecker {
    pub fn call(&self, msg: &[String], args_name: &str) -> Result<(), String> {
        (*self.fn_)(msg, args_name)
    }
}

/// **You could check arguments and returns error message by a closure**
pub trait StringsCheck {
    fn into_strings_check(self) -> StringsChecker;
}

impl<F: Fn(&[String], &str) -> Result<(), String> + 'static> StringsCheck for F {
    fn into_strings_check(self) -> StringsChecker {
        StringsChecker { fn_: Box::from(self) }
    }
}
