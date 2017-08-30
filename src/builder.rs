use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io::prelude::*;


pub fn minify(content : &String) -> String {
    println!("Minify task started ... ");
    let mut lines = content.split_whitespace();
    let mut minified_string = String::from("");
    for line in lines {
        minified_string.push_str(line);
    }
    minified_string
}
pub fn save_result_to_file(file_to_save : String, content : String) {
    println!("Saving output to {}", file_to_save);
    let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(file_to_save)
    .unwrap();
    writeln!(file, "{}", content);
}

// oversimplified
pub fn concat_files(file_list : Vec<String>, output_file : String) {

    let mut file_to_write = OpenOptions::new()
    .write(true)
    .create(true)
    .open(output_file)
    .unwrap();

    for file in file_list {
        println!("{}", file);
        // Todo log error if file not found, but do not die
        let mut f = File::open(file).expect("Cannot open file");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Cannot read from file");
        writeln!(file_to_write, "{}", contents);
    }
    print!("Concatenated files");
}


