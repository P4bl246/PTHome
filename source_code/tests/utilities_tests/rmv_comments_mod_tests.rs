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

    // ==================== Tests para nested_mode refactorizado ====================
    
    #[test]
    /// Test: Comentarios anidados básicos
    fn test_nested_comments_basic(){
        let content = "A /* B /* C */ D */ E";
        let vec_char: Vec<char> = vec![];
        let vec_str: Vec<&str> = vec![];
        let tuple = (&vec_char, &vec_str);
        let scape: Vec<char> = vec![];
        
        let result = remove_comments::block_comments(
            content, "/*", "*/", tuple, &scape, 
            remove_comments::ModeBlock::Nested, 
            remove_comments::ManageClose::Both
        );
        
        assert!(result.is_ok());
        let output = result.unwrap();
        // Debe quedar "A  E"
        assert!(output.contains("A"));
        assert!(output.contains("E"));
        assert!(!output.contains("B"));
        assert!(!output.contains("C"));
        assert!(!output.contains("D"));
    }

    #[test]
    /// Test: String con falsos delimitadores
    fn test_string_with_fake_delimiters(){
        let content = r#"let x = "/*"; /* real comment */ code"#;
        let vec_char: Vec<char> = vec![];
        let vec_str: Vec<&str> = vec![];
        let tuple = (&vec_char, &vec_str);
        let scape: Vec<char> = vec![];
        
        let result = remove_comments::block_comments(
            content, "/*", "*/", tuple, &scape,
            remove_comments::ModeBlock::Nested,
            remove_comments::ManageClose::Both
        );
        
        assert!(result.is_ok());
        let output = result.unwrap();
        // El string "/*" debe permanecer
        assert!(output.contains(r#""/*""#));
        assert!(output.contains("code"));
        assert!(!output.contains("real comment"));
    }

    #[test]
    /// Test: Delimitadores superpuestos
    fn test_overlapping_delimiters(){
        let content = "/*/**/*/code";
        let vec_char: Vec<char> = vec![];
        let vec_str: Vec<&str> = vec![];
        let tuple = (&vec_char, &vec_str);
        let scape: Vec<char> = vec![];
        
        let result = remove_comments::block_comments(
            content, "/*", "*/", tuple, &scape,
            remove_comments::ModeBlock::Nested,
            remove_comments::ManageClose::Both
        );
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("code"));
    }

    #[test]
    /// Test: Secuencias de escape en strings
    fn test_escape_sequences(){
        let content = r#"let y = '\''; /* Comment */"#;
        let vec_char: Vec<char> = vec![];
        let vec_str: Vec<&str> = vec![];
        let tuple = (&vec_char, &vec_str);
        let scape: Vec<char> = vec![];
        
        let result = remove_comments::block_comments(
            content, "/*", "*/", tuple, &scape,
            remove_comments::ModeBlock::Nested,
            remove_comments::ManageClose::Both
        );
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains(r#"'\''"#));
        assert!(!output.contains("Comment"));
    }

    #[test]
    /// Test: Comentario sin cerrar (debe retornar error)
    fn test_unclosed_comment_error(){
        let content = "code /* unclosed";
        let vec_char: Vec<char> = vec![];
        let vec_str: Vec<&str> = vec![];
        let tuple = (&vec_char, &vec_str);
        let scape: Vec<char> = vec![];
        
        let result = remove_comments::block_comments(
            content, "/*", "*/", tuple, &scape,
            remove_comments::ModeBlock::Nested,
            remove_comments::ManageClose::Both
        );
        
        // Debe retornar error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), 2);
    }

    #[test]
    /// Test: Comentario multi-línea
    fn test_multiline_comment(){
        let content = "line1\n/* start\ncomment line\nend */ line2";
        let vec_char: Vec<char> = vec![];
        let vec_str: Vec<&str> = vec![];
        let tuple = (&vec_char, &vec_str);
        let scape: Vec<char> = vec![];
        
        let result = remove_comments::block_comments(
            content, "/*", "*/", tuple, &scape,
            remove_comments::ModeBlock::Nested,
            remove_comments::ManageClose::Both
        );
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("line1"));
        assert!(output.contains("line2"));
        assert!(!output.contains("comment line"));
    }

    #[test]
    /// Test: Múltiples comentarios en una línea
    fn test_multiple_comments_one_line(){
        let content = "a /* c1 */ b /* c2 */ c";
        let vec_char: Vec<char> = vec![];
        let vec_str: Vec<&str> = vec![];
        let tuple = (&vec_char, &vec_str);
        let scape: Vec<char> = vec![];
        
        let result = remove_comments::block_comments(
            content, "/*", "*/", tuple, &scape,
            remove_comments::ModeBlock::Nested,
            remove_comments::ManageClose::Both
        );
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("a"));
        assert!(output.contains("b"));
        assert!(output.contains("c"));
        assert!(!output.contains("c1"));
        assert!(!output.contains("c2"));
    }

    #[test]
    /// Test: Anidamiento profundo (3 niveles)
    fn test_deep_nesting(){
        let content = "x /* l1 /* l2 /* l3 */ l2b */ l1b */ y";
        let vec_char: Vec<char> = vec![];
        let vec_str: Vec<&str> = vec![];
        let tuple = (&vec_char, &vec_str);
        let scape: Vec<char> = vec![];
        
        let result = remove_comments::block_comments(
            content, "/*", "*/", tuple, &scape,
            remove_comments::ModeBlock::Nested,
            remove_comments::ManageClose::Both
        );
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("x"));
        assert!(output.contains("y"));
        assert!(!output.contains("l1"));
        assert!(!output.contains("l2"));
        assert!(!output.contains("l3"));
    }

}

