include!("../examples/fht2p.rs");

// cargo t -- --nocapture
#[test]
fn main_() {
    // fun_("/path0 -p 8080,8000,80  /path1 -k /path2 --user Loli,16,./ -h");
    // fun_("src/ -p 8080,8000,80  tests/ -k examples/ --user Loli,16,./ .git run -home $HOME -h");
    // fun_("/path0 -p 8080,8000,80  /path1 -k /path2 --user Loli,16,./ build -h");
    // fun_("src -p 8080,8000,80 examples -k tests --user Loli,16,./");
    fun_("src -p 8080,8000,80 examples -k tests"); // optional
    // fun_("src -p 8080,8000,80 examples -k tests --user Loli,16,./ run --home $HOME");
    // fun_("src -p 8080,8000,80 examples -k tests --user Loli,16,./ build -r");
    // fun_("src -p 8080,8000,80 examples -k tests --user Loli,16,./ build -r -v");
    // fun_("src -p 8080,8000,80_  examples -k tests --user Loli,16,./ run -h");
    // fun_("");
}
fn fun_(args: &str) {
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
