use std::collections::HashMap;

pub enum JamValueType{
    
    Plain (String),
    Integer (isize),
    Float (f64),
    Bool (bool),
    Path (String),
    HexColor (String),
    RGBColor { r : usize , g : usize , b : usize },
    Url (String),

}



pub struct JamTextScanner{

   pub kv_list : HashMap<String , JamValueType>,
   pub source : Vec<String>,
   pub _line : usize

}


impl JamTextScanner {
    
    pub fn new(src : String) -> JamTextScanner{
        
        Self {
                
            kv_list : HashMap::new(),
            source : Vec::new(),
            _line : 1


        }

    }


}
