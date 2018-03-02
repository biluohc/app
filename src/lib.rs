/*!
# [app](https://github.com/biluohc/app)

## A easy-to-use command-line-parser written for Rust.

## Usage
Cargo.toml

```toml
    [dependencies]  
    app = "0.6.5" 
```
## Or 

```toml
    [dependencies]  
    app = { git = "https://github.com/biluohc/app",branch = "master", version = "0.6.5" }
```

## Documentation  
* Visit [Docs.rs](https://docs.rs/app/)  

Or

* Run `cargo doc --open` after modified the toml file.

## Examples

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
*/

#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate stderr;
extern crate term;
use term::color::Color;

include!("help.rs");
include!("render.rs");
include!("elesref.rs");
include!("error.rs");
mod ovp;
pub use ovp::{OptValue, OptValueParse};
mod avp;
pub use avp::{ArgsValue, ArgsValueParse};
/// Mut Statics
pub mod statics;

use std::collections::BTreeMap as Map;
use std::default::Default;
use std::io::prelude::*;
use std::process::exit;
use std::fmt::Display;
use std::path::PathBuf;
use std::env;

static mut HELP: bool = false;
static mut HELP_SUBCMD: bool = false;
static mut VERSION: bool = false;

/// **Application**
#[derive(Debug, Default)]
pub struct App<'app> {
    // None is main
    cmds: Map<Option<String>, Cmd<'app>>,    // key, Cmd
    str_to_key: Map<String, Option<String>>, // cmd/cmd_short, key
    helper: Helper,
}

/// A help function for `App`
pub fn app<S>(name: S, version: S, desc: &str) -> App
where
    S: Into<String>,
{
    App::new(name).version(version).desc(desc)
}

impl<'app> App<'app> {
    /// name
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        logger_init!();
        let mut app = Self::default();
        app.helper.name = name.into();
        app.cmds.insert(
            None,
            Cmd::default()
                .add_help(unsafe { &mut HELP })
                .add_version()
                .allow_zero_args(true),
        );
        app
    }
    /// version
    pub fn version<S>(mut self, version: S) -> Self
    where
        S: Into<String>,
    {
        self.helper.version = version.into();
        self
    }
    /// discription
    pub fn desc<'s: 'app>(mut self, desc: &'s str) -> Self {
        self.helper.desc = desc.to_string();
        self.cmds
            .get_mut(&None)
            .map(|main| main.desc = desc)
            .unwrap();
        self
    }
    /// name, email
    pub fn author<S>(mut self, name: S, email: S) -> Self
    where
        S: Into<String>,
    {
        self.helper.authors.push((name.into(), email.into()));
        self
    }
    /// url_name, url
    pub fn addr<S>(mut self, name: S, url: S) -> Self
    where
        S: Into<String>,
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
        self.cmds
            .get_mut(&None)
            .map(|main| main.args.push(args))
            .unwrap();
        self
    }
    /// add a sub_command
    pub fn cmd(mut self, cmd: Cmd<'app>) -> Self {
        let name = cmd.name.map(|s| s.to_string());
        let short = cmd.short.map(|s| s.to_string());
        let key = cmd.sort_key.map(|s| s.to_string());
        if self.str_to_key
            .insert(name.clone().unwrap(), key.clone())
            .is_some()
        {
            panic!("Cmd: \"{:?}\" already defined", name.as_ref().unwrap());
        }
        if short.is_some()
            && self.str_to_key
                .insert(short.clone().unwrap(), key.clone())
                .is_some()
        {
            panic!(
                "Cmd's short: \"{:?}\" already defined",
                short.as_ref().unwrap()
            );
        }
        if self.cmds.insert(key.clone(), cmd).is_some() {
            panic!("Cmd(or it's sort_key): \"{:?}\" already defined", key);
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
    /// You can modify `Helps.xxx` by `app.as_mut_helps()`
    pub fn build_helper(mut self) -> Self {
        self._build_helper();
        self
    }
    pub fn as_mut_helps(&mut self) -> &mut Helps {
        &mut self.helper.helps
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
                    assert_ne!(
                        "",
                        s.trim(),
                        "App::parse_strings()->Err(AppError::Parse(String::new()))"
                    );
                    self.helper
                        .help_cmd_err_exit(self.helper.current_cmd_ref(), s, 1);
                }
                AppError::Help(s) => {
                    assert_ne!(
                        Some(""),
                        s.as_ref().map(|s| s.as_str()),
                        "App::parse_strings()->Err(AppError::Help(String::new()))"
                    );
                    self.helper.help_cmd_exit(&s, 0);
                }
                AppError::Version => {
                    self.helper.ver_exit(0);
                }
            }
        }
        self.into_helper()
    }
    pub fn parse_strings(&mut self, args: &[String]) -> Result<(), AppError> {
        dbln!("parse_strings(): {:?}", args);
        self._build_helper();
        self.helper.args_len = args.len();
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
                if let Some(a) = self.str_to_key.get(arg) {
                    idx = i;
                    self.helper.current_cmd = self.cmds[a].name.map(|s| s.to_string());
                    self.helper.current_cmd_sort_key = a.clone();
                    break;
                }
            }
        }
        // -h/--help
        if let Some(s) = strings_idx(&args[..], 'h', "--help") {
            if idx != std::usize::MAX && idx < s {
                self.helper.current_cmd_ref().to_app_rest()?;
            } else {
                let none: Option<String> = None;
                none.to_app_rest()?;
            }
        }
        // -v/--version
        if let Some(s) = strings_idx(&args[..], 'V', "--version") {
            if idx >= s {
                return Err(AppError::Version);
            }
        }
        fn strings_idx(ss: &[String], msg0: char, msg1: &str) -> Option<usize> {
            for (idx, arg) in ss.iter().enumerate() {
                if flag_contains(arg, &msg0) || arg == msg1 {
                    return Some(idx);
                }
            }
            #[inline]
            fn flag_contains(arg: &str, flag: &char) -> bool {
                if arg.starts_with('-') && !arg.starts_with("--") {
                    for s in arg.chars() {
                        if s == *flag {
                            return true;
                        }
                    }
                }
                false
            }
            None
        }
        let app_has_subcmds = self.cmds.len() > 1;
        if idx != std::usize::MAX {
            self.cmds
                .get_mut(&None)
                .unwrap()
                .parse(&args[0..idx], &app_has_subcmds)?;
            self.cmds
                .get_mut(&self.helper.current_cmd_sort_key)
                .unwrap()
                .parse(&args[idx + 1..], &app_has_subcmds)?;
        } else {
            self.cmds
                .get_mut(&None)
                .unwrap()
                .parse(&args[..], &app_has_subcmds)?;
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
            opt.check()?;
        }
        // Args
        for args_ in &cmd.args {
            args_.check()?;
        }
        Ok(())
    }
    pub fn into_helper(self) -> Helper {
        self.helper
    }
}

/** 
## About Cargo

If the binary being calling as `subcommand` by `cargo`,

You should call `fix_helps_for_cargo()` and use `parse_args_for_cargo()` to replace `parse_args()`.

You could see the example: [cargo-http](https://github.com/biluohc/app/blob/master/examples/cargo-http.rs)

## Notice:

If Your `App`'s name is `xxx`, Your `crate`'s name have to name like `cargo-xxx`,

and you can install it by `cargo intsall`,

you can call it by `cargo xxx` or `cargo-xxx`(If it's path in the path environment variable, but is not recommended),

if you want to call it by `xxx`, you can use `ln` or `cp` command.

```sh
ln -s $HOME/.cargo/bin/cargo-xxx  $HOME/.cargo/bin/xxx
```
or

```sh
sudo ln -s $HOME/.cargo/bin/cargo-xxx  /usr/bin/xxx
```
*/
impl<'app> App<'app> {
    /// This function is only verified on Linux/Windows(cargo-V0.23.0) currently.
    pub fn as_cargo_subcmd() -> bool {
        let cargo_home_bin = env::var("CARGO_HOME").map(PathBuf::from).map(|mut p| {
            p.push("bin");
            p
        });
        let current_exe_dir = env::current_exe().map(|mut s| {
            s.pop();
            s
        });
        dbln!("$CARGO_HOME: {:?}", cargo_home_bin);
        dbln!("$current_exe_dir: {:?}", current_exe_dir);
        cargo_home_bin
            .map(|sc| current_exe_dir.map(|se| se == sc))
            .map(|ob| ob.unwrap_or_default())
            .unwrap_or_default()
    }
    pub fn fix_helps_for_cargo(&mut self) {
        self.as_mut_helps().version.insert_str(0, "cargo-");
        self.as_mut_helps()
            .cmd_infos
            .values_mut()
            .map(|v| v.insert_str(0, "cargo-"))
            .count();
        self.as_mut_helps()
            .cmd_usages
            .get_mut(&None)
            .map(|s| {
                let prefix_blanks_ends_idx = |s: &str| {
                    let mut len = 0;
                    for c in s.chars() {
                        if c == ' ' {
                            len += 1;
                        } else {
                            break;
                        }
                    }
                    len
                };
                let mut usage = String::default();
                for (idx, ss) in s.lines().enumerate() {
                    if idx == 0 {
                        usage.push_str(ss);
                    } else {
                        usage.push('\n');
                        usage.push_str(&ss[..prefix_blanks_ends_idx(ss)]);
                        usage.push_str("cargo ");
                        usage.push_str(&ss[prefix_blanks_ends_idx(ss)..]);
                    }
                }
                dbln!("{}", usage);
                *s = usage;
            })
            .unwrap()
    }
    /// `parse(std::env::args()[2..])` and `exit(1)` if parse fails.
    pub fn parse_args_for_cargo(self) -> Helper {
        let args: Vec<String> = env::args().skip(2).collect();
        self.parse(&args[..])
    }
}
/// **Command**
#[derive(Debug, Default)]
pub struct Cmd<'app> {
    name: Option<&'app str>,
    short: Option<&'app str>,
    sort_key: Option<&'app str>,
    desc: &'app str,
    opts: Map<String, Opt<'app>>,    // key to Opt
    str_to_key: Map<String, String>, //-short/--long to key
    args: Vec<Args<'app>>,
    allow_zero_args: bool,
}
impl<'app> Cmd<'app> {
    /// `default` and add `-h/--help` `Opt`
    fn add_help(self, b: &'static mut bool) -> Self {
        self.opt(
            Opt::new("help", b)
                .sort_key(statics::opt_help_sort_key_get())
                .short('h')
                .long("help")
                .help("Show the help message"),
        )
    }
    /// add `-v/version` `Opt`
    fn add_version(self) -> Self {
        self.opt(
            Opt::new("version", unsafe { &mut VERSION })
                .sort_key(statics::opt_version_sort_key_get())
                .short('V')
                .long("version")
                .help("Show the version message"),
        )
    }
    /// name and add `-h/--help`
    pub fn new<'s: 'app>(name: &'s str) -> Self {
        let mut c = Self::default();
        c.allow_zero_args = true;
        c.name = Some(name);
        c.sort_key = Some(name);
        c.add_help(unsafe { &mut HELP_SUBCMD })
    }
    pub fn short<'s: 'app>(mut self, short: &'s str) -> Self {
        self.short = Some(short);
        self
    }
    /// Default is `Cmd`'s name
    pub fn sort_key(mut self, sort_key: &'app str) -> Self {
        self.sort_key = Some(sort_key);
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
            panic!(
                "OPTION: \"{}\" don't have --{} and -{} all",
                name, name, name
            );
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
    fn parse(&mut self, args: &[String], app_has_subcmds: &bool) -> Result<(), String> {
        let mut args_vec: Vec<String> = Vec::new();
        let mut i = 0;
        for _ in 0..args.len() {
            if i >= args.len() {
                break;
            }
            let arg = &args[i];
            dbln!("i+1/args_len: {}/{}: {:?}", i + 1, args.len(), &args[i..]);
            match arg {
                s if s.starts_with("--") && s != "--" => {
                    if let Some(opt_key) = self.str_to_key.get(s.as_str()) {
                        let opt = self.opts.get_mut(opt_key).unwrap();
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
                        return Err(format!("OPTION: {:?} is undefined", s));
                    }
                }
                s if s.starts_with('-') && s != "-" => {
                    if s.chars().count() > 2 {
                        let flags: Vec<String> = s[1..].chars().map(|c| format!("-{}", c)).collect();
                        let mut last_flag_is_not_bool = false;
                        for idx in 0..flags.len() {
                            if let Some(opt_key) = self.str_to_key.get(flags[idx].as_str()) {
                                let opt = self.opts.get_mut(opt_key).unwrap();
                                let opt_is_bool = opt.is_bool();
                                if !opt_is_bool && args.len() > i + 1 && idx + 1 == flags.len() {
                                    opt.parse(&args[i + 1])?;
                                    last_flag_is_not_bool = true;
                                } else if opt_is_bool {
                                    opt.parse("")?;
                                } else {
                                    return Err(format!("OPTION({})'s value missing", flags[idx]));
                                }
                            } else {
                                return Err(format!("OPTION: {:?} is undefined", flags[idx]));
                            }
                        }
                        if last_flag_is_not_bool {
                            i += 2;
                        } else {
                            i += 1;
                        }
                    } else if let Some(opt_key) = self.str_to_key.get(s.as_str()) {
                        let opt = self.opts.get_mut(opt_key).unwrap();
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
                        return Err(format!("OPTION: {:?} is undefined", s));
                    }
                }
                s => {
                    args_vec.push(s.to_string());
                    i += 1;
                }
            }
        }
        if self.name.is_none() && *app_has_subcmds && self.args.is_empty() && !args_vec.is_empty() {
            return Err(format!("Command: {:?} is undefined", args_vec[0]));
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
        dbln!(
            "Args_len/argstr_len/argstr_used_len/a_len: {}/{}/{}/{}",
            args.len(),
            argstr.len(),
            argstr_used_len,
            a_len
        );
        if argstr_used_len == argstr.len() && a_len != 0 {
            dbln!("argstr_used_len == argstr.len() && a_len != 0");
            return Err(format!("Args(<{}>) not provide", a.name_get()));
        } else if argstr_used_len + a_len > argstr.len() {
            dbln!("argstr_used_len + a_len > argstr.len()");
            return Err(format!(
                "Args(<{}>) not provide enough: {:?}",
                a.name_get(),
                &argstr[argstr_used_len..]
            ));
        }
        argstr_used_len += a_len;
    }
    args_rec(args, ElesRef::new(argstr))?;
    Ok(())
}

#[allow(unknown_lints, needless_range_loop)]
fn args_rec(args: &mut [Args], mut argstr: ElesRef<String>) -> Result<(), String> {
    if args.is_empty() && argstr.is_empty() {
        return Ok(());
    }
    if args.is_empty() && !argstr.is_empty() {
        let e = format!("Args: \"{:?}\" not need", argstr.as_slice());
        return Err(e);
    }
    if !args.is_empty() && argstr.is_empty() {
        for idx in 0..args.len() {
            if !args[idx].is_optional() && args[idx].value.as_ref().default().is_none() {
                let e = format!("Args(<{}>) not provide", args[idx].name_get());
                return Err(e);
            }
        }
    }
    if let Some(len) = args[0].len {
        if len <= argstr.len() {
            args[0].parse(argstr.slice(0..len))?;
            dbln!(
                "Some(len): {} {:?} + {:?}",
                len,
                argstr.slice(0..len),
                argstr.slice(len..)
            );
            argstr.cut(len..);
            args_rec(&mut args[1..], argstr)?;
        } else if args[0].is_optional() {
            args[0].parse(argstr.as_slice())?;
        } else {
            let e = format!(
                "Args(<{}>): \"{:?}\" not provide enough",
                args[0].name_get(),
                argstr.as_slice()
            );
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

///**`OptionType`**
///
/// You should ignore `OptTypo` if the `Opt` is a flag(trait `OptValueParse`: `is_bool`).
#[derive(Debug, Clone, PartialEq)]
pub enum OptTypo {
    ///`App` will exit if the `Opt` occurs for the second time.
    Single,
    ///`App` will ignore all values of the `Opt` except the first time.
    Ignored,
    ///`Default`: `App` will overwrite the previous value with a later value.
    Covered,
    ///`App` will accumulate all values of the `Opt` if the `Opt`'s value is `Vec<T>` or `&mut [T]`, otherwise it equal to `Covered`(default).
    ///
    ///The value is the length set for `Vec<T>` or `&[T]`, default is `None` or the length of `&mut [T]`.
    Multiple(Option<usize>),
}
impl OptTypo {
    pub fn is_single(&self) -> bool {
        match *self {
            OptTypo::Single => true,
            _ => false,
        }
    }
    pub fn is_ignored(&self) -> bool {
        match *self {
            OptTypo::Ignored => true,
            _ => false,
        }
    }
    pub fn is_covered(&self) -> bool {
        match *self {
            OptTypo::Covered => true,
            _ => false,
        }
    }
    pub fn is_multiple(&self) -> bool {
        match *self {
            OptTypo::Multiple(_) => true,
            _ => false,
        }
    }
    /// If it not a `OptTypo::Multiple(_)`, will panic
    pub fn multiple_get(&self) -> Option<&usize> {
        match *self {
            OptTypo::Multiple(ref v) => v.as_ref(),
            _ => panic!("Unwrap OptTypo but it's not a Multiple: {:?}", self),
        }
    }
    /// Set it as `OptTypo::Multiple(_)`
    pub fn set_multiple(&mut self, len: Option<usize>) {
        if !self.is_multiple() {
            *self = OptTypo::Multiple(None);
        }
        match *self {
            OptTypo::Multiple(ref mut v) => *v = len,
            _ => unreachable!(),
        }
    }
}
impl Default for OptTypo {
    fn default() -> Self {
        OptTypo::Covered
    }
}
/// **Option**
#[derive(Debug)]
pub struct Opt<'app> {
    name: &'app str,
    sort_key: &'app str,
    value: OptValue<'app>,
    optional: bool,
    short: Option<char>,
    long: Option<&'app str>,
    help: &'app str,
    count: usize,
    typo: OptTypo,
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
    where
        V: OptValueParse<'app>,
    {
        Opt {
            value: value.into(),
            name: name,
            sort_key: name,
            optional: false,
            short: None,
            long: None,
            help: "",
            count: 0,
            typo: OptTypo::default(),
        }
    }
    /// Default is `Opt`'s name
    pub fn sort_key(mut self, sort_key: &'app str) -> Self {
        self.sort_key = sort_key;
        self
    }
    /// set `Opt`s optional as `true`(default is `false`)(override `OptValueParse`'s `default` and `check`).
    ///
    /// `App` will will not check it's value if the `Opt` not occurs and create help message without default's value if it is `true`.
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
    /// short
    pub fn short(mut self, short: char) -> Self {
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
    pub fn typo(mut self, typo: OptTypo) -> Self {
        self.typo = typo;
        self
    }
    #[doc(hidden)]
    pub fn count_add_one(&mut self) {
        self.count += 1;
    }
    #[doc(hidden)]
    pub fn parse(&mut self, msg: &str) -> Result<(), String> {
        self.count_add_one();
        self.value
            .as_mut()
            .parse(self.name, msg, &mut self.count, &mut self.typo)
    }
    #[doc(hidden)]
    pub fn check(&self) -> Result<(), String> {
        self.value
            .as_ref()
            .check(self.name, &self.optional, &self.count, &self.typo)
    }
}

/// A help function for `Opt`
pub fn opt<'a, V>(name: &'a str, value: V, short: Option<char>, long: Option<&'a str>, help: &'a str) -> Opt<'a>
where
    V: OptValueParse<'a>,
{
    let mut opt = Opt::new(name, value).sort_key(name).help(help);
    opt.short = short;
    opt.long = long;
    opt
}

/// A help function for `Args`
pub fn args<'a, V>(name: &'a str, value: V, help: &'a str) -> Args<'a>
where
    V: ArgsValueParse<'a>,
{
    Args::new(name, value).help(help)
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
        self.short.map(|s| format!("-{}", s))
    }
    pub fn long_get(&self) -> Option<String> {
        self.long.map(|s| "--".to_owned() + s)
    }
    pub fn help_get(&self) -> &str {
        self.help
    }
    pub fn typo_get(&self) -> &OptTypo {
        &self.typo
    }
    pub fn count_get(&self) -> &usize {
        &self.count
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
    count: usize,
}
impl<'app> Args<'app> {
    pub fn new<'s: 'app, V>(name: &'app str, value: V) -> Self
    where
        V: ArgsValueParse<'app>,
    {
        Args {
            name: name,
            value: value.into(),
            optional: false,
            len: None,
            help: "",
            count: 0,
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
    #[doc(hidden)]
    fn count_add_one(&mut self) {
        self.count += 1;
    }
    #[doc(hidden)]
    fn parse(&mut self, msg: &[String]) -> Result<(), String> {
        for arg in msg {
            self.count_add_one();
            self.value
                .as_mut()
                .parse(self.name, arg, &mut self.count, &mut self.len)?;
        }
        Ok(())
    }
    #[doc(hidden)]
    fn check(&self) -> Result<(), String> {
        self.value
            .as_ref()
            .check(self.name, &self.optional, &self.count, self.len.as_ref())
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
    pub fn count_get(&self) -> &usize {
        &self.count
    }
}
