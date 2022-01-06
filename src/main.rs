use gemjam::jam::utils::conf_reader::Config;
use yaml_rust::Yaml;
use gemjam::jam::builder::structure::StructureBuilder;
use std::collections::HashMap;
use std::path::PathBuf;

fn main() {
    let hm : HashMap<String , String> = HashMap::new(); 
    let mut s = StructureBuilder::config(hm, PathBuf::from(r"/home/palash/gemjam/myjam/polu/") ,  PathBuf::from(r"/home/palash/gemjam/myjam/log/") , PathBuf::from(r"/home/palash/gemjam/myjam/polu/templates/"));

    s.render_posts();

}
