use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::*;
// index.gmi -> index page >> where the post indexting will happen
// header.gmi -> will be pasted at the top of the index.gmi file
// footer.gmi -> botom
// vivekananda vs kal marx -> mitri koutillo
// vivekananda er biplab bad
//

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
    pub slug: String,
    pub reptags: HashMap<String, String>,
}

impl StructureBuilder {
    pub fn config(
        content_dir: PathBuf,
        template_dir: PathBuf,
        slug: String,
        reptags: HashMap<String, String>,
    ) -> StructureBuilder {
        Self {
            template_dir,
            content_dir,
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

    pub fn get_post_title(&mut self, p: PathBuf) -> Option<String> {
        let mut output: Option<String> = None;
        lazy_static! {
            static ref HEADER_REGEX: Regex =
                Regex::new(r"(?U)(?:\#{1,3}\s)(?P<title>.*)(?:\n)").unwrap();
        }
        let raw_file = fs::read_to_string(p).unwrap();
        for ix in HEADER_REGEX.captures(&raw_file) {
            output = ix.get(1).map_or(None, |s| Some(s.as_str().to_string()));
        }
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
                title: self.get_post_title(_posts.as_ref().unwrap().path().to_path_buf()),
                slug: _fn_vec[0].clone(),
                date: _fn_vec[1].clone(),
            });
        }
        gemposts
    }

    pub fn render_posts(&mut self) -> bool {
        let rex = Regex::new(r"\[\[\[(?P<reptag>.+)\]\]\]").unwrap();
        let mut no_error = true;
        let index_path = &self.template_dir.join("index.gmi");
        let posts_reptag: String = self.reptags.get("_posts_").unwrap().to_string();
        //let posts_reptag : String = self
        //println!("{}" , posts_reptag);
        if index_path.exists() {
            let index_raw = fs::read_to_string(index_path).unwrap();
            let mut _raw = index_raw.clone();
            let x = rex.captures_iter(&index_raw);
            for i in x {
                let _r = i["reptag"].to_string();
                //println!("{}" , _r == posts_reptag);
                if self.reptags.contains_key(&_r) {
                    if _r == posts_reptag {
                        _raw =
                            _raw.replace(i.get(0).unwrap().as_str(), &self.get_post_list_markup());
                        continue;
                    }

                    let _replace_with =
                        fs::read_to_string(self.template_dir.join(self.reptags.get(&_r).unwrap()))
                            .expect("Cannot file for this reptag!");
                    _raw = _raw.replace(i.get(0).unwrap().as_str(), &_replace_with);
                }
            }

            println!("==>\n{}\n<==", _raw);

            //let mut posts = self.get_posts();
            //let x = Regex::new(r"\[\[\[(\s*)(_posts_)(\s*)\]\]\]").unwrap();
            //let mut y = x.replace(&index_raw, &self.get_post_list_markup(&mut posts)).to_string();
            //let mut y = index_raw.replace("[[[_posts_]]]", &self.get_post_list_markup(&mut posts));
            //for item in _target_keys {
            //    if self.reptags.contains_key(&item) {
            //        let _rplc_with_fname = self.reptags.get(&item).unwrap();
            //        let _raw_file =
            //            fs::read_to_string(format!("myjam/polu/templates/{}", _rplc_with_fname))
            //                .expect("not found");
            //        y = y.replace(format!("[[[{}]]]", item).as_str(), &_raw_file);
            //    }
            //}
            //println!("{}", &y);
        } else {
            println!("[Err] => Can not find the index file <index.gmi> at the templates folder of your theme!");
            no_error = false;
        }

        //self.get_post_title(posts[0].path.to_path_buf());
        //println!("{:?}" , posts);
        //println!("{}" , self.get_post_list_markup(&mut posts));

        no_error
    }
}
