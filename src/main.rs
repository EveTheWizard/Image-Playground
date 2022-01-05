use std::env;
use std::fs;
use std::io;
use std::collections::HashMap;
use std::path::Path;
use chrono;
extern crate clap;
use clap::{Arg, App};
extern crate tensorflow;
//use image_downloader::*;
mod image_downloader;
use crate::image_downloader::Image_Downloader;

fn main() {

    //SETUP THE COMMANDLINE ARGUMENTS-----------------------------------------------------------
    let matches = App::new("Image Playground V1, No GUI")
        .version("0.1.0")
        .author("Eve Daily <evelyn.r.daily@gmail.com>")
        .about("Scrapes Art off the internet and generates images based on a selected algoritm")
        .arg(Arg::with_name("search")
                 .short("s")
                 .long("search")
                 .takes_value(true)
                 .help("Please insert a search term for the program to pull from, ex Horse"))
        .arg(Arg::with_name("folder")
                 .short("f")
                 .long("folder")
                 .takes_value(true)
                 .help("Alternate to search function, give a folder path full of images"))  
        .arg(Arg::with_name("num_images")
                 .short("i")
                 .long("images")
                 .takes_value(true)
                 .help("The number of images to be fed into the algoritm"))
        //TODO SETUP AGLORITH OPTIONS
        .arg(Arg::with_name("algorithm")
                 .short("a")
                 .long("algorithm")
                 .takes_value(true)
                 .help("The algorithm that will be used to generate the art, \n Options: fractal, ")) 
        .arg(Arg::with_name("ratio")
                 .short("r")
                 .long("ratio")
                 .takes_value(true)
                 .help("The ratio that the algorithm will use in order to determine its art"))
        .arg(Arg::with_name("num_generated")
                 .short("n")
                 .long("generated")
                 .takes_value(true)
                 .help("The number of images the software will generate"))
        .arg(Arg::with_name("temp_files")
                 .short("t")
                 .long("temp")
                 .takes_value(false)
                 .help("Sets whether the downlaoded image files will be temporary or not"))  
        .get_matches();
    //-------------------------------------------------------------------------------------


    //Determine if using Folderpath or Search Term and if they are valid
    let search_term = matches.value_of("search");
    let folder_path = matches.value_of("folder");
    let mut download_directory = "";
    match search_term {
        None => {
            println!("No search term found");
            match folder_path {
                None => {
                    println!("No Search Term or Folder Path Specified");
                }
                Some(f) => {
                    match f.parse::<String>() {
                        Ok(f_path) => {
                            println!("The Folder Path is: {}", f_path);
                            let download_directory = f_path;
                        }
                        Err(_) => println!("Folder Path is not valid"),
                    }
                } 
            }
            //println!("Folder path variable: {}", folder_path);
        }
        Some(s) => {
            match s.parse::<String>() {
                Ok(s_term) => {
                    println!("The Search Term is : {}", s_term);
                    let download_directory = &create_download_folder(&s_term);
                    println!("{}", &download_directory);
                    aquire_images_from_search(&s_term, &download_directory);
                    }
                Err(_) => println!("Not a valid search term! {}", s),
            }
        }
    }
    //DIRECTORY ESTABLISHED AND PICTURES DOWNLOADED
    


    println!("END OF PROGRAM");
}

//USE A LIST OF URLS AND DOWNLOAD EACH ONE TO THE DESIGNATED DIRECTORY
fn aquire_images_from_search(s_term: &str, dir: &str) -> Result<(), io::Error> 
    {
        let mut img_download = Image_Downloader::new(vec!["keyword test".to_string(),
                                                          "keyword test 2".to_string()], 
                                                     vec!["format test".to_string(),
                                                          "format test 2".to_string()],
                                                     HashMap::from([
                                                        ("color".to_string(), "red".to_string()),
                                                        ("type".to_string(), "test".to_string())
                                                     ])
                                                    );
        img_download.build_url_parameters();
        Ok(())
    }

//CREATE A DOWNLOAD FOLDER AND RETURN THE STRING VALUE FOR THE DIRECTORY
fn create_download_folder(s_term: &str) -> String 
    {
        let mut download_path: String = "".to_owned();
        download_path.push_str(s_term);
        download_path.push_str("-");
        download_path.push_str(&chrono::offset::Utc::now().time().to_string());
        println!("{}", download_path );
        fs::create_dir_all(&download_path);
        return String::from(download_path);
    }

//GENERATE A LIST OF IMAGE URLS AND RETURN A VECTOR OF URL STRINGS
fn generate_url_list(s_thaserm: &str) -> Result<(), io::Error>
{
    Ok(())
    
}

//DOWNLOAD A IMAGE BASED ON AN INPUT URL
fn download_image_url(url: &str) -> Result<(), io::Error>
    {
       //let img_bytes = reqwest::blocking::get(&url)?
       //    .bytes()?;
       //let image = image::load_from_memory(&img_bytes)?;
       Ok(())
    }   
