extern crate env_logger;
#[macro_use]
extern crate structopt;
extern crate tlog_server;

use structopt::StructOpt;
use tlog_server::server::Server;
use std::env;

#[derive(StructOpt, Debug)]
#[structopt(name = "tlog_server")]
struct Opt {
    /// log mode
    #[structopt(short = "l", long = "log", default_value = "info")]
    log: String,

    /// Tlog server address
    #[structopt(short = "a", long = "addr", default_value = "0.0.0.0:11211")]
    addr: String,

    /// zstor server address
    #[structopt(short = "s", long = "storage", default_value = "127.0.0.1:8080")]
    storage: String,

    /// namespace to be loaded
    #[structopt(short = "n", long = "namespace", default_value = "default")]
    namespace: String,

    /// namespace to be loaded
    #[structopt(short = "b", long = "branch", default_value = "0")]
    branch: usize,
}

fn main() {
    let opt = Opt::from_args();
    env::set_var("RUST_LOG", &opt.log);
    env_logger::init();
    let mut server = Server::new(&opt.addr, &opt.storage, &opt.namespace, opt.branch);
    server.serve().unwrap();
}
