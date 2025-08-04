/// This module provides functionality to normalize files by removing empty lines and numbering the lines.
/// It usees [`crate::main_code::utilities::general`] for the actual implementations. 
/// Contains the API for the user.
/// # FUNCTIONS
/// 
pub mod normalize_file{
  pub mod remove_comments{
    pub fn remove_simple_comments(input_file: &str, delimiter: &str){
        
    }
//------------------------------------------------------------------
    pub fn remove_block_comments(input_file: &str, start_delimiter: &str, end_delimiter: &str){}

  }
} 