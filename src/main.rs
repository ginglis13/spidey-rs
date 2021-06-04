use clap::{Arg, App};
use log::{info, debug, error};
use env_logger;

mod socket;
mod request;

fn main() -> std::io::Result<()> {
    env_logger::init();

    let matches = App::new("spidey-rs")
        .version("0.1.0")
        .author("ginglis")
        .about("multiprocessing webserver written in rust")
        .arg(Arg::with_name("mode")
            .short("c")
            .long("concurrency-mode")
            .takes_value(true)
            .help("single or forking mode"))
        .arg(Arg::with_name("mimepath")
            .short("m")
            .long("mime-path")
            .takes_value(true)
            .help("path to mimetypes file"))
        .arg(Arg::with_name("mimetype")
            .short("M")
            .long("default-mimetype")
            .takes_value(true)
            .help("default mimetype"))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .takes_value(true)
            .help("port to listen on"))
        .arg(Arg::with_name("root-dir")
            .short("r")
            .long("root-dir")
            .takes_value(true)
            .help("root directory to serve from"))
        .get_matches();

    let mode      = matches.value_of("mode").unwrap_or("single");
    let mimepath  = matches.value_of("mimepath").unwrap_or("/etc/mime.types");
    let mimetype  = matches.value_of("mimetype").unwrap_or("text/plain");
    let port      = matches.value_of("port").unwrap_or("8080").parse::<u16>().unwrap_or(8080);
    let root_path = matches.value_of("root-dir").unwrap_or("www");

    info!("Listening on port {}", port);
    debug!("Root Path: {}", root_path);
    debug!("Mimetypes Path: {}", mimepath);
    debug!("Default Mimetype: {}", mimetype);
    debug!("Concurrency mode: {}", mode);

    let tcp_listener = socket::socket_listen(port)?;

    if mode == "single" {
        //single_server(tcp_listener);
    } else if mode == "forking" {
        //forking_server(tcp_listener);
    }

    Ok(())
}
