pub mod builder;

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
        let css = builder::Css{ content : css_string.to_string() };
        assert_eq!(fixture, css.minify());
    }

     #[test]
    fn builder_can_save_to_file() {
        let css_string = String::from("p{text-align:center;color:red;}");
        let string_to_save = String::from("this should be saved");
        builder::Css::save_result_to_file(String::from("./output/text.txt"), css_string);
    }
}


