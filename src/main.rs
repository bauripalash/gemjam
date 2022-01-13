//use gemjam::jam::builder::structure::StructureBuilder;
//use gemjam::jam::utils::conf_reader::Config;
//use std::collections::HashMap;
//use std::path::PathBuf;
//use yaml_rust::Yaml;
//use std::env;
use clap::{Arg,App};
fn main() {
    
    let matches = App::new("GemJam")
                .version("0.1")
                .author("Palash Bauri <palashbauri1@gmail.com>")
                .about("simple and kinda fast gemlog maker!")
                .arg(Arg::new("config")
                     .short('c')
                     .long("conf")
                     .takes_value(true)
                     .required(false)
                     .help("Directory where jam.yaml file can be found")).get_matches();
    println!("config file => {}" , matches.value_of("config").unwrap_or("t"));




    //println!("{:?}" , env::current_dir().unwrap());

    //let hm: HashMap<String, String> = HashMap::new();
    //let mut s = StructureBuilder::config(
    //    hm,
    //    PathBuf::from(r"/home/palash/gemjam/myjam/polu/"),
    //    PathBuf::from(r"/home/palash/gemjam/myjam/log/"),
    //    PathBuf::from(r"/home/palash/gemjam/myjam/polu/templates/"),
    //);

    //s.render_posts();
}
