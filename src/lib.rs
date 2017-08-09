/*!
# [app](https://github.com/biluohc/app)

## A easy-to-use command-line-parser written for Rust.

## Usage
Cargo.toml

```toml
    [dependencies]  
    app = "0.6.0" 
```
## Or 

```toml
    [dependencies]  
    app = { git = "https://github.com/biluohc/app",branch = "master", version = "0.6.0" }
```

## Documentation  
* Visit [Docs.rs](https://docs.rs/app/)  
or 
* Run `cargo doc --open` after modified the toml file.

## Examples
* [fht2p](https://github.com/biluohc/app/blob/master/examples/fht2p.rs): Options and Args

```rustful
    git clone https://github.com/biluohc/app
    cd app
    cargo run --example fht2p -- -h
```
* [cp](https://github.com/biluohc/app/blob/master/examples/cp.rs): Options and `Multi_Args` 

```rustful
    git clone https://github.com/biluohc/app
    cd app
    cargo run --example cp
```

* [zipcs](https://github.com/biluohc/app/blob/master/examples/zipcs.rs): `Sub_Commands, OptValue and OptValueParse`

```rustful
    git clone https://github.com/biluohc/app
    cd app
    cargo run --example zipcs
```
* [http](https://github.com/biluohc/app/blob/master/examples/http.rs): Custom `Helps`

```rustful
    git clone https://github.com/biluohc/app
    cd app
    cargo run --example http
```
*/

#[macro_use]
extern crate stderr;
extern crate term;
#[macro_use]
extern crate quick_error;
include!("help.rs");
include!("render.rs");
include!("elesref.rs");
include!("error.rs");
mod ovp;
pub use ovp::{OptValue, OptValueParse};
mod avp;
pub use avp::{ArgsValue, ArgsValueParse};

use std::collections::BTreeMap as Map;
use std::default::Default;
use std::io::prelude::*;
use std::process::exit;
use std::fmt::Display;
use std::env;

const ERROR_LINE_NUM: usize = 1; // for print error with color(Red)
static mut HELP: bool = false;
static mut HELP_SUBCMD: bool = false;
static mut VERSION: bool = false;
static OPTIONAL: &'static str = "(optional)";
static OPT_HELP_SORT_KEY: &'static str ="___app_internal0";
static OPT_VERSION_SORT_KEY: &'static str = "___app_internal1";

/// **Application**
#[derive(Debug,Default)]
pub struct App<'app> {
    // None is main
    cmds: Map<Option<String>, Cmd<'app>>, // key, Cmd
    str_to_key: Map<String,Option<String>>, // cmd/cmd_short, key
    pub helper: Helper,
}

impl<'app> App<'app> {
    /// name
    pub fn new<S>(name: S) -> Self
        where S: Into<String>
    {
        logger_init!();
        let mut app = Self::default();
        app.helper.name = name.into();
        app.cmds
            .insert(None,
                    Cmd::default()
                        .add_help(unsafe { &mut HELP })
                        .add_version()
                        .allow_zero_args(true));
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
        {
            let mut main = self.cmds.get_mut(&None).unwrap();
            main.desc = desc;
        }
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
        {
            let mut main = self.cmds.remove(&None).unwrap();
            main = main.opt(opt);
            self.cmds.insert(None, main);
        }
        self
    }
    /// get arguments
    pub fn args(mut self, args: Args<'app>) -> Self {
        {
            let mut main = self.cmds.get_mut(&None).unwrap();
            main.args.push(args);
        }
        self
    }
    /// add a sub_command
    pub fn cmd(mut self, cmd: Cmd<'app>) -> Self {
        let name = cmd.name.map(|s| s.to_string());
        let short = cmd.short.map(|s| s.to_string());
        let key = cmd.sort_key.map(|s| s.to_string());
        if self.str_to_key.insert(name.clone().unwrap(),key.clone()).is_some() {
            panic!("Cmd: \"{:?}\" already defined",name.as_ref().unwrap());
        }
        if short.is_some() {
            if self.str_to_key.insert(short.clone().unwrap(),key.clone()).is_some() {
            panic!("Cmd's short: \"{:?}\" already defined",short.as_ref().unwrap());
        }
        }
        if self.cmds.insert(key.clone(), cmd).is_some() {
            panic!("Cmd(or it's sort_key): \"{:?}\" already defined",key);
        }
        self
    }
    /// allow `env::args().count() == 1`
    ///
    /// deafult: true
    pub fn allow_zero_args(mut self, allow: bool) -> Self {
        self.cmds
            .get_mut(&None)
            .map(|main| main.allow_zero_args = allow)
            .unwrap();
        self
    }
}
impl<'app> App<'app> {
    /// build `Helper` for custom `Helps`
    ///
    /// You can modify `app.helper.helps.xxx`
    pub fn build_helper(mut self) -> Self {
        self._build_helper();
        self
    }
    /// `parse(std::env::args()[1..])` and `exit(1)` if parse fails.
    pub fn parse_args(self) -> Helper {
        let args: Vec<String> = env::args().skip(1).collect();
        self.parse(&args[..])
    }
    /// `parse(&[String])` and `exit(1)` if parse fails.
    pub fn parse(mut self, args: &[String]) -> Helper {
        if let Err(e) = self.parse_strings(args) {
            match e {
                AppError::Parse(s) => {
                    assert_ne!("",
                               s.trim(),
                               "App::parse_strings()->Err(AppError::Parse(String::new()))");
                    self.helper
                        .help_cmd_err_exit(self.helper.current_cmd_ref(), s, 1);
                }
                AppError::Help(s) => {
                    assert_ne!(Some(""),
                               s.as_ref().map(|s| s.as_str()),
                               "App::parse_strings()->Err(AppError::Help(String::new()))");
                    self.helper.help_cmd_exit(&s, 0);
                }
                AppError::Version => {
                    self.helper.ver_exit(0);
                }
            }
        }
        self.into_helper()
    }
    fn parse_strings(&mut self, args: &[String]) -> Result<(), AppError> {
        dbln!("args: {:?}", args);
        self._build_helper();
        self.helper.current_exe = env::current_exe()
            .map(|s| s.to_string_lossy().into_owned())
            .ok();
        self.helper.current_dir = env::current_dir()
            .map(|s| s.to_string_lossy().into_owned())
            .ok();
        self.helper.home_dir = env::home_dir().map(|s| s.to_string_lossy().into_owned());
        self.helper.temp_dir = env::temp_dir().to_string_lossy().into_owned();
        let mut idx = std::usize::MAX; // cmd_idx
        {
             for (i, arg) in args.iter().enumerate() {
                if let Some(a)= self.str_to_key.get(arg) {
                        idx = i;
                        self.helper.current_cmd = self.cmds[&a].name.map(|s|s.to_string());                        
                        self.helper.current_cmd_sort_key = a.clone();
                        break ;
                }
            }
        }
        // -h/--help
        if let Some(s) = strings_idx(&args[..], "-h", "--help") {
            if idx != std::usize::MAX && idx < s {
                self.helper.current_cmd_ref().to_app_rest()?;
            } else {
                let none: Option<String> = None;
                none.to_app_rest()?;
            }
        }
        // -v/--version
        if let Some(s) = strings_idx(&args[..], "-V", "--version") {
            if idx >= s {
                return Err(AppError::Version);
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
            self.cmds.get_mut(&None).unwrap().parse(&args[0..idx])?;
            self.cmds
                .get_mut(&self.helper.current_cmd_sort_key)
                .unwrap()
                .parse(&args[idx + 1..])?;
        } else {
            self.cmds.get_mut(&None).unwrap().parse(&args[..])?;
        }
        // check main
        self.check(&None)?;
        // check current_cmd
        if self.helper.current_cmd_sort_key.is_some() {
            self.check(&self.helper.current_cmd_sort_key)?;
        }
        // check allow_zero_args
        let cmd = &self.cmds[&self.helper.current_cmd_sort_key];
        if !cmd.allow_zero_args && self.cmds.len() > 1 && self.helper.current_cmd.is_none() {
            Err(AppError::Parse("OPTION/COMMAND missing".to_owned()))
        } else if !cmd.allow_zero_args {
            Err(AppError::Parse("OPTION missing".to_owned()))
        } else {
            Ok(())
        }
    }
    // check Cmd's Opts and Args
    fn check(&self, cmd_key: &Option<String>) -> Result<(), String> {
        let cmd = &self.cmds[cmd_key];
        // Opt
        for opt in cmd.opts.values() {
            if !opt.is_optional() {
                opt.value.as_ref().check(opt.name_get())?;
            }
        }
        // Args
        for args_ in &cmd.args {
            if !args_.optional {
                args_.value.as_ref().check(args_.name_get())?;
            }
        }
        Ok(())
    }
    pub fn into_helper(self) -> Helper {
        self.helper
    }
}

/// **Command**
#[derive(Debug,Default)]
pub struct Cmd<'app> {
    name: Option<&'app str>,
    short: Option<&'app str>,
    sort_key: Option<&'app str>,
    desc: &'app str,
    opts: Map<String, Opt<'app>>, // key to Opt
    str_to_key: Map<String, String>, //-short/--long to key
    args: Vec<Args<'app>>,
    allow_zero_args: bool,
}
impl<'app> Cmd<'app> {
    /// `default` and add `-h/--help` `Opt`
    fn add_help(self, b: &'static mut bool) -> Self {
        self.opt(Opt::new("help", b)
                    .sort_key(OPT_HELP_SORT_KEY)
                     .short("h")
                     .long("help")
                     .help("Show the help message"))
    }
    /// add `-v/version` `Opt`
    fn add_version(self) -> Self {
        self.opt(Opt::new("version", unsafe { &mut VERSION })
                    .sort_key(OPT_VERSION_SORT_KEY)
                     .short("V")
                     .long("version")
                     .help("Show the version message"))

    }
    /// name and add `-h/--help`
    pub fn new<'s: 'app>(name: &'s str) -> Self {
        let mut c = Self::default();
        c.allow_zero_args = true;
        c.name = Some(name.clone());
        c.sort_key = Some(name);
        c.add_help(unsafe { &mut HELP_SUBCMD })
    }
    pub fn short<'s: 'app>(mut self,short: &'s str) -> Self {
        self.short=Some(short);
        self
    }
    /// Default is `Cmd`'s name
    pub fn sort_key(mut self,sort_key: &'app str)->Self {
        self.sort_key =Some(sort_key);
        self
    }
    /// description
    pub fn desc<'s: 'app>(mut self, desc: &'s str) -> Self {
        self.desc = desc;
        self
    }
    /// get argument
    pub fn args(mut self, args: Args<'app>) -> Self {
        self.args.push(args);
        self
    }
    /// add `Opt`
    pub fn opt(mut self, opt: Opt<'app>) -> Self {
        let long = opt.long_get();
        let short = opt.short_get();
        let name = opt.name_get();
        let key = opt.sort_key_get().to_string();        
        if long.is_none() && short.is_none() {
            panic!("OPTION: \"{}\" don't have --{} and -{} all",
                   name,
                   name,
                   name);
        }
        if let Some(ref s) = long {
            if self.str_to_key.insert(s.clone(), key.clone()).is_some() {
                panic!("long: \"{}\" already defined", s);
            }
        }
        if let Some(ref s) = short {
            if self.str_to_key.insert(s.clone(), key.clone()).is_some() {
                panic!("short: \"{}\" already defined", s);
            }
        }
        if self.opts.insert(key.clone(), opt).is_some() {
            panic!("Opt(or it's sort_key): \"{}\" already defined", key);
        }
        self
    }
    /// default: true
    pub fn allow_zero_args(mut self, allow: bool) -> Self {
        self.allow_zero_args = allow;
        self
    }
    fn parse(&mut self, args: &[String]) -> Result<(), String> {
        let mut args_vec: Vec<String> = Vec::new();
        let mut i = 0;
        for _ in 0..args.len() {
            if i >= args.len() {
                break;
            }
            let arg = &args[i];
            // println!("i+1/args_len: {}/{}: {:?}", i + 1, args.len(), &args[i..]);
            match arg {
                s if s.starts_with("--") && s != "--" => {
                    if let Some(opt_key) = self.str_to_key.get(s.as_str()) {
                        let mut opt = self.opts.get_mut(opt_key).unwrap();
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
                s if s.starts_with('-') && s != "-" && s != "--" => {
                    if let Some(opt_key) = self.str_to_key.get(s.as_str()) {
                        let mut opt = self.opts.get_mut(opt_key).unwrap();
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
                    args_vec.push(s.to_string());
                    i += 1;
                }
            }
        }
        args_handle(&mut self.args, &args_vec[..])?;
        Ok(())
    }
}
fn args_handle(args: &mut [Args], argstr: &[String]) -> Result<(), String> {
    let mut argstr_used_len = 0;
    for a in args.iter() {
        let a_len = if !a.is_optional() && a.value.as_ref().default().is_none() {
            a.len.unwrap_or(1)
        } else {
            0
        };
        dbln!("Args_len/argstr_len/argstr_used_len/a_len: {}/{}/{}/{}",
              args.len(),
              argstr.len(),
              argstr_used_len,
              a_len);
        if argstr_used_len == argstr.len() && a_len != 0 {
            dbln!("argstr_used_len == argstr.len() && a_len != 0");
            return Err(format!("Args({}) no provide", a.name_get()));
        } else if argstr_used_len + a_len > argstr.len() {
            dbln!("argstr_used_len + a_len > argstr.len()");
            return Err(format!("Args({}) no provide enough: {:?}",
                               a.name_get(),
                               &argstr[argstr_used_len..]));
        }
        argstr_used_len += a_len;
    }
    args_rec(args, ElesRef::new(argstr))?;
    Ok(())
}

#[allow(unknown_lints,needless_range_loop)]
fn args_rec(args: &mut [Args], mut argstr: ElesRef<String>) -> Result<(), String> {
    if args.is_empty() && argstr.is_empty() {
        return Ok(());
    }
    if args.is_empty() && !argstr.is_empty() {
        let e = format!("Args: \"{:?}\" no need", argstr.as_slice());
        return Err(e);
    }
    if !args.is_empty() && argstr.is_empty() {
        for idx in 0..args.len() {
            if !args[idx].is_optional() && args[idx].value.as_ref().default().is_none() {
                let e = format!("Args({}) no provide", args[idx].name_get());
                return Err(e);
            }
        }
    }
    if let Some(len) = args[0].len {
        if len <= argstr.len() {
            args[0].parse(argstr.slice(0..len))?;
            dbln!("Some(len): {} {:?} + {:?}",
                  len,
                  argstr.slice(0..len),
                  argstr.slice(len..));
            argstr.cut(len..);
            args_rec(&mut args[1..], argstr)?;
        } else if args[0].is_optional() {
            args[0].parse(argstr.as_slice())?;
        } else {
            let e = format!("Args({}): \"{:?}\" no provide enough",
                            args[0].name_get(),
                            argstr.as_slice());
            return Err(e);
        }
    } else if args.len() > 1 {
        argstr.reverse();
        args.reverse();
        dbln!("len()>1:\nRaw: {:?}\nslice: {}", argstr, argstr);
        args_rec(args, argstr)?;
    } else {
        args[0].parse(argstr.as_slice())?;
    }
    Ok(())
}

/// **Option**
#[derive(Debug)]
pub struct Opt<'app> {
    name: &'app str,
    sort_key:&'app str,
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
            value: value.into(),
            name: name.clone(),
            sort_key:name,
            optional: false,
            short: None,
            long: None,
            help: "",
        }
    }
    /// Default is `Opt`'s name
    pub fn sort_key(mut self,sort_key: &'app str)->Self {
        self.sort_key = sort_key;
        self
    }
    /// set `Opt`s optional as `true`(default is `false`)(override `OptValueParse`'s `default` and `check`).
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
    pub fn sort_key_get(&self) -> &'app str {
        self.sort_key
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


/// **Args**
#[derive(Debug)]
pub struct Args<'app> {
    name: &'app str,
    value: ArgsValue<'app>,
    optional: bool,
    len: Option<usize>, // default have not limit
    help: &'app str,
}
impl<'app> Args<'app> {
    pub fn new<'s: 'app, V>(name: &'app str, value: V) -> Self
        where V: ArgsValueParse<'app>
    {
        Args {
            name: name,
            value: value.into(),
            optional: false,
            len: None,
            help: "",
        }
    }
    pub fn len<L: Into<usize>>(mut self, len: L) -> Self {
        self.len = Some(len.into());
        self
    }
    /// set `Args`s optional as `true`(default is `false`)(override `ArgsValueParse`'s `default` and `check`).
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
    /// help message
    pub fn help(mut self, help: &'app str) -> Self {
        self.help = help;
        self
    }
    fn parse(&mut self, msg: &[String]) -> Result<(), String> {
        let name = self.name_get().to_string();
        self.value.as_mut().parse(&name, msg)
    }
}

impl<'app> Args<'app> {
    pub fn value(&self) -> &'app ArgsValue {
        &self.value
    }
    pub fn value_mut(&mut self) -> &'app mut ArgsValue {
        &mut self.value
    }
    pub fn is_optional(&self) -> bool {
        self.optional
    }
    pub fn name_get(&self) -> &'app str {
        self.name
    }
    pub fn help_get(&self) -> &str {
        self.help
    }
}
