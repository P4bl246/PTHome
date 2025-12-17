#[cfg(test)]
/// # Module `rmv_comments_mode_tests`
/// Contains tests for the `remove_comments` module.
mod rmv_comments_mode_tests{
    use PTHome::main_code::utilities::remove_comments;
    #[test]
    /// `simple_comments` Test
    fn test_simple_comments(){
        let content = "This is some code. // This is a comment.\nlet x = 5; // Another comment.\" lo\"";
        let expected = "This is some code. \nlet x = 5; \n";
        let a: Vec<char> = vec![];
        let b: Vec<&str> = vec![];
        let result = remove_comments::simple_comments(content, "//", (&a, &b), &a, false);
        assert_eq!(result, Some(expected.to_string()));
    }

}
