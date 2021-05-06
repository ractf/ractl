mod install;

use clap::{App, SubCommand};

fn main() {
    let matches = App::new("ractfctl")
        .version("0.1")
        .author("RACTF Admins <admins@ractf.co.uk>")
        .about("RACTF Admin CLI")
        .subcommand(install::subcommand())
        .get_matches();

    if let ("install", Some(x)) = matches.subcommand() {
        install::run();
    }
}
