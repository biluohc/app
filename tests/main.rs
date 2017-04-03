extern crate app;
use app::{App, Opt, Cmd, OptValue, OptValueParse};

// cargo t -- --nocapture
#[test]
fn main() {
    // let args = "/path0 -p 8080,8000,80  /path1 -ka /path2 --user Loli,16,./ -h";
    let args = "/path0 -p 8080,8000,80  /path1 -ka /path2 --user Loli,16,./ run -h";
    // let args = "/path0 -p 8080,8000,80  /path1 -ka /path2 --user Loli,16,./ build -h";
    // let args = "/path0 -p 8080,8000,80  /path1 -ka /path2 --user Loli,16,./";
    // let args = "/path0 -p 8080,8000,80_  /path1 -ka /path2 --user Loli,16,./ run -h";
    let args: Vec<String> = args.split_whitespace().map(|s| s.to_string()).collect();
    fun(args);
}
fn fun(args: Vec<String>) {
    let mut fht2p = Fht2p::default();
    println!("{:?}", fht2p);
    {
        let mut app = App::new("fht2p")
            .version("0.1.0")
            .desc("A HTTP Server for Static File.")
            .author("Wspsxing", "biluohc@qq.com")
            .author("Xyz.org", "moz@mio.org")
            .addr("GitHub", "https://biluohc.github.com/fht2p")
            .opt(Opt::new("keep-alive", &mut fht2p.keep_alive)
                     .short("ka")
                     .long("keep-alive")
                     .help("open keep-alive"))
            .opt(Opt::new("ports", &mut fht2p.ports)
                     .short("p")
                     .long("ports")
                     .help("Sets listenning port"))
            .opt(Opt::new("user", &mut fht2p.user)
                     .short("u")
                     .long("user")
                     .help("Sets user information"))
            .args("Paths", &mut fht2p.routes)
            .cmd(Cmd::new("run")
                     .desc("run the sub_cmd")
                     .opt(Opt::new("log", &mut fht2p.run.log)
                              .long("log")
                              .help("running and print log")))
            .cmd(Cmd::new("build")
                     .desc("build the file")
                     .opt(Opt::new("release", &mut fht2p.build.release)
                              .short("r")
                              .long("release")
                              .help("Build artifacts in release mode, with optimizations")));
        //You should use app.parse(), app.parse_strings(args) is write for test conveniently.
        if let Err(e) = app.parse_strings(args) {
            println!("{}", e);
        }
        // println!("\n{:?}", app);
    }
    println!("{:?}\n", fht2p);
    // macth sub_cmd's name
    match fht2p.sub_cmd() {
        "" => {} //main
        "run" => {}
        "build" => {}        
        _ => unreachable!(),
    }
}

#[derive(Debug,Default)]
struct Fht2p {
    ports: Vec<u32>,
    keep_alive: bool,
    routes: Vec<String>,
    sub_cmd: String,
    user: User,
    run: Run,
    build: Build,
}
impl Fht2p {
    fn sub_cmd(&self) -> &str {
        &self.sub_cmd
    }
}

#[derive(Debug,Default)]
struct Run {
    log: bool,
}

#[derive(Debug,Default)]
struct Build {
    release: bool,
}

#[derive(Debug,Default)]
struct User {
    name: String,
    age: u8,
    address: String,
}

// Custom OptValue by impl OptValueParse
impl<'app, 's: 'app> OptValueParse<'app> for &'s mut User {
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::from(self) }
    }
    // As --help/-h,they not have value follows it.
    fn is_bool(&self) -> bool {
        false
    }
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
        self.name.clear();
        self.address.clear();
        let vs: Vec<&str> = msg.split(',').filter(|s| !s.is_empty()).collect();
        if vs.len() != 3 {
            return Err(format!("OPTION({}) parse<User> fails: \"{}\"", opt_name, msg));
        }
        self.name.push_str(vs[0]);
        self.age = vs[1].parse::<u8>()
            .map_err(|_| format!("OPTION({}) parse<User.age> fails: \"{}\"", opt_name, msg))?;
        self.address.push_str(vs[2]);
        Ok(())
    }
}