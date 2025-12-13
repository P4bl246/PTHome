#[cfg(test)]
/// # Module `rmv_comments_mode_tests`
/// Contains tests for the `remove_comments` module.
mod rmv_comments_mode_tests{
    use crate::main_code::utilities::remove_comments;
    #[test]
    /// Test for simple_comments function
    fn test_simple_comments(){
        let content = "This is some code. // This is a comment.\nlet x = 5; // Another comment.";
        let expected = "This is some code. \nlet x = 5; ";
        let result = remove_comments::simple_comments(content, "//");
        assert_eq!(result, expected);
    }
}
