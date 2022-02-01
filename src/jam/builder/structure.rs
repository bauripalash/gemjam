use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::*;

// index.gmi -> index page >> where the post indexting will happen
// header.gmi -> will be pasted at the top of the index.gmi file
// footer.gmi -> botom
// vivekananda vs kal marx -> mitri koutillo
// vivekananda er biplab bad
//

lazy_static! {
    static ref REX: Regex = Regex::new(r"\[\[\[(?P<reptag>.+)\]\]\]").unwrap();
    static ref HEADER_REGEX: Regex = Regex::new(r"(?U)(?:\#{1,3}\s)(?P<title>.*)(?:\n)").unwrap();
    static ref POST_REX: Regex = Regex::new(r"\[\[\[(_content_)\]\]\]").unwrap();
}

#[derive(Debug)]
pub struct GemPost {
    pub path: PathBuf,
    pub title: Option<String>,
    pub slug: String,
    pub date: String,
}

pub struct StructureBuilder {
    pub template_dir: PathBuf,
    pub content_dir: PathBuf,
    pub output_dir: PathBuf,
    pub index_entry: PathBuf,
    pub post_entry_fstring: String,
    pub slug: String,
    pub reptags: HashMap<String, String>,
}

impl StructureBuilder {
    pub fn config(
        content_dir: PathBuf,
        template_dir: PathBuf,
        output_dir: PathBuf,
        index_entry: PathBuf,
        post_entry: PathBuf,
        slug: String,
        reptags: HashMap<String, String>,
    ) -> StructureBuilder {
        if !output_dir.exists() {
            fs::create_dir(&output_dir).unwrap();
        } else {
            fs::remove_dir_all(&output_dir).unwrap();
            fs::create_dir(&output_dir).unwrap();
            fs::create_dir(&output_dir.join(&slug)).unwrap();
        }
        Self {
            template_dir,
            content_dir,
            output_dir,
            index_entry,
            post_entry_fstring: fs::read_to_string(post_entry).unwrap(),
            slug,
            reptags,
        }
    }

    pub fn get_post_list_markup(&mut self) -> String {
        let posts = &mut self.get_posts();
        let mut output: String = String::from("");

        posts.sort_by(|a, b| a.date.cmp(&b.date));
        for post in posts {
            output += &format!(
                "=> /{}/{} {} {}\n",
                self.slug,
                post.slug,
                post.date,
                post.title.as_ref().unwrap_or(&"".to_string())
            );
        }

        output
    }

    pub fn get_post_title(&mut self, p: PathBuf, file_name: &String) -> Option<String> {
        let mut output: Option<String> = None;

        let raw_file = fs::read_to_string(p).unwrap();

        let _out_post = POST_REX.replace(&self.post_entry_fstring, &raw_file);

        for ix in HEADER_REGEX.captures(&raw_file) {
            output = ix.get(1).map_or(None, |s| Some(s.as_str().to_string()));
        }
        let output_path = self
            .output_dir
            .join(&self.slug)
            .join(file_name.to_owned() + &".gmi".to_string());
        let mut fdsc = File::create(&output_path)
            .expect("Failed to create a file for your post in output directory!");
        fdsc.write(_out_post.as_bytes())
            .expect("Failed to save post!"); //fs::write(output_path, &*_out_post).unwrap();
        output
    }

    pub fn get_posts(&mut self) -> Vec<GemPost> {
        let mut gemposts: Vec<GemPost> = Vec::new();

        for _posts in self.content_dir.read_dir().unwrap() {
            let file_name = String::from(
                _posts
                    .as_ref()
                    .unwrap()
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            );
            let _fn_vec: Vec<String> = file_name
                .split("_")
                .into_iter()
                .map(|s| s.to_string())
                .collect();
            gemposts.push(GemPost {
                path: _posts.as_ref().unwrap().path(),
                title: self
                    .get_post_title(_posts.as_ref().unwrap().path().to_path_buf(), &_fn_vec[0]),
                slug: _fn_vec[0].clone(),
                date: _fn_vec[1].clone(),
            });
        }
        gemposts
    }

    pub fn render_posts(&mut self) -> bool {
        let mut no_error = true;
        let index_path = &self.index_entry; //&self.template_dir.join("index.gmi");
        let posts_reptag: String = self.reptags.get("_posts_").unwrap().to_string();
        if index_path.exists() {
            let index_raw = fs::read_to_string(index_path).unwrap();
            let mut _raw = index_raw.clone();
            let x = REX.captures_iter(&index_raw);
            for i in x {
                let _r = i["reptag"].to_string();
                if self.reptags.contains_key(&_r) {
                    if _r == posts_reptag {
                        _raw =
                            _raw.replace(i.get(0).unwrap().as_str(), &self.get_post_list_markup());

                        continue;
                    }
                    let _rrv = self.reptags.get(&_r).unwrap();
                    let mut _replace_with = String::new();
                    if _rrv.starts_with("file=>") {
                        let _filename = _rrv.strip_prefix("file=>").unwrap();
                        //println!("{}", _filename);

                        _replace_with = fs::read_to_string(self.template_dir.join(_filename))
                            .expect("Cannot file for this reptag!");
                    } else {
                        _replace_with = _rrv.to_string();
                    }
                    _raw = _raw.replace(i.get(0).unwrap().as_str(), &_replace_with);
                }
            }

            let out_index_path = &self.output_dir.join("index.gmi");
            let mut index_file_dsc =
                File::create(out_index_path).expect("Cannot create index file in output directory");
            index_file_dsc
                .write(_raw.as_bytes())
                .expect("Failed to save index file in output directory");
        } else {
            println!(
                "[Err] => Can not find the entry file {} at the templates folder of your theme!",
                self.index_entry.file_name().unwrap().to_str().unwrap()
            );
            no_error = false;
        }

        no_error
    }
}
