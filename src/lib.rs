pub mod builder;
pub mod config;

#[cfg(test)]
mod tests {
    use super::*;
    use builder;

    #[test]
    fn builder_can_minify() {
        let css_string = "p {
                            text-align: center;
                            color: red;
                        } ";
        let fixture = "p{text-align:center;color:red;}";
        assert_eq!(fixture, builder::minify( &String::from( css_string ) )) ;
    }

     #[test]
    fn builder_can_save_to_file() {
        let css_string = String::from("p{text-align:center;color:red;}");
        let string_to_save = String::from("this should be saved");
        builder::save_result_to_file(String::from("./output/text.txt"), css_string);
    }

    #[test]
    fn builder_can_concatenate_files() {
        let file_one : String = String::from("./mockup/app.css");
        let file_two : String = String::from("./mockup/home.css");
        let files_to_concat = vec![file_one, file_two];
        builder::concat_files(files_to_concat, String::from("output/app.concat.min.css"))
    }
}


