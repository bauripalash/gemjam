use clap::{arg, App, AppSettings, Arg};

use gemjam::build_site;

use std::path::PathBuf;
fn main() {
    let jam_args = App::new("GemJam")
        .version("0.1")
        .author("Palash Bauri <palashbauri1@gmail.com>")
        .about("simple and kinda fast gemlog maker!")
        .subcommand(
            App::new("post")
                .about("New post")
                .arg(arg!(<NAME> "New post title"))
                .setting(AppSettings::SubcommandsNegateReqs)
                .setting(AppSettings::SubcommandPrecedenceOverArg),
        )
        .subcommand(
            App::new("site")
                .about("New Gemblog site")
                .arg(arg!(<NAME> "Name of the site"))
                .setting(AppSettings::SubcommandsNegateReqs)
                .setting(AppSettings::SubcommandPrecedenceOverArg),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("conf")
                .takes_value(true)
                .required(false)
                .help("Directory where jam.yaml file can be found"),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .takes_value(false)
                .required(false)
                .help("show version information"),
        )
        .get_matches();
    match jam_args.subcommand() {
        Some(("post", sub_m)) => println!("I think I have to create a new post {:?}", sub_m),
        Some(("site", sub_m)) => {
            println!("I have to create a new gemblog site {:?}", sub_m)
        }
        _ => {
            if jam_args.is_present("config") {
                let mut site_path = PathBuf::new();
                site_path.push(jam_args.value_of("config").unwrap());

                build_site(Some(&site_path));
            } else {
                build_site(None)
            }
        }
    }
}
