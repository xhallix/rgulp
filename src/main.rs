extern crate rgulp;
extern crate serde_json;

use self::serde_json::Value;
use rgulp::builder;
use rgulp::config::Config;
use std::fs::File;
use std::io::prelude::*;


fn main() {

    println!("Rust gulp");
    let json_config : Value = Config::read_as_json(get_content_from_file("mockup/rsconfig.json"));
    let css_config = &json_config["css"];
    let js_config = &json_config["jss"];

    if *css_config != Value::Null {
        let css_files = &css_config["files"];
        let out_file = &css_config["out"];
        for file in css_files.as_array().unwrap() {
            css_task(file.as_str().unwrap(), out_file.as_str().unwrap());
        }      
    }

    if *js_config != Value::Null {
        js_task("mockup/app.js");
    }


}


///
/// Retrieves all content from the specified file
///
fn get_content_from_file(file_name : &str) -> String {
    let mut f = File::open(file_name).expect(&(file_name.to_string() + &" <-- File not found".to_string()));
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Cannot read from file");
    contents
}

///
/// Runs the tasks for CSS
///
fn css_task(file_name : &str, out_file : &str) {
    let content = get_content_from_file(file_name);
    let minified_css = builder::minify(&content);
    builder::save_result_to_file(String::from(out_file), minified_css);   
}

///
/// Runs the tasks for JS 
///
fn js_task(file_name : &str) {
    let content = get_content_from_file(file_name);
    let minified_js = builder::minify(&content);
    builder::save_result_to_file(String::from("output/app.min.js"), minified_js); 
} 