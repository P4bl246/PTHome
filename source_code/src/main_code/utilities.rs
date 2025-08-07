/// # Mod `general` from `utilities.rs`
/// This module provides general utility functions for file operations use in the soruce code of PTHome aplication.
/// It includes functions to remove empty lines, numerate lines, skip line numbers, and others.
/// # FUNCTIONS
/// * [`remove_empty_lines`] - Removes empty lines from a file.
/// * [`NumLines`] - A public struct with private elements to hold the configuration for handle `numerate_lines` with methods.
/// * [`NumLines::new`] - Creates a new instance of `NumLines`.
/// * [`NumLines::numerate_lines`] - Numerates the lines of a file.
/// * [`NumLines::remove_num_lines`] - Removes line numbers from a file.
/// * [`NumLines::skip_num_line`] - Skips the line number in a file.
/// * [`NumLines::get_num_line`] - Gets the current line number from a file
/// * [`NumLines::get_input_file`] - Gets the input file path.
/// * [`NumLines::get_delimiter`] - Gets the delimiter.
/// * [`NumLines::set_input_file`] - Sets the input file path.
/// * [`NumLines::set_delimiter`] - Sets the delimiter.
/// * [`all_appears_index`] - Finds all occurrences of a string in a file and returns their indices.
pub mod general{
  #![allow(unused)]
    use std::fs;
    use std::io::Write;
    /// ## `remove_empty_lines`
    /// Removes empty lines from a file and rewrites it.
    /// ### Arguments
    /// * `input_file: &str` - The path to the file from which empty lines will be removed.
    /// ### Example
    /// ```rust
    /// mod main_code;
    /// fn main (){
    /// use crate::main_code::utilities::general;
    /// let input_file = "example.txt";
    /// general::remove_empty_lines(input_file);
    /// }
    /// ```
    /// The result is a file withouth empty lines.
    /// ### Errors
    /// If the file cannot be read or written, the function will panic with an error message.
   pub fn remove_empty_lines(input_file: &str){
    println!("REMOVING VOID LINES FROM FILE: {}", input_file);
     let mut new_content = String::new();
     {
       let content= fs::read_to_string(input_file).expect(&format!("Error trying to open the file '{}'", input_file));
       
       for line in content.lines(){
         if line.is_empty(){
             continue;
         }
         new_content.push_str(&format!("{}\n",line));
       }

     }
     fs::remove_file(input_file).expect(&format!("Error trying to remove the file '{}'", input_file));

     let mut new_file = fs::File::create(input_file).expect(&format!("Error trying to remove the file '{}'", input_file));
     new_file.write_all(new_content.as_bytes()).expect(&format!("Error trying to write in the file '{}'", input_file));
     println!("VOID LINES REMOVED FROM FILE: {}", input_file);
   }
//------------------------------------------------------------------------------------------
    /// ## `num_lines`
    /// A struct to hold the configuration for the `NumLines` instance and his metods.
     pub struct NumLines {
          input_file: &'static str,
          delimiter: &'static str,
    }
    
    /// ## `impl num_lines`
    /// This implementation provides methods to handle line numbers in a file.
    /// It includes functions to push and remove line numbers, skip line numbers, and get the current line number.
    /// Include setters an getters
    /// * `get_input_file`
    /// * `get_delimiter`
    /// * `set_input_file`
    /// * `set_delimiter`
    impl NumLines{
    /// ## `new`
    /// Creates a new instance of `NumLines`.
    /// ### Arguments
    /// * `input_file: &'static str` - The path to the file to be processed.
    /// * `delimiter: &'static str` - The delimiter to be used for line numbering.
    /// ### Example
    /// ```rust
    /// mod main_code;
    /// fn main (){
    /// use crate::main_code::utilities::general;
    /// let input_file = general::NumLines::new("example.txt", " - ");
    /// input_file.numerate_lines();
    /// ```
    /// The result is a file with lines numerated.
    /// ### Return
    /// Returns a new instance of `NumLines` with the specified input file and delimiter.
    /// ### IMPORTANT
    /// If don't want use a delimiter just use an empty string `""`.
    /// **NOTE:** The default delimiter is an space `' '`
    pub fn new(input_file: &'static str, delimiter: &'static str) -> Self{
      Self {
        input_file,
        delimiter,
      }
    }
//---------------------------------------------------------------------
       /// ### `numerate_lines`
       /// Numerates the lines of a file and rewrites it
       /// #### Example
       /// ```rust
       /// mod main_code;
       /// fn main (){
       /// use crate::main_code::utilities::general;
       /// 
       /// let input_file = general::NumLines::new("example.txt", " - ");
       /// input_file.numerate_lines();
       /// }
       /// ```
       /// The result is a file with lines numerated.
       /// #### Errors
       /// If the file cannot be read or written, the function will panic with an error message.
       pub fn numerate_lines(&self){
        println!("NUMERATING LINES FROM FILE: {}", self.input_file);
        let mut new_content = String::new();

        {
            let content = fs::read_to_string(self.input_file).expect(&format!("Error trying to open the file '{}'", self.input_file));
            let mut count = 1;
            for line in content.lines(){
              if self.delimiter.is_empty(){
              new_content.push_str(&format!("{} {}\n", count, line)); 
              }
              else { 
                new_content.push_str(&format!("{}{}{}\n", count, self.delimiter, line));
              }
               
               count += 1;
            }
            
        }
        fs::remove_file(self.input_file).expect(&format!("Error trying to remove the file '{}'", self.input_file));
        let mut new_file = fs::File::create(self.input_file).expect(&format!("Error trying to remove the file '{}'", self.input_file));
        new_file.write_all(new_content.as_bytes()).expect(&format!("Error trying to write in the file '{}'", self.input_file));
        println!("LINES NUMERATED FROM FILE: {}", self.input_file);
    }
//---------------------------------------------------------------------
        /// ### `remove_num_lines`
        /// Removes line numbers from a file. Recomended use this just if you use before the function `numerate_lines`.
        /// #### Example
        /// ```rust
        /// mod main_code;
        /// fn main (){
        /// use crate::main_code::utilities::general;
        /// let input_file = general::NumLines::new("example.txt", " - ");
        /// input_file.remove_num_lines();
        /// }   
        /// ```
        /// /// The result is a file without line numbers.
        /// #### Errors
        /// If the file cannot be read or written, the function will panic with an error message
        pub fn remove_num_lines(&self) {
            println!("REMOVING LINE NUMBERS FROM FILE: {}", self.input_file);
            let mut new_content = String::new();
            {
                let content = fs::read_to_string(self.input_file).expect(&format!("Error trying to open the file '{}'", self.input_file));
                for line in content.lines() {
                    if self.delimiter.is_empty(){
                        if let Some(pos) = line.find(' ') {
                            new_content.push_str(&line[pos + 1..]);
                    }
                  }else{
                    if let Some(pos) = line.find(self.delimiter) {
                        new_content.push_str(&line[pos + self.delimiter.len()..]);
                    }
                }
                new_content.push('\n');
            }
         }
            fs::remove_file(self.input_file).expect(&format!("Error trying to remove the file '{}'", self.input_file));
            let mut new_file = fs::File::create(self.input_file).expect(&format!("Error trying to create the file '{}'", self.input_file));
            new_file.write_all(new_content.as_bytes()).expect(&format!("Error trying to write in the file '{}'", self.input_file));
            println!("LINE NUMBERS REMOVED FROM FILE: {}", self.input_file);
        }
//---------------------------------------------------------------------
        /// ### `skip_num_line`
        /// Skips the line number in a file. Use this just if you use before the function `numerate_lines`.
        /// #### Arguments
        /// * `line: &str` - The line from which the line number will be skipped.
        /// #### Example
        /// ```rust
        /// mod main_code;
        /// fn main (){
        /// use crate::main_code::utilities::fn_num_lines;
        /// let input_file = general::NumLines::new("example.txt", " - ");
        /// 
        /// let all_after_num_line = input_file.skip_num_line("1 - This is a test line.");
        /// println!("Line without number: {}", all_after_num_line);
        /// }
        /// ```
        /// #### Return
        /// Return a `String` with the line without the line numbers.
        /// Return an empty `String` if the delimiter is not found.
        pub fn skip_num_line(&self, line:&str) -> String{
            let mut new_content = String::new();
            if self.delimiter.is_empty(){
            if let Some(pos) = line.find(' ') {
                        new_content.push_str(&line[pos + 1..]);
               }
            }
            else{
            if let Some(pos) = line.find(self.delimiter) {
                        new_content.push_str(&line[pos + self.delimiter.len()..]);
                    }else{
                        println!("Delimiter '{}' not found in line: '{}'", self.delimiter, line);
                        return "".to_string();
                    }
                }
                
            return new_content; 
        }
//---------------------------------------------------------------------
        /// ### `get_num_line`
        /// Gets the current line number from a file.
        /// #### Arguments
        /// * `line: &str` - The line from which the current line number will be extracted.
        /// 
        /// #### Example
        /// ```rust
        /// mod main_code;
        ///fn main (){
        /// use crate::main_code::utilities::fn_num_lines;
        /// let input_file = general::NumLines::new("example.txt", " - ");
        /// let current_line = input_file.get_num_line("1 - This is a test line.");
        /// println!("Current line number: {}", current_line);
        /// }
        /// ```
        /// #### Return
        /// Returns the current line number as a `i32`.
        /// If the delimiter is not found, it returns an empty -1
        /// #### Errors
        /// If the line number cannot be parsed, the function will panic with an error message.
        pub fn get_num_line(&self, line:&str) -> i32{
            let mut new_content = String::new();
            if self.delimiter.is_empty(){
                if let Some(pos) = line.find(' ') {
                    new_content.push_str(&line[..pos]);
                }
            }else{
                if let Some(pos) = line.find(self.delimiter) {
                    new_content.push_str(&line[..pos]);
                }else{
                    println!("Delimiter '{}' not found in line: '{}'", self.delimiter, line);
                    return -1;
                }
            }
            return new_content.parse().expect(&format!("Error trying to parse the line number from '{}'", new_content));
        }
//---------------------------------------------------------------------   
       /// ## `get_input_file`
       /// Gets the input file path.
       /// ### Return
       /// Returns the input file path as a `String`.
       pub fn get_input_file(&self) -> String{
            self.input_file.to_string()
        }
//---------------------------------------------------------------------
       /// ## `get_delimiter`
       /// Gets the delimiter used for line numbering.
       /// ### Return
       /// Returns the delimiter as a `String`.
       pub fn get_delimiter(&self) -> String{
            self.delimiter.to_string()
        }
//---------------------------------------------------------------------
       /// ## `set_input_file`
       /// Sets the input file path.
       /// ### Arguments
       /// * `input_file: &'static str` - The new input file path to be set.
       /// ### Example
       /// ```rust
       /// mod main_code;
       /// fn main (){
       /// use crate::main_code::utilities::general;
       /// let mut input_file = general::NumLines::new("example.txt", " - ");
       /// input_file.set_input_file("new_example.txt");
       /// ```
       pub fn set_input_file(&mut self, new_value:&'static str){
            self.input_file = new_value;
        }
//---------------------------------------------------------------------
       /// ## `set_delimiter`
       /// Sets the delimiter
       /// ## Arguments 
       /// * `delimiter: &'static str` - The new delimiter to be set.
       /// ### Example  
       /// ```rust
       /// mod main_code;
       /// fn main (){
       /// use crate::main_code::utilities::general;
       /// let mut input_file = general::NumLines::new("example.txt", " - ");
       /// input_file.set_delimiter(" | ");
       /// ```
       pub fn set_delimiter(&mut self, new_value:&'static str){
            self.delimiter = new_value;
       }
       
    }
//------------------------------------------------------------------------------------------
    /// ## `all_appears_index`
    /// Finds all occurrences of a substring in a string and returns their indexes.
    /// ### Arguments
    /// * `input_str: &str` - The string in which to search for the substring.
    /// * `search_str: &str` - The substring to search for.
    /// ### Example
    /// ```rust 
    /// #[derive(Debug)]
    /// mod main_code;
    /// fn main (){ 
    /// use crate::main_code::utilities::general;
    /// let input_str = "This is a test string. This is another test string.";
    /// let search_str = "test";
    /// let indexes = general::all_appears_index(input_str, search_str);
    /// println!("Indexes of '{}' in '{}': {:?}", search_str, input_str, indexes);
    /// }
    /// ```
    /// The result is a vector with the indexes of all occurrences of the substring in the string.
    /// ### Return
    /// Returns a vector of `usize` containing the indexes of all occurrences of the substring in the string.
    /// If the substring or the string is empty, it returns an empty vector.
    /// ### Errors
    /// If the input string or the search string is empty, the function will return an empty vector.
  pub fn all_appears_index(input_str:&str, search_str: &str) -> Vec<usize>{
    let mut indexes = Vec::new();
    let mut copy = input_str.to_string();
    let mut remove = 0;
    if search_str.is_empty() || input_str.is_empty() {
        return indexes;
    }
    else{
      while copy.contains(search_str){
         if let Some(index) = copy.find(search_str){
          indexes.push(index + (remove*search_str.len()));
          //remove this appear
          copy = copy.replacen(search_str, "", 1);
          remove += 1;
         }
       }
    }
    return indexes;
     
  }
}
//------------------------------------------------------------------
/// # Mod `remove_comments` from `utilities.rs`
/// This module provides functions to remove comments from files.   
/// # FUNCTIONS
/// * [`remove_comments::simple_comments`] - Removes simple comments from the file.
/// * [`remove_comments::block_comments`] - Removes block comments from the file.
pub mod remove_comments{
  #![allow(unused)]
    use std::fs;
    use std::io::Write;

    /// ## `simple_comments`
    /// Removes simple comments from a file and rewrites it.
    /// ### Arguments
    /// * `input_file: &str` - The path to the file from which simple comments will be removed.
    /// * `delimiter: &str` - The delimiter used to identify simple comments.
    /// ### Example
    /// ```rust 

    /// mod main_code;
    /// fn main (){
    /// use crate::main_code::utilities::remove_comments;
    /// let input_file = "example.txt";
    /// remove_comments::remove_simple_comments(input_file, "//");
    /// }
    /// ```
    /// The result is a file with simple comments removed.
    /// ### Errors
    /// If the file cannot be read or written, the function will panic with an error message.
    /// ### Note
    /// The function will remove everything after the first occurrence of the delimiter in each line.
    
    pub fn simple_comments(input_file: &str, delimiter: &str){
        println!("REMOVING SIMPLE COMMENTS FROM FILE: {}", input_file);
      let mut new_content = String::new();
      {
        let file_content = fs::read_to_string(input_file).expect(&format!("Failed to read the file '{}'", input_file));
        for line in file_content.lines() {
          if let Some(pos) = line.find(delimiter) {
            let new_line = &line[..pos];
            new_content.push_str(new_line);
            new_content.push('\n');
          } else {
            new_content.push_str(line);
            new_content.push('\n');
          }
        }
      }
      fs::remove_file(input_file);
      let mut file = fs::File::create(input_file).expect(&format!("Failed to create the file '{}'", input_file));
      file.write_all(new_content.as_bytes()).expect(&format!("Failed to write to the file '{}'", input_file));
        println!("SIMPLE COMMENTS REMOVED FROM FILE: {}", input_file);
    }
//------------------------------------------------------------------
    /// ## `ModeBlock`
    /// An enum to specify the mode of block comment removal.
    /// ### Variants
    /// * `Nested` - Removes nested block comments. This mode will handle block comments that may contain other block comments within them, and ensure that nested comments are properly closed.
    /// * `Single` - Removes single block comments. This mode will handle block comments that are not nested and will remove them in a single pass, without considering nested structures.
     pub enum ModeBlock{
        Nested,
        Single
     }
    /// ## `block_comments`
    /// Removes block comments from a file and rewrites it. 
    /// * This function is an API for the functions [`single_mode`] and [`nested_mode`].
    /// ### Arguments
    /// * `input_file: &str` - The path to the file from which block comments will be removed.
    /// * `start_delimiter: &str` - The starting delimiter of the block comment.
    /// * `end_delimiter: &str` - The ending delimiter of the block comment.
    /// * `mode: ModeBlock` - The mode of block comment removal, either [`ModeBlock::Nested`] or [`ModeBlock::Single`]
    /// ### Example
    /// ```rust
    /// mod main_code;
    /// fn main (){
    /// use crate::main_code::utilities::remove_comments;
    /// let input_file = "example.txt";
    /// remove_comments::remove_block_comments(input_file, "/*", "*/", ModeBlock::Single);
    /// }
    /// ```
    /// The result is a file with block comments removed.
    /// ### Errors
    /// If the file cannot be read or written, the function will panic with an error message
    /// If there is a block comment without end delimiter, the function will return -1 with an error message.
    /// ### Return
    /// * `-1` - If there is a block comment without an end delimiter.
    /// * `0` - If the block comments were successfully removed.
    
    pub fn block_comments(input_file: &str, start_delimiter: &str, end_delimiter: &str, mode: ModeBlock) -> i32{
      println!("REMOVING BLOCK COMMENTS FROM FILE: {}", input_file);
      let mut new_content = String::new();
      match mode{
      ModeBlock::Single =>{
        let file_content = fs::read_to_string(input_file).expect(&format!("Failed to read the file '{}'", input_file));
        match single_mode(&file_content, start_delimiter, end_delimiter){
            Ok(content) =>  new_content.push_str(&content) ,
            Err(_) => return -1
        }
       }
       ModeBlock::Nested =>{
        let file_content = fs::read_to_string(input_file).expect(&format!("Failed to read the file '{}'", input_file));
        match nested_mode(&file_content, start_delimiter, end_delimiter){
          Ok(content) => new_content.push_str(&content),
          Err(_) => return -1
        }
       }
      }
      fs::remove_file(input_file);
      let mut file = fs::File::create(input_file).expect(&format!("Failed to create the file '{}'", input_file));
      file.write_all(new_content.as_bytes()).expect(&format!("Failed to write to the file '{}'", input_file));
      println!("BLOCK COMMENTS REMOVED FROM FILE: {}", input_file);
      return 0;
    }
//------------------------------------------------------------------------------------------
    /// ## `single_mode`
    /// Removes block comments in single mode from a line.
    /// ### Arguments
    /// * `content: &Vec<&str>` - A vector of lines from content from which block comments will be removed.
    /// * `delimiter_end: &str` - The ending delimiter of the block comment.
    /// * `delimiter_start: &str` - The starting delimiter of the block comment.
    /// * **NOTE:** This is use in his API [`remove_comments::remove_block_comments`] fuction.
    /// ### Return
    /// Returns a `Result<String, i32>`:
    /// * `Ok(String)` - If the block comments were successfully removed, returns a `String` with the content without block comments.
    /// * `Err(i32)` - If there is an error, returns an `i32` error code:
    ///   - `-1` - If the start and end delimiters are the same or content vector is empty.
    ///   - `2` - If the block comment are not closed and arrive to the end of content vector, with an error message indicating the line number and content of the line.
 fn single_mode(content: &str, delimiter_start: &str, delimiter_end: &str) -> Result<String, i32>{
        if delimiter_start == delimiter_end{
            println!("Error: The start and end delimiters are the same.");
            return Err(-1);
        }
        if content.is_empty(){
            println!("Error: The content vector is empty");
            return Err(-1);
        }
       
          
         let mut line_content = String::new();//buffer for store the line when start the block comment
         let mut block_open = false; // flag to indicate if a block comment is open
         let mut new_content = String::new(); // buffer for store the new content without block comments
         let mut counter = 0;// counter for the line number
         let mut line_num = 0;// line number where the block comment starts
         let mut multi_line = false; // flag to indicate if the block comment is multi-line
         // Iterate through each line in the content
         // This is a single mode, so we don't need to handle nested comments
         for line in content.lines() {
          counter += 1; // Increment the line counter
           let mut line_copy= line.to_string(); // copy the line for handle his content
           // Check if the line contains the start delimiter or if a block comment is already open
           if line_copy.contains(delimiter_start) || block_open{ 
              // While the line contains the start delimiter
               while line_copy.contains(delimiter_start){
                // If a block comment isn't already open, set the line number and content
                // This made for take and store the data for the error message if the block comment isn't closed
                if !block_open{
                  line_num = counter;
                  line_content = line.to_string();
                }
                // Find the position of the start delimiter in the line
               if let Some(start_pos) = line_copy.find(delimiter_start){
                // If the start delimiter is found, check if a block comment is already open
                // If not, push the content before the start delimiter to the new content
                if !block_open {new_content.push_str(&line_copy[..start_pos]); block_open = true;}
                // If the start delimiter is found, check if the end delimiter is also present in the line
                if let Some(end_pos) = line_copy.find(delimiter_end){
                    // For preserved code between comments, but no inside of any of them, in other words, code between start and end block comments delimiters.
                    // The comp its this, becuase the code between comments, is in the start and end of comment, like this '/*thi*/between/*other*/', like we look here, the start delimiter have a greater index than end delimiter
                    //and the content "between" starts after the end delimiter, so we can push en_pos+delimiter_end.len(), and need been not multi-line, because the while loop and all this flux into the for-loop trate with a single line, 
                    //so we need have a way to indicate the comment in some line where open a block comment, is not close, therefore, all after this start must be skiped and ignored.
                    if start_pos > end_pos+delimiter_end.len() && !multi_line{
                    new_content.push_str(&line_copy[end_pos+delimiter_end.len()..start_pos]);
                    // Remove the end delimiter from the line copy to continuing process the next start block comment in the line
                    line_copy.replace_range(end_pos..end_pos+delimiter_end.len(), "");
                    block_open = true;
                     }
                    }
                    // Remove the start delimiter from the line copy, for not process this again, and avoid problems
                    line_copy.replace_range(start_pos..start_pos+delimiter_start.len(), "");
                }
                
              }
             //pass here when the line hasn't more start delimiters
              if let Some(end_pos) = line_copy.find(delimiter_end){
                // If a block comment is open and the end delimiter is found, push the content after the end delimiter to the new content
                // and close the block comment
                if block_open{
                new_content.push_str(&line_copy[end_pos+delimiter_end.len()..]);
                new_content.push('\n');
                block_open = false;
                }
              }
              //indicate its a multi-line block comment
            else{
                block_open = true;
                multi_line = true;
                continue;
              }
             }
             // If the line doesn't contain the start delimiter and a block comment is not open, push the line to the new content
            else if !block_open{
             new_content.push_str(&line_copy);
             new_content.push('\n');
           }
          
         }
          // If a block comment is open at the end of the content, return an error
         if block_open {
            println!("Error: Block comment without end delimiter in line '{}': '{}'", line_num, line_content);
            return Err(2);
           }
         return Ok(new_content);                
    }

//------------------------------------------------------------------------------------------
    /// ## `nested_mode`
    /// Removes block comments in nested mode from a line.
    /// ### Arguments
    /// * `content: &str` - A string containing the content from which block comments will be removed.
    /// * `delimiter_start: &str` - The starting delimiter of the block comment.
    /// * `delimiter_end: &str` - The ending delimiter of the block comment.
    /// * **NOTE:** This is use in his API [`remove_comments::remove_block_comments`] fuction.
    /// ### Return
    /// Returns a `Result<String, i32>`:
    /// * `Ok(String)` - If the block comments were successfully removed, returns a `String` with the content without block comments.
    /// * `Err(i32)` - If there is an error, returns an `i32` error code:
    ///   - `-1` - If the start and end delimiters are the same or content vector is empty.
    ///   - `2` - If the block comment are not closed and arrive to the
    fn nested_mode(content: &str, delimiter_start: &str, delimiter_end: &str)-> Result<String, i32>{
       use crate::main_code::utilities::general;
       if content.is_empty(){
        println!("Error: The content vector is empty");
        return Err(-1);
       }
       if delimiter_start == delimiter_end{
        println!("Error: The start and end delimiters are the same.");
        return Err(-1);
       }
       let mut new_content = String::new();
       let mut in_block_comment = false;
       let mut is_multiline = false;
       let mut block_comment_level:usize = 0;
       let mut end = 0;
       let mut start = 0;
        let mut line_num = 0;
        let mut line_content = String::new();
        let mut counter = 0;
        let mut indexes:Vec<usize> = Vec::new();
        let mut processed = false; 
        let mut indexes_end: Vec<usize> = Vec::new();
        // Iterate through each line in the content
        // This is a nested mode, so we must to handle nested comments
        for line in content.lines(){
         counter += 1;
          let mut line_copy = line.to_string();
          // Check if the line contains the start or end delimiter
        if line_copy.contains(delimiter_end) || line_copy.contains(delimiter_start){
          // If the line contains the end delimiter, find all occurrences of it
          // and store their indexes in the indexes_end vector
          if line_copy.contains(delimiter_end){
          indexes_end = general::all_appears_index(&line_copy, delimiter_end);
          }
          // If the line contains the start delimiter, find all occurrences of it
          // and store their indexes in the indexes vector
           if line_copy.contains(delimiter_start){
             start = line_copy.find(delimiter_start).unwrap();
            line_num = counter;
            line_content = line.to_string();
            // If a block comment is not already open, push the content before the start delimiter to the new content
            if !in_block_comment{
              new_content.push_str(&line_copy[..start]);
              in_block_comment = true;
            }
            let indexes_start_in_line = general::all_appears_index(&line_copy, delimiter_start);
            for i in indexes_start_in_line.iter(){
              // Store the indexes of the start delimiters in the indexes vector
              indexes.push(*i);
            }
          }
         } 
         else{
          processed = false;
         }
         // Next, we check if the line is not empty and if it contains the start delimiter
         // If it does because we need to handle the block comment
        if !indexes.is_empty(){
          let mut indexes_to_delete:Vec<usize> = Vec::new();
          // We need to check if the indexes_end are in the indexes vector
          // If it does because we need to handle conflicts between, end delimiter and start delimter
          // its occurs when the end delimiter and start delimiter superpose, like this '/*/', 
          //in this example the start delimiter are in the first and second character ('/*'), but the end delimiter
          // are in the second and third character ('*/'), so, for avoid this problems the code priorize the end delimiter
          // and remove the start delimiter from the vector indexes, therefore, this '/*/' are interpreting like a end delimiter
          // and the case when the end delimiter has the same index than the start delimiter can't appear.
          // The index to remove in the indexes vector is store into the vector indexes_to_delete
          for(i, value) in indexes.iter().enumerate(){
            if indexes_end.contains(&(value+delimiter_start.len()-1)){
              indexes_to_delete.push(i);
            }
          }
          // Here remove them index from the vector indexes
          for i in indexes_to_delete.iter(){
            indexes.remove(*i);
          }
          let i = 0;// We need to use this index for the while loop
         block_comment_level += indexes.len();// This is the number of block comments intialize in the line
         // If the block comment level is greater than 0, we need to handle the block
         if block_comment_level > 0 || in_block_comment{
          // We need to handle the end delimiter, because we need to remove the block comment
          // We need to check if the end delimiter is in the indexes_end vector
          while !indexes_end.is_empty() && !(indexes_end.len() <= 0){
            // If the end delimiter is in the indexes_end vector, we need to handle the block comment
            // We need to check if the end delimiter is grether than the start delimiter at the first time or index[0] for both vectors
             if indexes_end[i] > indexes[i]+delimiter_start.len()-1{
              // If the end delimiter is greater than the start delimiter, we need to handle the block comment
              if indexes.len() > i+1{
                // If the end delimiter is less than the next start delimiter, and not its a nested block comment, or we are not be in a block comment
                // We need to push the content between the end delimiter and the next start delimiter to the new content
                // And remove this level, from the vectors and block_comment_level counter
               if indexes_end[i] < indexes[i+1] && !in_block_comment{
                 new_content.push_str(&line_copy[indexes_end[i]+delimiter_end.len()..indexes[i+1]]);
                 indexes_end.remove(i);
                 indexes.remove(i);
                  block_comment_level -= 1;
                 
                }
                // If the end delimiter is greater than the next start delimiter
                // we mark this as a in_block_comment, because we are in a nested block comment
                // and in this iteration that end delimiter were the end delimiter for this nested block comment, 
                //and now we can remove this, because this are be closed, and up to next leve of block comment.
                else{
                  in_block_comment = true;
                  indexes_end.remove(i);
                   indexes.remove(i);
                   block_comment_level -= 1;
                 }
                 continue;
               }
               // If the indexes are equal 1 or i+1 but i is even 0, 
               // that means that we are in the last layer of the block comments or the first block comment
               // therefore, the end delimiter is the end of the block comment, and can copy the value after this
               else if indexes.len() == 1 && indexes_end.len() >= 1 && block_comment_level == 1{
                 new_content.push_str(&line_copy[indexes_end[i]+delimiter_end.len()..]);
                 indexes_end.remove(i);
                 indexes.remove(i);
                 block_comment_level -= 1;
                 in_block_comment = false;
                 is_multiline = false;
                 processed = true;
               }
               // if not has more end delimiters this level of block comment its multi-line
               else{
                  in_block_comment = true;
                  is_multiline = true;
                 break;
               }

             }
             
          }
        }
       }
       if !in_block_comment && !processed{
         new_content.push_str(&line_copy);
         new_content.push('\n');
       }
      }
        if in_block_comment || block_comment_level > 0{
          println!("Error: Block comment without end delimiter in line '{}': '{}'", line_num, line_content);
          return Err(2);
        }
        return Ok(new_content);
    
        
  }

//------------------------------------------------------------------------------------------

}
