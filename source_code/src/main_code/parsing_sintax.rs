/// This module provide functions to normalize files
/// It usees [`crate::main_code::utilities::{general, remove_comments}`] for the actual implementations. 
/// Contains the API for the user.

pub mod normalize_file{
    use crate::main_code::utilities::{general, remove_comments};
    pub fn normalize_file(path: &str) -> Result<(), String> {
        general::remove_empty_lines(path);
         let remove_chr = vec!['"', '"'];
         let rmv_str = vec!["'", "'"];
        let ignore_into = (remove_chr, rmv_str);
        remove_comments::simple_comments(path, "//", ignore_into);
        match remove_comments::block_comments(path, "/*", "*/", remove_comments::ModeBlock::Nested) {
            0 => Ok(()),
            -1 => Err(format!("Error removing block comments: {}", path)),
            _ => Err("Unknown error".to_string()),
            
        }
        
    }
} 