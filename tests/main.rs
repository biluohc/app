include!("../examples/fht2p.rs");

// cargo t -- --nocapture
#[test]
fn main_() {
    // let args ="";
    // let args = "/path0 -p 8080,8000,80  /path1 -ka /path2 --user Loli,16,./ -h";
    // let args = "/path0 -p 8080,8000,80  /path1 -ka /path2 --user Loli,16,./ run -h";
    // let args = "/path0 -p 8080,8000,80  /path1 -ka /path2 --user Loli,16,./ build -h";
    // let args = "/path0 -p 8080,8000,80  /path1 -ka /path2 --user Loli,16,./";
    // let args = "/path0 -p 8080,8000,80  /path1 -ka /path2 --user Loli,16,./ run";
    // let args = "/path0 -p 8080,8000,80  /path1 -ka /path2 --user Loli,16,./ build -r";
    let args = "/path0 -p 8080,8000,80_  /path1 -ka /path2 --user Loli,16,./ run -h";
    let args: Vec<String> = args.split_whitespace().map(|s| s.to_string()).collect();
    fun_(args);
}
fn fun_(args: Vec<String>) {
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
            .current_cmd(&mut fht2p.sub_cmd)
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