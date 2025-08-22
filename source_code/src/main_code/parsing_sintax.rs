/// This module provide functions to normalize files
/// It usees [`crate::main_code::utilities::{general, remove_comments}`] for the actual implementations. 
/// Contains the API for the user.

pub mod normalize_file{
    #![allow(unused)]
    use std::fs;
    use crate::main_code::utilities::{general, remove_comments};
    pub fn normalize_file(path: &str) -> Option<String> {      
         let remove_chr = vec!['"', '"'];
         let rmv_str = vec!["'", "'"];
         let mut processed = String::new();
        let ignore_into = (&remove_chr, &rmv_str);
        let content= fs::read_to_string(path).unwrap_or(format!("Error when try to read the file '{}'", path));
        let scape = vec!['\\'];
        match remove_comments::simple_comments(&content, "//", ignore_into, &scape, true){
             Some(i) => processed = i,
            None=> return None,
        };
        match remove_comments::block_comments(&processed, "/*", "*/", ignore_into, &scape, remove_comments::ModeBlock::Nested, remove_comments::ManageClose::Both) {
            Some(i) => processed = i,
            None=> return None,
        };
        let copy = processed.to_string();
        let num_line= general::NumLines::new(&copy,  " ");
        processed = num_line.numerate_lines();
        
       {
        let mut new_content = String::new();
        for line in processed.lines(){
            let line2 = num_line.skip_num_line(line);
            if !line2.is_empty(){
                new_content.push_str(line);
            }
         }
         processed = new_content;
        }
        return Some(processed);  
    }
    
}

 pub mod parser{
    
 }