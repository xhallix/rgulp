use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io::prelude::*;

struct MainBuilder {
    content: String
}

pub struct Css {
    pub content : String
}

impl MainBuilder {
    fn minify(&self) -> String {
        println!("Minify task started ... ");
        let mut lines = self.content.split_whitespace();
        let mut minified_string = String::from("");
        for line in lines {
            minified_string.push_str(line);
        }
        minified_string
    }
    fn save_result_to_file(file_to_save : String, content : String) {
        println!("Saving output to {}", file_to_save);
        let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_to_save)
        .unwrap();
        writeln!(file, "{}", content);
    }
}

impl Css {
    pub fn minify(&self) -> String {
        let main_builder = MainBuilder{ content : self.content.to_string() }; 
        main_builder.minify()
    }

    pub fn save_result_to_file(file_to_save : String, content : String) {
        MainBuilder::save_result_to_file(file_to_save, content)
    }
}
