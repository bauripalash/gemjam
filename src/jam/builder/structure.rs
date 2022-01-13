use yaml_rust::YamlLoader;
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
pub struct GemPost{

    pub path : PathBuf,
    pub title : Option<String>,
    pub slug : String,
    pub date : String,

}

pub struct StructureBuilder {
    pub basic_site_data: HashMap<String, String>,
    pub theme_dir: PathBuf,
    pub template_dir: PathBuf,
    pub content_dir: PathBuf,
}

impl StructureBuilder {
    pub fn config(
        site_data: HashMap<String, String>,
        theme_dir: PathBuf,
        content_dir: PathBuf,
        template_dir: PathBuf,
    ) -> StructureBuilder {
        Self {
            basic_site_data: site_data,
            theme_dir,
            template_dir,
            content_dir,
        }
    }

    pub fn get_post_list_markup(&self , posts : &mut Vec<GemPost>) -> String{
        let mut output : String = String::from("");
        let slug_prefix = "/log/";
        posts.sort_by(|a,b| a.date.cmp(&b.date));
        for post in posts{
        
            output += &format!("=> {}{} {} {}\n", slug_prefix , post.slug , post.date , post.title.as_ref().unwrap_or(&"".to_string()));

        }

        output

    }

    pub fn get_post_title(&mut self , p : PathBuf) -> Option<String>{
        let mut output : Option<String> = None;
        let header_regex = Regex::new(r"(?U)(?:\#{1,3}\s)(?P<title>.*)(?:\n)").unwrap();
        let raw_file = fs::read_to_string(p).unwrap();
        for ix in header_regex.captures(&raw_file){
            output = ix.get(1).map_or(None, |s| Some(s.as_str().to_string()));
        }
        return output;

    }

    pub fn get_posts(&mut self) -> Vec<GemPost>{
        let mut gemposts : Vec<GemPost> = Vec::new();

        for _posts in self.content_dir.read_dir().unwrap(){
            
            let file_name = String::from(_posts.as_ref().unwrap().path().file_stem().unwrap().to_str().unwrap());
            let _fn_vec : Vec<String> = file_name.split("_").into_iter().map(|s| s.to_string()).collect(); 
            gemposts.push(GemPost{
                
                path : _posts.as_ref().unwrap().path(),
                title : self.get_post_title(_posts.as_ref().unwrap().path().to_path_buf()),
                slug :  _fn_vec[0].clone(),
                date : _fn_vec[1].clone()

            });

            
            

        }
        gemposts


    }

    pub fn render_posts(&mut self) -> bool {
        let rex = Regex::new(r"\[\[\[(?P<reptag>.+)\]\]\]").unwrap();
        let mut no_error = true;
        let index_path = &self.template_dir.join("index.gmi");
        //let mut reptag_raws: Vec<String> = Vec::new();
        //let mut reptag_names: Vec<String> = Vec::new();
        let _con = YamlLoader::load_from_str("
templates:
    footer : footer.gmi
    header : header.gmi",
        )
        .unwrap();
        let mut _template_keys : Vec<String> = Vec::new();
        let mut _target_keys : Vec<String> = Vec::new();
        let _cf = &_con[0];
        if !_cf["templates"].is_badvalue(){
            for (k,_) in _cf["templates"].as_hash().unwrap(){
                
                _template_keys.push(k.as_str().unwrap().to_string());

            }
        }

        if index_path.exists(){
            let index_raw = fs::read_to_string(index_path).unwrap();
            let x = rex.captures_iter(&index_raw);
            for i in x{
                
                _target_keys.push(i["reptag"].trim().to_string())

            }

            let mut posts = self.get_posts();
            //let x = Regex::new(r"\[\[\[(\s*)(_posts_)(\s*)\]\]\]").unwrap();
            //let mut y = x.replace(&index_raw, &self.get_post_list_markup(&mut posts)).to_string();
            let mut y = index_raw.replace("[[[_posts_]]]", &self.get_post_list_markup(&mut posts));
            for item in _target_keys{
                
                if _template_keys.contains(&item){
                    
                    let _rplc_with_fname = &_cf["templates"][item.as_str()];
                    let _raw_file = fs::read_to_string(format!("myjam/polu/templates/{}" , _rplc_with_fname.as_str().unwrap())).expect("not found");
                    y = y.replace(format!("[[[{}]]]" , item).as_str(), &_raw_file);

                }

            }
            println!("{}" , &y);

        }else {
            
            println!("[Err] => Can not find the index file <index.gmi> at the templates folder of your theme!");
            no_error = false;


        }

        //self.get_post_title(posts[0].path.to_path_buf());
        //println!("{:?}" , posts);
        //println!("{}" , self.get_post_list_markup(&mut posts));

        no_error
    }

}
