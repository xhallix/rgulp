extern crate rgulp;

use rgulp::builder;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Rust gulp");
    let content = get_content_from_file();
    css_task(content);
}

fn get_content_from_file() -> String {
    let mut f = File::open("mockup/app.css").expect("Cannot open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Cannot read from file");
    contents
}

fn css_task(contents : String) {
    let css = builder::Css { content: contents };
    let minified_css = css.minify();
    builder::Css::save_result_to_file(String::from("output/app.min.css"), minified_css);   
}