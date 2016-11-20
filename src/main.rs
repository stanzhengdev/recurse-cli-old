// Interact with recurse.com resources
//
// cli portal to recurse services

extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Recurse")
        .version("0.0.1")
        .author("Stanley Zheng<dev.stanzheng@gmail.com>")
        .about("Interact with the Recurse Center")
        .subcommand(SubCommand::with_name("search")
            .about("Controls portal search")
            .arg(Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely")))
        .get_matches();
}