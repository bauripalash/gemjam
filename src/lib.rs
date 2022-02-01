pub mod jam;

use jam::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::{path::PathBuf, process::exit};
use yaml_rust::{Yaml, YamlLoader};

pub fn get_config(p: &PathBuf) -> Yaml {
    let _p = p.join("jam.yaml");
    let raw = fs::read_to_string(_p).unwrap();
    let _cfg = YamlLoader::load_from_str(&raw).unwrap();
    let _config = &_cfg[0];
    return _config.to_owned();
}

pub struct MakeJam {
    working_dir: PathBuf,
    config: Yaml,
}

impl MakeJam {
    pub fn newjar(working_dir: PathBuf) -> MakeJam {
        Self {
            config: get_config(&working_dir),
            working_dir,
        }
    }

    pub fn rocknroll(&self) {
        // ===========================================================
        // Main Site Configs
        // ===========================================================

        let content_dir_str = if !self.config["site_config"]["content_dir"].is_badvalue() {
            self.config["site_config"]["content_dir"]
                .as_str()
                .expect("Error in your jam.yaml file -> invalid 'theme_dir'")
        } else {
            panic!("Invalid 'content dir'")
        };
        let template_dir_str = if !self.config["site_config"]["template_dir"].is_badvalue() {
            self.config["site_config"]["template_dir"]
                .as_str()
                .expect("Error in your jam.yaml file -> invalid 'template_dir'")
        } else {
            panic!("Invalid 'theme_dir'")
        };

        let output_dir_str = if !self.config["site_config"]["output_dir"].is_badvalue() {
            self.config["site_config"]["output_dir"]
                .as_str()
                .expect("Error in your jam.yaml file -> invalid 'output_dir'")
        } else {
            panic!("Invalid 'theme_dir")
        };

        let index_entry_str = if !self.config["site_config"]["entry"].is_badvalue() {
            self.config["site_config"]["entry"]
                .as_str()
                .expect("Error in your jam.yaml -> Invalid 'entry' point")
        } else {
            panic!("Invalid 'entry' point")
        };

        let slug_path = if !self.config["site_config"]["slug_format"].is_badvalue() {
            self.config["site_config"]["slug_format"]
                .as_str()
                .unwrap_or("log")
                .to_string()
        } else {
            eprintln!("Invalid or nonexistant 'slug_format' --> using default 'log'");
            "log".to_string()
        };

        let mut reptags: HashMap<String, String> = HashMap::new();

        if !self.config["reptags"].is_badvalue() {
            for (key, value) in self.config["reptags"].as_hash().unwrap() {
                if !key.is_badvalue() && !value.is_badvalue() {
                    reptags.insert(
                        key.as_str().unwrap().to_string(),
                        value.as_str().unwrap().to_string(),
                    );
                }
            }
        }

        let content_dir: PathBuf = self.working_dir.join(content_dir_str);
        let template_dir: PathBuf = self.working_dir.join(template_dir_str);
        let output_dir: PathBuf = self.working_dir.join(output_dir_str);

        if !template_dir.exists() {
            eprintln!(
                "Template directory at {} not found\nBye!",
                template_dir.display()
            );
            exit(1);
        }
        if !content_dir.exists() {
            eprintln!(
                "Content directory at {} not found\nBye!",
                content_dir.display()
            );
            exit(1);
        }
        if !output_dir.exists() {
            eprintln!(
                "Output directory at {} not found\nCreating it for you!",
                output_dir.display()
            );
            fs::create_dir(&output_dir).unwrap();

            //exit(1);
        }

        let index_entry: PathBuf = self
            .working_dir
            .join(&template_dir_str)
            .join(index_entry_str);
        if !index_entry.exists() {
            eprintln!(
                "Entry point file at {} not found\nBye!",
                index_entry.display()
            );
            exit(1);
        }

        // ======================= End of Main Site Configs =========================

        // =====================================================================
        // Posts Configs
        // =====================================================================
        //

        let posts_entry_str = if !self.config["post_config"]["entry"].is_badvalue() {
            self.config["post_config"]["entry"]
                .as_str()
                .expect("Error in your jam.yaml => invalid post template entry file!")
        } else {
            eprintln!("Invalid post template entry file!");
            panic!();
        };

        let posts_entry = self.working_dir.join(&template_dir_str).join(posts_entry_str);

        if !posts_entry.exists() {
            eprintln!(
                "Posts entry file at {} not found\nBye!",
                posts_entry.display()
            );
            exit(1);
        }

        let mut s_builder = builder::structure::StructureBuilder::config(
            content_dir,
            template_dir,
            output_dir,
            index_entry,
            posts_entry,
            slug_path,
            reptags,
        );
        s_builder.render_posts();
    }
}
