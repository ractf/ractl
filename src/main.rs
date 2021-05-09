mod install;

use clap::{App, SubCommand};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("RACTF Admins <admins@ractf.co.uk>")
        .about("RACTF Admin CLI")
        .subcommand(install::subcommand())
        .get_matches();

    if let ("install", Some(x)) = matches.subcommand() {
        install::run();
    }
}
