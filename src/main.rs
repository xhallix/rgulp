extern crate rgulp;

use rgulp::builder;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Rust gulp");
    let css_content = get_content_from_file("mockup/app.css");
  //  let js_content = get_content_from_file("mockup/app.js");
    css_task(&css_content);
 //   js_task(&js_content);
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

fn css_task(contents : &String) {
    let minified_css = builder::minify(contents);
    builder::save_result_to_file(String::from("output/app.min.css"), minified_css);   
}

fn js_task(contents : &String) {
    let minified_js = builder::minify(contents);
    builder::save_result_to_file(String::from("output/app.min.js"), minified_js); 
} 