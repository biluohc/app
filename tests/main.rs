extern crate app;
use app::{App, Opt, Cmd, OptValue, OptValueParse};

// cargo t -- --nocapture
#[test]
fn main() {
    // fun("/path0 -p 8080,8000,80  /path1 -k /path2 --user Loli,16,./ -h");
    // fun("src/ -p 8080,8000,80  tests/ -k examples/ --user Loli,16,./ .git run -home $HOME -h");
    // fun("/path0 -p 8080,8000,80  /path1 -k /path2 --user Loli,16,./ build -h");
    // fun("src -p 8080,8000,80 examples -k tests --user Loli,16,./");
    fun("src -p 8080,8000,80 examples -k tests"); // optional
    // fun("src -p 8080,8000,80 examples -k tests --user Loli,16,./ run --home $HOME");
    // fun("src -p 8080,8000,80 examples -k tests --user Loli,16,./ build -r");
    // fun("src -p 8080,8000,80 examples -k tests --user Loli,16,./ build -r -v");
    // fun("src -p 8080,8000,80_  examples -k tests --user Loli,16,./ run -h");
    // fun("");
}
fn fun(args: &str) {
    println!("Args: {:?}", args);
    let args: Vec<String> = args.split_whitespace().map(|s| s.to_string()).collect();
    let mut fht2p = Fht2p::default();
    fht2p.build.files.push("src/avp.rs".to_string());
    let mut cmd: Option<String> = None;
    println!("{:?}", fht2p);
    let helper = {
        App::new("fht2p")
            .version("0.5.0")
            .desc("A HTTP Server for Static File.")
            .author("Wspsxing", "biluohc@qq.com")
            .author("Xyz.org", "moz@mio.org")
            .addr("GitHub", "https://biluohc.github.com/fht2p")
            .opt(Opt::new("keep-alive", &mut fht2p.keep_alive)
                     .short("k")
                     .long("keep-alive")
                     .help("open keep-alive"))
            .opt(Opt::new("ports", &mut fht2p.ports)
                     .short("p")
                     .long("port")
                     .help("Sets listenning port"))
            .opt(Opt::new("user", &mut fht2p.user)
                     .optional()
                     .short("u")
                     .long("user")
                     .help("Sets user information"))
            .args("Dirs", &mut fht2p.dirs)
            .args_help("Sets the paths to share")
            .args_optional()
            .current_cmd(&mut cmd)
            .cmd(Cmd::new("run")
                     .desc("run the sub_cmd")
                     .opt(Opt::new("home", &mut fht2p.run.home)
                              .short("home")
                              .long("home")
                              .help("running in the home"))
                     .opt(Opt::new("log", &mut fht2p.run.log)
                              .long("log")
                              .help("running and print log")))
            .cmd(Cmd::new("build")
                     .desc("build the file")
                     .opt(Opt::new("release", &mut fht2p.build.release)
                              .short("r")
                              .long("release")
                              .help("Build artifacts in release mode, with optimizations"))
                     .args("Files", &mut fht2p.build.files)
                     .args_help("Files to build"))
            .parse(&args[..])
    };
    println!("{:?}_{:?}", cmd, helper.current_cmd_ref().clone());
    assert_eq!(cmd, helper.current_cmd_ref().clone());

    println!("{:?}", fht2p);
    match helper.current_cmd_str() {
        None => {
            println!("Command::running: main");
        } //main
        Some("run") => {
            println!("Command::running: {:?}", helper.current_cmd_str());
        }
        Some("build") => {
            println!("Command::running: {:?}", helper.current_cmd_str());
        }   
        _ => unreachable!(),
    }
    println!("----------------------app -v/--version--------------------");
    println!("{}", helper.ver().trim());
    for (k, v) in helper.helps() {
        println!("----------------------app {:?}--------------------\n{}",
                 k,
                 v.trim());
    }
}

#[derive(Debug,Default)]
struct Fht2p {
    ports: Vec<u32>,
    keep_alive: bool,
    dirs: Vec<String>,
    user: User,
    run: Run,
    build: Build,
}

#[derive(Debug,Default)]
struct Run {
    home: String,
    log: bool,
}

#[derive(Debug,Default)]
struct Build {
    release: bool,
    files: Vec<String>,
}

#[derive(Debug,Default)]
struct User {
    name: String,
    age: u8,
    address: String,
}

// Custom OptValue by impl OptValueParse
impl<'app, 's: 'app> OptValueParse<'app> for &'s mut User {
    fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    // As --help/-h,they not have value follows it.
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
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
        self.name.clear();
        self.address.clear();
        let vs: Vec<&str> = msg.split(',').filter(|s| !s.is_empty()).collect();
        if vs.len() != 3 {
            return Err(format!("OPTION({}) parse<User> fails: \"{}\"", opt_name, msg));
        }
        self.name.push_str(vs[0]);
        self.age = vs[1]
            .parse::<u8>()
            .map_err(|_| format!("OPTION({}) parse<User.age> fails: \"{}\"", opt_name, msg))?;
        self.address.push_str(vs[2]);
        Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.name.is_empty() {
            Err(format!("OPTION({})'s value missing", opt_name))
        } else {
            Ok(())
        }
    }
}