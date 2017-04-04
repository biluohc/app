include!("../examples/fht2p.rs");

#[macro_use]
extern crate stderr;

// cargo t -- --nocapture
#[test]
fn main_() {
    // fun_("/path0 -p 8080,8000,80  /path1 -ka /path2 --user Loli,16,./ -h");
    fun_("src/ -p 8080,8000,80  tests/ -ka examples/ --user Loli,16,./ .git run -hm $HOME -h");
    // fun_("/path0 -p 8080,8000,80  /path1 -ka /path2 --user Loli,16,./ build -h");
    // fun_("src -p 8080,8000,80  /examples -ka /tests --user Loli,16,./");
    // fun_("src -p 8080,8000,80 examples -ka tests --user Loli,16,./ run --home $HOME");
    // fun_("src -p 8080,8000,80 examples -ka tests --user Loli,16,./ build -r");
    // fun_("src -p 8080,8000,80 examples -ka tests --user Loli,16,./ build -r -v");
    // fun_("src -p 8080,8000,80_  examples -ka tests --user Loli,16,./ run -h");
    // fun_("");
}
fn fun_(args: &str) {
    let mut fht2p = Fht2p::default();
    println!("{:?}", fht2p);
    println!("Args: {:?}", args);
    let args: Vec<String> = args.split_whitespace().map(|s| s.to_string()).collect();
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
            .args("Dirs", &mut fht2p.routes)
            .args_check(args_checker)
            .current_cmd(&mut fht2p.sub_cmd)
            .cmd(Cmd::new("run")
                     .desc("run the sub_cmd")
                     .opt(Opt::new("home", &mut fht2p.run.home)
                              .short("hm")
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
                              .help("Build artifacts in release mode, with optimizations")));
        //You should use app.parse(), app.parse_strings(args) is write for test conveniently.
        if let Err(e) = app.parse_strings(args) {
            if e == String::new() {
                std::process::exit(0); // -h/-v
            }
            errln!("ERROR:\n  {}\n", e);
            if app.current_cmd_get().is_some() && app.current_cmd_get() != Some(&mut String::new()) {
                if let Some(ref s) = app.current_cmd_get() {
                    app.help_cmd(s);
                }
            } else {
                app.help();
            }
            std::process::exit(1);
        }
        // println!("\n{:?}", app);
    }
    println!("{:?}\n", fht2p);
    println!("CMD: {:?}", fht2p.sub_cmd());
    // macth sub_cmd's name
    match fht2p.sub_cmd() {
        "" => {} //main
        "run" => {}
        "build" => {}        
        _ => unreachable!(),
    }
}