use reqwest;
use std::collections::HashMap;

//OPTIONS
//

pub struct Image_Downloader{
    pub keywords: Vec<String>,
    pub lang: String,
    pub size: String,
    pub color: String,
    pub color_type: String,
    pub usage_rights: String,
    pub img_type: String,
    pub time: String,
    pub aspect_ratio: String,
    pub format: Vec<String>,
    pub safe_search: bool,
    pub params: HashMap<String, String>,
}

impl Image_Downloader{
    pub fn new (keywords: Vec<String>, format: Vec<String>, params: HashMap<String, String>) -> Self 
    {
        Image_Downloader
        {
            keywords: keywords,
            lang: "".to_string(),
            size: "".to_string(),
            color: "".to_string(),
            color_type: "".to_string(),
            usage_rights: "".to_string(),
            img_type: "".to_string(),
            time: "".to_string(),
            aspect_ratio: "".to_string(),
            format: format,
            safe_search: false,
            params: params,
        }
    }

    pub fn create_search_url(&mut self)
    {
        
    }    

    pub fn build_url_parameters(&mut self) -> String
    {

        let mut built_url = "&tbs=";
        //TODO: setup lang builder
        //let mut lang_header = "&lr=";
        let mut lang_url = "";
        //if user_params.contains_key("lang") {
            //let mut lang = 
            //Lang param Hashmap
        //lang_url.push_str(lang_header);
        //lang.url.push_str(lang
        //}
        
        //TODO Exact Size Code

        //Params as hashmaps within hashmap to reference arguments for url param building
        let mut params = HashMap::from([
            ("color", HashMap::from([
                        ("red", "ic:specific,isc:red"), ("orange", "ic:specific,isc:orange"),
                        ("yellow", "ic:specific,isc:yellow"), ("green", "ic:specific,isc:green"),
                        ("teal", "ic:specific,isc:teal"), ("blue", "ic:specific,isc:blue"),
                        ("purple", "ic:specific,isc:purple"), ("pink", "ic:specific,isc:pink"),
                        ("white", "ic:specific,isc:white"), ("gray", "ic:specific,isc:gray"),
                        ("black", "ic:specific,isc:black"), ("brown", "ic:specific,isc:brown")
                        ])),
             ("color_type", HashMap::from([])),
             ("usage_rights", HashMap::from([
                        ("labeled-for-reuse-with-modifications", "sur:fmc"),
                        ("labeled-for-reuse", "sur:fc"),
                        ("labeled-for-noncommercial-reuse-with-modification","sur:fm"),
                        ("labeled-for-noncommercial-reuse", "sur:f")
             ])),
             ("size", HashMap::from([])),
             ("type", HashMap::from([])),
             ("time", HashMap::from([])),
             ("aspect_ratio", HashMap::from([])),
             ("format", HashMap::from([])),
            ]);
       let mut user_params = &self.params;
       let mut loop_params = &params;
        //For loop to find key value pairs that exsist and add them to built url
        //Consider map.drain() or map.iter()
        for (key, value) in &*loop_params {
            //println!("{:?} / {:#?}", key, value);
            if !(user_params.is_empty()) 
            {
                if user_params.contains_key(&key as &str) {
                    //println!("{:?}", user_params.get(&key as &str));
                    match user_params.get(&key as &str) {
                        None => {
                            println!("No value declared for {}", &key);
                        }
                        Some(uv) => {
                            //println!("{:?}", value.get(&uv as &str));
                            match value.get(&uv as &str) {
                                None => {
                                    println!("No Matching value found for {} within {:?}", &uv, &value);
                                }
                                Some(lv) => {
                                    println!("{:?}", &lv);}
                                } 
                        }
                    }
                    //println!("{:?}", value.get(&user_params.get(&key as str) as &str));
                }
            } else {
                println!("No user params detected");
            }
        }
        return "TEST".to_string();
    }

    fn file_size()
    {

    }

    fn create_directories()
    {
    
    }

    pub fn download_image()
    {

    }

    pub fn download()
    {

    }

    pub fn download_executor()
    {
    
    }

    pub fn get_next_tab()
    {
        
    }

    pub fn get_all_tabs()
    {
        
    }

    pub fn format_object()
    {

    }
}


