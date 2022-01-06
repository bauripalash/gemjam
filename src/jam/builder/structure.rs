use std::collections::HashMap;
use std::path::*;
use std::fs;

// index.gmi -> index page >> where the post indexting will happen
// header.gmi -> will be pasted at the top of the index.gmi file
// footer.gmi -> botom
// vivekananda vs kal marx -> mitri koutillo
// vivekananda er biplab bad

pub struct StructureBuilder {
    pub basic_site_data: HashMap<String, String>,
    pub theme_dir: PathBuf,
    pub template_dir : PathBuf,
    pub content_dir: PathBuf,
}

impl StructureBuilder {
    pub fn config(
        site_data: HashMap<String, String>,
        theme_dir: PathBuf,
        content_dir: PathBuf,
        template_dir : PathBuf
    ) -> StructureBuilder {
        Self {
            basic_site_data: site_data,
            theme_dir,
            template_dir,
            content_dir,
        }
    }

    pub fn render_posts(&mut self) -> bool {
        let mut no_error = true;
        let index_path = &self.template_dir.join("index.gmi");
        if index_path.exists(){
            
            println!("Index file found");

        } else {
            
            println!("Index file not found");
            no_error = false;

        }
        
        //let raw_index = fs::read_to_string(index_path);
        //println!("{}" , raw_index.as_ref().unwrap());
        //let raw_header = fs::read_to_string(self.template_dir.join("header.gmi"));
        //fs::write(self.template_dir.join("out.gmi"), raw_header.unwrap() +  &raw_index.unwrap());
        
        for item in fs::read_dir(&self.content_dir).unwrap(){
        
           println!("/{}/{:?}" , self.content_dir.file_name().unwrap().to_string_lossy()  , item.unwrap().file_name());

        }

        no_error
    }
}
