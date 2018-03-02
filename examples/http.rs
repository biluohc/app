extern crate app;
use app::{App, Args, Opt};

#[derive(Debug, Default)]
struct Http {
    keep_alive: bool,
    port: Option<u16>,
    paths: Vec<String>,
}
fn main() {
    let mut http = Http::default();
    http.paths.push("./".to_owned());
    {
        let mut app = App::new("http")
            .version("0.5.0")
            .desc("A Simple HTTP Server for Static File.")
            .author("Wspsxing", "biluohc@qq.com")
            .author("Xyz.org", "moz@mio.org")
            .addr(
                "GitHub",
                "https://github.com/biluohc/app/blob/master/examples/http.rs",
            )
            .opt(
                Opt::new("keep-alive", &mut http.keep_alive)
                    .short('k')
                    .long("keep-alive")
                    .help("open keep-alive"),
            )
            .opt(
                Opt::new("port", &mut http.port)
                    .short('p')
                    .long("port")
                    .help("Sets listenning port"),
            )
            .args(Args::new("PATH", &mut http.paths).help("Sets the path to share"))
            .build_helper();
        app.as_mut_helps()
            .cmd_usages
            .get_mut(&None)
            .map(|s| *s = "USAGE: \n   http [options] [<PATH>...]\n   http -p <port> [<PATH>...]\n   http --port <port> [<PATH>...]".to_owned())
            .unwrap();
        app.parse_args();
    }
    fun(&http);
}

fn fun(http: &Http) {
    println!("Http: {:?}", http);
    // do something
}
