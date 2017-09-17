#![allow(dead_code)]
#[macro_use]
extern crate stderr;
extern crate app;
use app::{App, Opt, Args, Cmd, OptValue, OptValueParse, OptTypo, AppError};

trait IsParse {
    fn is_parse(&self) -> bool;
}
impl IsParse for AppError {
    fn is_parse(&self) -> bool {
        match *self {
            AppError::Parse(_) => true,
            _ => false,
        }
    }
}
impl IsParse for Result<(), AppError> {
    fn is_parse(&self) -> bool {
        match *self {
            Ok(_) => false,
            Err(ref e) => e.is_parse(),
        }
    }
}
// cargo t -- --nocapture
#[test]
fn inner() {
    errln!("pkg!: {:?}", pkg!());
    fun("", Err(AppError::Parse(String::new())), Fht2p::default());
    fun(
        "/path0 -p 8080 -p 8000 -p 80   /path1 -k /path2 --user Loli,16,./ -V r",
        Err(AppError::Version),
        Fht2p::default(),
    );
    fun(
        "/path0 -p 8080 -p 8000 -p 80   /path1 -k /path2 --user Loli,16,./ r -V",
        Err(AppError::Parse(String::new())),
        Fht2p::default(),
    );
    fun(
        "/path0 -p 8080 -p 8000 -p 80   /path1 -k /path2 --user Loli,16,./ -h",
        Err(AppError::Help(None)),
        Fht2p::default(),
    );
    fun(
        "src/ -p 8080 -p 8000 -p 80   tests/ -k examples/ --user Loli,16,./ .git run -home $HOME -h",
        Err(AppError::Help(Some("run".to_owned()))),
        Fht2p::default(),
    );
    fun(
        "/path0 -p 8080 -p 8000 -p 80   /path1 -k /path2 --user Loli,16,./ b -h",
        Err(AppError::Help(Some("build".to_owned()))),
        Fht2p::default(),
    );

    fun(
        "src -p 8080 -p 8000 -p 80  examples -k tests --user Loli,16,./",
        Ok(()),
        Fht2p::new(
            vec![8080, 8000, 80u32],
            true,
            vec!["src", "examples", "tests"],
            User::new("Loli", 16, "./"),
            Run::default(),
            Build::default(),
        ),
    );
    fun(
        "src -p 8080 -p 8000 -p 80  examples -k tests",
        Ok(()),
        Fht2p::new(
            vec![8080, 8000, 80u32],
            true,
            vec!["src", "examples", "tests"],
            User::new("", 0, ""),
            Run::default(),
            Build::default(),
        ),
    );
    fun(
        "src -p 8080 -p 8000 -p 80  examples -k tests --user Loli,16,./ r --home $HOME",
        Ok(()),
        Fht2p::new(
            vec![8080, 8000, 80u32],
            true,
            vec!["src", "examples", "tests"],
            User::new("Loli", 16, "./"),
            Run::new("$HOME", false),
            Build::default(),
        ),
    );
    fun(
        "src -p 8080 -p 8000 -p 80  examples -k tests --user Loli,16,./ build -r sec ssx",
        Ok(()),
        Fht2p::new(
            vec![8080, 8000, 80u32],
            true,
            vec!["src", "examples", "tests"],
            User::new("Loli", 16, "./"),
            Run::default(),
            Build::new(true, vec!["sec", "ssx"]),
        ),
    );
}
fn fun(msg: &str, rest: Result<(), AppError>, value: Fht2p) {
    let args: Vec<String> = msg.split_whitespace().map(|s| s.to_string()).collect();
    let mut fht2p = Fht2p::default();

    println!("parse-before: {:?}", fht2p);
    let rest_parse = {
        App::new("fht2p")
            .version("0.5.0")
            .desc("A HTTP Server for Static File.")
            .author("Wspsxing", "biluohc@qq.com")
            .author("Xyz.org", "moz@mio.org")
            .addr("GitHub", "https://biluohc.github.com/fht2p")
            .opt(
                Opt::new("keep-alive", &mut fht2p.keep_alive)
                    .short('k')
                    .long("keep-alive")
                    .help("open keep-alive"),
            )
            .opt(
                Opt::new("ports", &mut fht2p.ports)
                    .short('p')
                    .long("port")
                    .help("Sets listenning port"),
            )
            .opt(
                Opt::new("user", &mut fht2p.user)
                    .short('u')
                    .long("user")
                    .optional()
                    .help("Sets user information"),
            )
            .args(Args::new("PATHS", &mut fht2p.dirs).help(
                r#"Sets the path to share"#,
            ))
            .cmd(
                Cmd::new("run")
                    .short("r")
                    .desc("run the sub_cmd")
                    .opt(
                        Opt::new("home", &mut fht2p.run.home)
                            .short('H')
                            .long("home")
                            .help("running in the home"),
                    )
                    .opt(
                        Opt::new("log", &mut fht2p.run.log)
                            .long("long")
                            .short('l')
                            .help("running and print log"),
                    ),
            )
            .cmd(
                Cmd::new("build")
                    .short("b")
                    .desc("build the file")
                    .opt(
                        Opt::new("release", &mut fht2p.build.release)
                            .short('r')
                            .long("release")
                            .help("Build artifacts in release mode, with optimizations"),
                    )
                    .args(Args::new("File", &mut fht2p.build.files).help(
                        "File to build",
                    )),
            )
            .parse_strings(&args[..])
    };
    dbln!("msg_args: {:?}", args);
    dbln!("rest: {:?}\t\trest_parse: {:?}", rest, rest_parse);
    dbln!("fht2p_value: {:?}", value);
    dbln!("fht2p_parse: {:?}", fht2p);
    dbln!();
    if rest.is_ok() && rest_parse.is_ok() {
        assert_eq!(value , fht2p);
    } else if rest.is_parse() {
    } else {
        assert_eq!(rest ,rest_parse);
    }
}

#[derive(Debug, Default, PartialEq)]
struct Fht2p {
    ports: Vec<u32>,
    keep_alive: bool,
    dirs: Vec<String>,
    user: User,
    run: Run,
    build: Build,
}
impl Fht2p {
    fn new(ports: Vec<u32>, keep_alive: bool, dirs: Vec<&str>, u: User, r: Run, b: Build) -> Self {
        let vs: Vec<String> = dirs.iter().map(|s| s.to_string()).collect();
        Fht2p {
            ports: ports,
            keep_alive: keep_alive,
            dirs: vs,
            user: u,
            run: r,
            build: b,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct Run {
    home: String,
    log: bool,
}
impl Run {
    fn new(home: &str, log: bool) -> Self {
        Run {
            home: home.to_owned(),
            log: log,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct Build {
    release: bool,
    files: Vec<String>,
}
impl Build {
    fn new(r: bool, files: Vec<&str>) -> Self {
        let vs: Vec<String> = files.iter().map(|s| s.to_string()).collect();
        Build {
            release: r,
            files: vs,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct User {
    name: String,
    age: u8,
    address: String,
}

impl User {
    fn new(name: &str, age: u8, addr: &str) -> Self {
        User {
            name: name.to_owned(),
            age: age,
            address: addr.to_owned(),
        }
    }
}

/// Custom `OptValue` by impl `OptValueParse`
impl<'app, 's: 'app> OptValueParse<'app> for &'s mut User {
    fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        if self.name.is_empty() {
            None
        } else {
            Some(format!("{},{},{}", self.name, self.age, self.address))
        }
    }
    fn parse(&mut self, opt_name: &str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String> {
        if *count == 0 || typo.is_covered() || typo.is_multiple() {
            self.name.clear();
            self.address.clear();
            let vs: Vec<&str> = msg.split(',').filter(|s| !s.is_empty()).collect();
            if vs.len() != 3 ||vs[0].trim().is_empty() {
                return Err(format!(
                    "OPTION(<{}>) parse<User> fails: \"{}\"",
                    opt_name,
                    msg
                ));
            }
            self.name.push_str(vs[0]);
            self.age = vs[1].parse::<u8>().map_err(|_| {
                format!("OPTION(<{}>) parse<User.age> fails: \"{}\"", opt_name, msg)
            })?;
            self.address.push_str(vs[2]);
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
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))?;
        }
        Ok(())
    }
}