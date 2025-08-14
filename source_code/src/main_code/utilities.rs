/// # Mod `general` from `utilities.rs`
/// This module provides general utility functions for file operations use in the soruce code of PTHome aplication.
/// It includes functions to remove empty lines, numerate lines, skip line numbers, and others.
pub mod general{
  #![allow(unused)]
    use std::fs;
    use std::io::Write;
    /// # `remove_empty_lines`
    /// Removes empty lines from a file and rewrites it.
    /// # Arguments
    /// * `content: &str` - The content from which empty lines will be removed.
    /// # Example
    /// ```rust
    /// mod main_code;
    /// fn main (){
    /// use crate::main_code::utilities::general;
    /// let input_file = "example.txt";
    /// general::remove_empty_lines(input_file);
    /// }
    /// ```
    /// The result is a file withouth empty lines.
    /// # Errors
    /// If the file cannot be read or written, the function will panic with an error message.
   pub fn remove_empty_lines(content: &str) -> String{
    println!("REMOVING VOID LINES FROM CONTENT: {}", content);
     let mut new_content = String::new();
     {       
       for line in content.lines(){
         if line.is_empty(){
             continue;
         }
         new_content.push_str(&format!("{}\n",line));
       }

     }
     println!("VOID LINES REMOVED FROM FILE: {}", content);
     return new_content;
   }
//------------------------------------------------------------------------------------------
    /// # `num_lines`
    /// A struct to hold the configuration for the `NumLines` instance and his metods.
     pub struct NumLines<'a> {
          content: &'a str,
          delimiter: &'a str,
    }
    
    /// # `impl num_lines`
    /// This implementation provides methods to handle line numbers in a file.
    /// It includes functions to push and remove line numbers, skip line numbers, and get the current line number.
    /// Include setters an getters
    /// * `get_input_file`
    /// * `get_delimiter`
    /// * `set_input_file`
    /// * `set_delimiter`
    impl<'a> NumLines<'a>{
    /// # `new`
    /// Creates a new instance of `NumLines`.
    /// # Arguments
    /// * `content: &'a str` - The str to be processed.
    /// * `delimiter: &'a str` - The delimiter to be used for line numbering.
    /// # Example
    /// ```rust
    /// mod main_code;
    /// fn main (){
    /// use crate::main_code::utilities::general;
    /// let instance = general::NumLines::new("example.txt\n   \nsdf", " - ");
    /// }
    /// ```
    /// # Return
    /// Returns a new instance of `NumLines` with the specified input file and delimiter.
    /// # IMPORTANT
    /// If don't want use a delimiter just use an empty string `""`.
    /// **NOTE:** The default delimiter is an space `' '`
    pub fn new(content: &'a str, delimiter: &'a str) -> Self{
      Self {
        content:content,
        delimiter:delimiter,
      }
    }
//---------------------------------------------------------------------
       /// # `numerate_lines`
       /// Numerates the lines of `content` and return a String with the `content` numerate
       /// # Example
       /// ```rust
       /// mod main_code;
       /// fn main (){
       /// use crate::main_code::utilities::general;
       /// 
       /// let mut instance = general::NumLines::new("example.txt", " - ");
       /// let numerate = instance.numerate_lines();
       /// //Upload content just as here
       /// instance.set_content(&numerate);
       /// }
       /// ```
       /// # Return
       /// String with the `content` str numerated
       pub fn numerate_lines(&self) -> String{
        println!("NUMERATING LINES FROM CONTENT: {}", self.content);
        let mut new_content = String::new();

        {
            let mut count = 1;
            for line in self.content.lines(){
              if self.delimiter.is_empty(){
              new_content.push_str(&format!("{} {}\n", count, line)); 
              }
              else { 
                new_content.push_str(&format!("{}{}{}\n", count, self.delimiter, line));
              }
               
               count += 1;
            }
            
        }
        println!("LINES NUMERATED FROM CONTENT");
        return new_content;
    }
//---------------------------------------------------------------------
        /// # `remove_num_lines`
        /// Removes line numbers from `content`. Recomended use this just if you use before the function `numerate_lines` and upload `content`.
        /// # Example
        /// ```rust
        /// mod main_code;
        /// fn main (){
        /// use crate::main_code::utilities::general;
        /// let instance = general::NumLines::new("example.txt", " - ");
        /// let mut rmv_num = instance.remove_num_lines();
        /// //Upload content just as here
        /// instance.set_content(&rmv_num);
        /// }   
        /// ```
        /// # Return
        /// String with the `content` cleaned of numeber of lines
        pub fn remove_num_lines(&self) -> String{
            println!("REMOVING LINE NUMBERS FROM CONTENT: {}", self.content);
            let mut new_content = String::new();
            {
                for line in self.content.lines() {
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
            println!("LINE NUMBERS REMOVED FROM CONTENT");
            return new_content;
        }
//---------------------------------------------------------------------
        /// # `skip_num_line`
        /// Skips the line number in str. Use this just if you use before the function `numerate_lines`.
        /// # Arguments
        /// * `line: &str` - The line from which the line number will be skipped.
        /// # Example
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
        /// # Return
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
        /// # `get_num_line`
        /// Gets the current line number from a str.
        /// # Arguments
        /// * `line: &str` - The line from which the current line number will be extracted.
        /// 
        /// # Example
        /// ```rust
        /// mod main_code;
        ///fn main (){
        /// use crate::main_code::utilities::fn_num_lines;
        /// let input_file = general::NumLines::new("example.txt", " - ");
        /// let current_line = input_file.get_num_line("1 - This is a test line.");
        /// println!("Current line number: {}", current_line);
        /// }
        /// ```
        /// # Return
        /// Returns the current line number as a `i32`.
        /// If the delimiter is not found, it returns an empty -1
        /// # Errors
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
       /// # `get_input_file`
       /// Gets the input file path.
       /// # Return
       /// Returns the input file path as a `String`.
       pub fn get_input_file(&self) -> String{
            self.content.to_string()
        }
//---------------------------------------------------------------------
       /// # `get_delimiter`
       /// Gets the delimiter used for line numbering.
       /// # Return
       /// Returns the delimiter as a `String`.
       pub fn get_delimiter(&self) -> String{
            self.delimiter.to_string()
        }
//---------------------------------------------------------------------
       /// # `set_content`
       /// Sets the input file path.
       /// # Arguments
       /// * `input_file: &'a str` - The new input file path to be set.
       /// # Example
       /// ```rust
       /// mod main_code;
       /// fn main (){
       /// use crate::main_code::utilities::general;
       /// let mut input_file = general::NumLines::new("example.txt", " - ");
       /// input_file.set_content("new_example.txt");
       /// }
       /// ```
       pub fn set_content(&mut self, new_value:&'a str){
            self.content = new_value;
        }
//---------------------------------------------------------------------
       /// # `set_delimiter`
       /// Sets the delimiter
       /// ## Arguments 
       /// * `delimiter: &'a str` - The new delimiter to be set.
       /// # Example  
       /// ```rust
       /// mod main_code;
       /// fn main (){
       /// use crate::main_code::utilities::general;
       /// let mut input_file = general::NumLines::new("example.txt", " - ");
       /// input_file.set_delimiter(" | ");
       /// }
       /// ```
       pub fn set_delimiter(&mut self, new_value:&'a str){
            self.delimiter = new_value;
       }
       
    }
//------------------------------------------------------------------------------------------
    /// # `all_appears_index`
    /// Finds all occurrences of a substring in a string and returns their indexes.
    /// # Arguments
    /// * `input_str: &str` - The string in which to search for the substring.
    /// * `search_str: &str` - The substring to search for.
    /// # Example
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
    /// # Return
    /// Returns a vector of `usize` containing the indexes of all occurrences of the substring in the string.
    /// If the substring or the string is empty, it returns an empty vector.
    /// # Errors
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
//------------------------------------------------------------------------------------------
  /// # `sub_vec`
  /// Returns a sub-vector from a given vector starting at a specified index and taking a specified number of elements.
  /// # Arguments
  /// * `vec: &Vec<T>` - The vector from which to extract the sub-vector.
  /// * `num_elements: usize` - The number of elements to take from the vector.
  /// * `start_index: usize` - The index at which to start taking elements from the vector.
  /// # Example
  /// ```rust
  /// mod main_code;
  /// fn main (){
  /// use crate::main_code::utilities::general;
  /// let vec = vec![1, 2, 3, 4, 5];
  /// let sub_vec = general::sub_vec(&vec, 3, 1);
  /// println!("Sub-vector: {:?}", sub_vec);  
  /// }
  /// ```
  /// The result is a sub-vector containing the specified number of elements starting from the specified index.
  /// # Return
  /// Returns a new vector containing the specified number of elements starting from the specified index.
  /// # Errors
  /// If the vector is empty or the number of elements to take is zero, the function will panic with an error message.
  pub fn sub_vec<T: Clone>(vec: &Vec<T>, num_elements: usize, start_index: usize) -> Vec<T>{
    if vec.is_empty(){
      panic!("Error: The vector is empty.");
    }
    if num_elements == 0{
      panic!("Error: The number of elements to take cannot be zero.");
    }
    let mut start_i = start_index;
    if start_index >= vec.len(){
      start_i = 0;
    }
    if start_i + num_elements-1 >= vec.len(){
     let sub_vec = vec[start_i..].to_vec();
     return sub_vec;
    }
    let sub_vec = vec[start_i..start_i+num_elements].to_vec();
    return sub_vec;
  }
  
}
//------------------------------------------------------------------
/// # Mod `remove_comments` from `utilities.rs`
/// This module provides functions to remove comments from files.   
pub mod remove_comments{
  #![allow(unused)]
    use std::fs::{self, remove_dir_all};
    use std::io::Write;

    /// # `simple_comments`
    /// Removes simple comments from a string
    /// # Arguments
    /// * `content: &str` - The string which simple comments will be removed.
    /// * `delimiter: &str` - The delimiter used to identify simple comments.
    /// * `ignore_content_between: (&Vec<char>, &Vec<&str>)` - A tuple containing two vectors:
    ///   - A vector of characters that should be ignored the content between this when removing comments.
    ///   - A vector of strings that should be ignored the content between this when removing comments.
    /// * `manage_close: bool` - Ensure the close of the ignore_content_between tuple
    /// # Return
    /// Returns an `Option<String>`:
    /// * `Some(String)` - If the simple comments were successfully removed, returns `Some(new_content)`.
    /// * `None` - If there is an error, returns `None` with an error message.
    /// # Example
    /// ```rust 

    /// mod main_code;
    /// fn main (){
    /// use std::fs::{self, remove_dir_all};
    /// use crate::main_code::utilities::remove_comments;
    /// let input_file = "example.txt";
    /// let content = fs::read_to_string(input_file).expect(&format!("Failed to read the file '{}'", input_file));
    /// let vec_char:Vec<char> = Vec::new();
    /// let vec_str:Vec<String> = Vec::new();
    /// let tuple = (vec_char, vec_str):
    /// let new_content = remove_comments::remove_simple_comments(content, "//", tuple, false);
    /// }
    /// ```
    /// # Errors
    /// If content or delimiter is empty go to panic
    /// # Note
    /// The function will remove everything after the first occurrence of the comment delimiter in each line.
    
     pub fn simple_comments(content: &str, delimiter: &str, ignore_content_between: (&Vec<char>, &Vec<&str>), manage_close: bool)-> Option<String>{
       use crate::main_code::utilities::general;
        println!("REMOVING SIMPLE COMMENTS FROM CONTENT: {}", content);
        if delimiter.is_empty(){
            panic!("Error: The delimiter cannot be an empty string.");
        }
        if content.is_empty(){
          panic!("Error: The content cannot be an empty string.");
        }
        let mut i: usize = ignore_content_between.0.len()/2;
        if !(ignore_content_between.0.is_empty() && ignore_content_between.1.is_empty()){
       if !ignore_content_between.0.is_empty(){
        for ch in ignore_content_between.0{
          if delimiter.contains(*ch){
            println!("Error: The delimiter '{}' cannot be in the ignore characters vector '{:?}'", delimiter, ignore_content_between.0);
            return None;
            }
          }
          //Chekc if the vector ignore_content_between.0 has an even number of elements
          //Becuase is a pair start-end, so, all the characters must be in pairs, like this: ['{', '}'], ['(', ')'], ['[', ']']
          let i = ignore_content_between.0.len();
         if i % 2 != 0{
            println!("Error: The ignore characters vector '{:?}' must have an even number of elements", ignore_content_between.0);
            return None;
         }
        }
        if !ignore_content_between.1.is_empty(){
        for ch in ignore_content_between.1{
          if delimiter.contains(*ch){
            println!("Error: The delimiter '{}' cannot be in the ignore strings vector '{:?}'", delimiter, ignore_content_between.1);
            return None;
          }
         }
         // Chekc if the vector ignore_content_between.1 has an even number of elements
        //Becuase is a pair start-end, so, all the strings must be in pairs, like this: ["{", "}"], ["(", ")"], ["[", "]"]
          let i = ignore_content_between.1.len();
          if i % 2 != 0{
            println!("Error: The ignore strings vector '{:?}' must have an even number of elements", ignore_content_between.1);
            return None;
          }
        }
        if !ignore_content_between.0.is_empty() && !ignore_content_between.1.is_empty(){
        for ch in ignore_content_between.0{
          if ignore_content_between.1.contains(&&(*ch.to_string())){
            println!("Error: The ignore characters vector '{:?}' cannot contain the same characters as the ignore strings vector '{:?}'", ignore_content_between.0, ignore_content_between.1);
            return None;
          }
        }
       }
      }
      let mut new_content = String::new();
      let mut counter = 0;
      let mut line_start = String::new();
      let mut num_line = 0;
      let mut in_ignore = false; // flag to indicate if we are in the ignore content
      let mut delimiter_ignore = String::new();
      let mut ignore_delimiters = false;
      if !ignore_content_between.0.is_empty() || !ignore_content_between.1.is_empty(){ignore_delimiters = true;}

      {
        
        let mut removed = 0;
        let mut contains = false;
        for line in content.lines() {    
          contains = false;    
          counter += 1;
          removed = 0;
          let mut copy = line.to_string();
         if ignore_delimiters{ 
          if in_ignore{
            if let Some(end) = copy.find(&delimiter_ignore){
              in_ignore = false;
              copy = copy.replacen(&delimiter_ignore, "",1);
              removed += delimiter_ignore.len();
            }
          }
          if !in_ignore{
            let mut j = 0;
            let mut some_start_ignore:Vec<String> = Vec::new();
            if !ignore_content_between.0.is_empty(){
             while j <= ignore_content_between.0.len()-1{
              let mut sub_vec = general::sub_vec(&ignore_content_between.0, 2, j);
              some_start_ignore.push(sub_vec[0].to_string());
              sub_vec.clear();
              j+=2;
              }
             }
             j= 0;
             if !ignore_content_between.1.is_empty(){
             while j <= ignore_content_between.1.len()-1{
              let mut sub_vec = general::sub_vec(&ignore_content_between.1, 2, j);
              some_start_ignore.push(sub_vec[0].to_string());
              sub_vec.clear();
              j+=2;
               } 
             }
            if !some_start_ignore.is_empty(){
              for element in some_start_ignore{
              if copy.contains(&element){
                contains = true;
                break;
              }
             }
            }
          }
          if copy.contains(delimiter) && !in_ignore && contains{
            if !ignore_content_between.0.is_empty() || !ignore_content_between.1.is_empty(){
            let result = content_between(ignore_content_between.0, ignore_content_between.1, delimiter, &copy);
            delimiter_ignore = result.0;
            in_ignore = result.1;
            new_content.push_str(&line[..result.2.len()+removed]);
              new_content.push('\n');
            if in_ignore{
              num_line = counter;
              line_start = copy;
            }
          }else{
            if let Some(del_pos) = copy.find(delimiter){
               new_content.push_str(&copy[..del_pos]);
               new_content.push('\n');
            }
          }
          }
          else{
            if line.contains(delimiter) && !in_ignore{
              if let Some(delimiter) = copy.find(delimiter){
               new_content.push_str(&line[..delimiter]);
               new_content.push('\n');
             }
            }
            else{
            new_content.push_str(&line);
            new_content.push('\n');
             }
            }
           }
           else{
             if let Some(delimiter) = copy.find(delimiter){
               new_content.push_str(&line[..delimiter]);
               new_content.push('\n');
             }else{
              new_content.push_str(&line);
               new_content.push('\n');
             }
           }
          }  
        }
        // if some ignore are open after process all the file, print an error
        if in_ignore && manage_close && ignore_delimiters{
           println!("Error in the line: '{}': '{}'. missing close delimiter: {}", num_line, line_start, delimiter_ignore);
           return None;
        }

        println!("SIMPLE COMMENTS REMOVED FROM CONTENT");
        return Some(new_content);
    }
//------------------------------------------------------------------
    /// # `content_between`
    /// process the line with comment delimiters, management the secuence
    /// # Arguments
    /// * `delimiters_array_char: &Vec<char>` - Array of chars to indicate pairs that indicate a start and end delimiter of a conent must be are ignored
    /// * `delimiters_array_str: &Vec<&str>` - Array of Strings to indicate pairs that indicate a start and end delimiter of a conent must be are ignored
    /// * `delimiter:&str` - comment delimiter
    /// * `line: &str` - line to process
    /// # Return
    /// A tuple with 3 elements 
    /// * Elements:
    /// - `0:String`. Is a void string if the start delimiter ignore are correctly closely in the same line, else is the start delimiter ignore not closed
    /// - `1:bool`. Is `true` if the some ignore pair are be open but not closely, else its `false`
    /// - `2:String`. Is the string result to the process
    /// # Note 
    /// This is use in the function [`simple_comments`] 
    fn content_between(delimiters_array_char: &Vec<char>, delimiters_array_str: &Vec<&str>, delimiter: &str, line: &str) -> (String, bool, String){
       let mut new_line2 = String::new();
       let mut in_ignore = false;
       let mut result:(String, bool, String);
       // If the line contains a comment delimiter start to check this
            let pos = line.find(delimiter).unwrap(); //position of the comment delimiter
             new_line2 = line[..pos].to_string(); //content before the comment delimiter
            let mut delimiters_array:Vec<String> = Vec::new();
            if !delimiters_array_char.is_empty(){
            for element in delimiters_array_char{
              delimiters_array.push(element.to_string());
             }
            }
            if !delimiters_array_str.is_empty(){
              for element in delimiters_array_str{
              delimiters_array.push(element.to_string());
             }
            }
            // process the char array.
            let result1 = process(in_ignore, &delimiters_array, line, pos, delimiter);
            if !result1.1{
                new_line2 = result1.2;
                  result = ("".to_string(), false, new_line2.to_string());
                 return result;
             }
            else{
              result = (result1.0, result1.1, result1.2);
            return result;
             }
          //if the line not contains some comment delimiter return the line
          result = ("".to_string(), false, line.to_string());
          return result;
      
    }
//---------------------------------------------------------
    /// # `process`
    /// process a string for identify the content to ignore and identify the comments in this string
    /// # Arguments
    /// * `in_ignore:bool` - Flag to indicate if are be in ignore content
    /// * `delimiters_array:&Vec<String>` - Array that contains the delimiters to indicate when the content are must be ignored
    /// * `line:&str` - Line to process
    /// * `pos:usize` - Position of the comment delimiter in the line
    /// * `delimiter:&str` - Comment delimiter 
    /// # Return 
    /// A tuple with 3 elements 
    /// * Elements:
    /// - `0:String`. Is a void string if the start delimiter ignore are correctly closely in the same line, else is the end delimiter expected for the close
    /// - `1:bool`. Is `true` if the some ignore pair are be open but not closely, else its `false`
    /// - `2:String`. Is the string result to the process
    /// # Note 
    /// This is use in the function [`content_between`] 

    fn process(mut in_ignore:bool, delimiters_array:&Vec<String>, line:&str, mut pos:usize, delimiter:&str)->(String, bool, String){
      use crate::main_code::utilities::general;
      if !in_ignore{
      let mut copy = line.to_string(); // create a mutable copy of the input line
      let mut j = 0; // Use like a global counter 
      let mut start_ignore_index:Vec<usize> = Vec::new();//Array of indexes by start ignore delimiters found
      let mut end_ignore_index:Vec<usize> = Vec::new();// Array of indexes by end ignore delimiters found
      let mut content_out_of_comment = line[..pos].to_string(); // String to contain the content before a delimiter comment
      let mut some_start_ignore:Vec<String> = Vec::new(); // Array of strings, for store the start ignore delimiter
      let mut removed = 0; // global variable for store some temporary value 
      let mut without_end: Vec<usize> = Vec::new(); // Array for store the indexes of the ignore delimiters start without end delimiter
      let mut expected: Vec<String> = Vec::new(); // Array for store the end delimiter for them indexes without end delimiter
      
        j= 0;//reset j
        // Fill some_start_ignore
        while j<= delimiters_array.len()-1{
          let mut sub_vec = general::sub_vec(delimiters_array, 2, j);
        some_start_ignore.push(sub_vec[0].to_string());
        sub_vec.clear();
        j+=2;
        }
        let mut ignore_order: Vec<usize> = Vec::new(); //Array to store the order of ignore start delimiters like has a end delimiter ignore found
         j= 0;//reset j
         //Create an scope for local processing, phase of "IDENTIFY THE ORDER OF THE DELIMITERS AND FILTER THIS"
         {
          // get the order of the delimiters in the string
          //iterate in the start delimiters for search this
           for (n, i) in some_start_ignore.iter().enumerate(){
            //iterate in all appears of this delimiter
            while copy.contains(i){
              let mut copy2 = copy.to_string();
              let mut i2 = 0;
              // Look the pos of the delimier
              if let Some(pos_start) = copy2.find(i){
                let mut new_copy = String::new();
                let mut before_f = pos_start;
                removed = i.len();
                for (n, c) in copy2.chars().enumerate(){
                  if i2<removed && n==before_f{
                    new_copy.push(' ');
                    i2+=1;
                    before_f+=1;
                  }
                  else{
                    new_copy.push(c);
                  }
                }
                copy2 = new_copy.to_string();
                i2 = 0;
                // Search the pos of the end delimiter if have
                 if let Some(pos_end) = copy2.find(&delimiters_array[n+1]){
                  start_ignore_index.push(pos_start);
                  end_ignore_index.push(pos_end);
                  removed = copy[pos_start..pos_end+delimiters_array[n+1].len()].len();
                  let mut before = pos_start;
                  let mut new_string = String::new();
                  //replace this range  with spaces (' ') for avoid move indexes and edit the length of the string
                    for (n, c) in copy2.chars().enumerate(){
                      if i2<removed  && n== before{
                        new_string.push(' ');  
                        before += 1;
                      i2+=1;
                      }
                      
                      else{new_string.push(c)}
                    }
                   
                  copy = new_string.to_string(); //upload copy
                 }
                 // if not have his end delimiter
                 else{
                 without_end.push(pos_start);
                 expected.push(delimiters_array[n+1].clone()); 
                 copy = copy2.to_string();
                 }
               }
             }
           }
           //sort the indexes than less to greater 
           start_ignore_index.sort();
           end_ignore_index.sort();
         }

        let mut comment_appears:Vec<usize> = Vec::new();    
        // Scope for process the indexes and comments "FILTER COMMENTS AND INDEXES" 
        {   
        let mut s = 0;
        let mut comment_appears_first = general::all_appears_index(&copy, delimiter);
        let mut copy_start = start_ignore_index.to_vec();
        let mut copy_end = end_ignore_index.to_vec();
        let mut index_remove_comment: Vec<usize> = Vec::new();
        //remove comments into ignore content
        while comment_appears_first.len() > 0{
          if s>comment_appears_first.len()-1{break;}
          if copy_start.len() <= 0 || copy_end.len() <= 0{
              break;
            }
          let mut index_removed = 0;
          //if the comment is into an ignore content
           for (l, n) in comment_appears_first.iter().enumerate(){
            if *n  > copy_start[0] && *n  < copy_end[0]{
              index_remove_comment.push(l);
            }
            else{
              comment_appears.push(*n);
              }
            }
            s += 1;
            //remove indexes processed
           if !index_remove_comment.is_empty(){
            for n in &index_remove_comment{
              comment_appears_first.remove((*n)-index_removed);
              index_removed += 1;
            }
           }
            if copy_start.len()-1 <= 0 || copy_end.len()-1 <= 0{
              break;
            }
            copy_start.remove(0);
            copy_end.remove(0);
         }
        }
        // If the line contains all his comments delimiters into ignore content
       if comment_appears.is_empty(){
        copy = line.to_string();
        let mut copy2= copy.to_string();
           let mut contains = false;
            for n in &some_start_ignore{
                  if copy2.contains(n){
                  contains = true;
                  break;
               }
            }
            //process the rest of the ignore delimiters pairs for identify if some are not closely
            let mut sub_vec2:Vec<String> = Vec::new();

            if contains{
              let mut sub_vec_start = 0;
              //search all remaining ignore delimiter pairs
              while sub_vec_start <= delimiters_array.len()-1 && !in_ignore{
                  copy2 = copy.to_string();
                  sub_vec2 = general::sub_vec(&delimiters_array, 2, sub_vec_start);
                  //search the start ignore delimiter and remove themm
                  if let Some(ignore_start) = copy.find(&sub_vec2[0]){
                      in_ignore=true;
                      copy2 = copy2.replacen(&sub_vec2[0], "", 1);
                      //search the end ignore delimiter and remove the content and the delimiters for the line copy
                      if let Some(ignore_end) = copy2.find(&sub_vec2[1]){
                      sub_vec_start = 0;
                      in_ignore = false;
                      copy.replace_range(ignore_start..ignore_end+sub_vec2[1].len()+sub_vec2[0].len(), "");
                      }
                    }
                    else{
                      sub_vec_start += 2;
                    }
                  }
                let result = (sub_vec2[1].to_string(), in_ignore, line.to_string());
                return result;
              }
              else{
                let result = ("".to_string(), in_ignore, line.to_string());
                return result;
              }
          }
          //if the line has comments
        else{
          let mut comment_appears2 = comment_appears.to_vec();
          // Check if the first start comment are into ignore content
          for (i, n) in without_end.iter().enumerate(){
            if *n < comment_appears2[0]{
              let result = (expected[i].to_string(), true, line.to_string()); 
              return result;
            }
          }
          // Check if start delimiter ignore are into a comment or after of them
          while !comment_appears2.is_empty(){
            let mut index_rmv = 0;
            let mut remove:Vec<usize> = Vec::new();
            for (s, r) in start_ignore_index.iter().enumerate(){
             if comment_appears2[0] < *r{
              remove.push(s);
             }
            }
            comment_appears2.remove(0);
            for n in remove{   
              start_ignore_index.remove(n-index_rmv);
              end_ignore_index.remove(n-index_rmv);
               index_rmv+=1;
            }
          }
        }
 
            //upload the index of the pos
        if !comment_appears.is_empty(){content_out_of_comment = line[..comment_appears[0]].to_string();}

        let result = ("".to_string(), false, content_out_of_comment.to_string());
        return result;
      }else{
        let result = ("".to_string(), true, line.to_string());
        return result;
      }
        
   }
//------------------------------------------------------------------
    /// # `ModeBlock`
    /// An enum to specify the mode of block comment removal.
    /// # Variants
    /// * `Nested` - Removes nested block comments. This mode will handle block comments that may contain other block comments within them, and ensure that nested comments are properly closed.
    /// * `Single` - Removes single block comments. This mode will handle block comments that are not nested and will remove them in a single pass, without considering nested structures.
     pub enum ModeBlock{
        Nested,
        Single
     }
    /// # `block_comments`
    /// Removes block comments from a string. 
    /// * This function is an API for the functions [`single_mode`] and [`nested_mode`].
    /// # Arguments
    /// * `content: &str` - The string from which block comments will be removed.
    /// * `start_delimiter: &str` - The starting delimiter of the block comment.
    /// * `end_delimiter: &str` - The ending delimiter of the block comment.
    /// * `ignore_content_between: (&Vec<char>, &Vec<&str>)` - A tuple containing two vectors:
    /// * `mode: ModeBlock` - The mode of block comment removal, either [`ModeBlock::Nested`] or [`ModeBlock::Single`]
    /// # Example
    /// ```rust
    /// mod main_code;
    /// fn main (){
    /// use crate::main_code::utilities::remove_comments;
    /// let content = "example.txt/*fadfjs*/";
    /// let vec_char:Vec<char> = Vec::new();
    /// let vec_str:Vec<String> = Vec::new();
    /// let tuple = (vec_char, vec_str):
    /// let result = remove_comments::remove_block_comments(content, "/*", "*/", tuple, ModeBlock::Single);
    /// }
    /// ```
    /// The result is a file with block comments removed.
    /// # Errors
    /// If the file cannot be read or written, the function will panic with an error message
    /// If there is a block comment without end delimiter, the function will return -1 with an error message.
    /// # Return
    /// * `None` - If there is a block comment without an end delimiter.
    /// * `Some(String)` - If the block comments were successfully removed.
    
    pub fn block_comments(content: &str, start_delimiter: &str, end_delimiter: &str, ignore_content_between: (&Vec<char>, &Vec<&str>), mode: ModeBlock) -> Option<String>{
      if content.is_empty(){
        panic!("Error: the argument 'conten't is empty");
      }
      if start_delimiter.is_empty() || end_delimiter.is_empty(){
        panic!("Error: start delimiter or end delimiter is empty");
      }

      if !(ignore_content_between.0.is_empty() && ignore_content_between.1.is_empty()){
       if !ignore_content_between.0.is_empty(){
        for ch in ignore_content_between.0{
          if start_delimiter.contains(*ch)||end_delimiter.contains(*ch){
            println!("Error: The start delimiter '{}' or end delimiter '{}' cannot be in the ignore characters vector '{:?}'", start_delimiter, end_delimiter, ignore_content_between.0);
            return None;
            }
          }
          //Chekc if the vector ignore_content_between.0 has an even number of elements
          //Becuase is a pair start-end, so, all the characters must be in pairs, like this: ['{', '}'], ['(', ')'], ['[', ']']
          let i = ignore_content_between.0.len();
         if i % 2 != 0{
            println!("Error: The ignore characters vector '{:?}' must have an even number of elements", ignore_content_between.0);
            return None;
         }
        }
        if !ignore_content_between.1.is_empty(){
        for str in ignore_content_between.1{ 
          if start_delimiter.contains(*str) || end_delimiter.contains(*str){
            println!("Error: The start delimiter '{}' or end delimiter '{}' cannot be in the ignore strings vector '{:?}'", start_delimiter,end_delimiter, ignore_content_between.1);
            return None;
           }
         }
         // Chekc if the vector ignore_content_between.1 has an even number of elements
        //Becuase is a pair start-end, so, all the strings must be in pairs, like this: ["{", "}"], ["(", ")"], ["[", "]"]
          let i = ignore_content_between.1.len();
          if i % 2 != 0{
            println!("Error: The ignore strings vector '{:?}' must have an even number of elements", ignore_content_between.1);
            return None;
          }
        }
        if !ignore_content_between.0.is_empty() && !ignore_content_between.1.is_empty(){
        for ch in ignore_content_between.0{
          if ignore_content_between.1.contains(&&(*ch.to_string())){
            println!("Error: The ignore characters vector '{:?}' cannot contain the same characters as the ignore strings vector '{:?}'", ignore_content_between.0, ignore_content_between.1);
            return None;
          }
        }
       }
      }
      println!("REMOVING BLOCK COMMENTS FROM CONTENT: {}", content);
      let mut new_content = String::new();
      match mode{
      ModeBlock::Single =>{
        match single_mode(&content, start_delimiter, end_delimiter){
            Ok(content2) =>  new_content.push_str(&content2) ,
            Err(_) => return None
        }
       }
       ModeBlock::Nested =>{
        match nested_mode(&content, start_delimiter, end_delimiter){
          Ok(content2) => new_content.push_str(&content2),
          Err(_) => return None
        }
       }
      }
      println!("BLOCK COMMENTS REMOVED FROM CONTENT");
      return Some(new_content);
    }
//------------------------------------------------------------------------------------------
    /// # `single_mode`
    /// Removes block comments in single mode from a line.
    /// # Arguments
    /// * `content: &Vec<&str>` - A vector of lines from content from which block comments will be removed.
    /// * `delimiter_end: &str` - The ending delimiter of the block comment.
    /// * `delimiter_start: &str` - The starting delimiter of the block comment.
    /// * **NOTE:** This is use in his API [`block_comments`] fuction.
    /// # Return
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
         let mut in_ignore = false; //flag to indicate if we are in ignore_conntent
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
                if !block_open {
                  
                  new_content.push_str(&line_copy[..start_pos]); block_open = true;
                }
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
    /// # `nested_mode`
    /// Removes block comments in nested mode from a line.
    /// # Arguments
    /// * `content: &str` - A string containing the content from which block comments will be removed.
    /// * `delimiter_start: &str` - The starting delimiter of the block comment.
    /// * `delimiter_end: &str` - The ending delimiter of the block comment.
    /// * **NOTE:** This is use in his API [`block_comments`] fuction.
    /// # Return
    /// Returns a `Result<String, i32>`:
    /// * `Ok(String)` - If the block comments were successfully removed, returns a `String` with the content without block comments.
    /// * `Err(i32)` - If there is an error, returns an `i32` error code:
    ///   - `-1` - If the start and end delimiters are the same or content vector is empty.
    ///   - `2` - If the block comment are not closed and arrive to the
    /// # Use
    /// This function is used to handle nested block comments, where block comments can contain other block comments within them. It ensures that nested comments are properly closed and that the content between comments is preserved.
    /// ## Example
    /// ```rust
    /// mod main_code;
    /// fn main (){
    /// use crate::main_code::utilities::remove_comments;
    /// use std::fs;
    /// use  std::io::Write;
    /// let mut file = fs::File::create("example.txt").expect("Failed to create the file");
    /// let content = "This is a test /* This is a nested*/ between /* comment /* block comment */*/line/*/*.";
    /// file.write_all(content.as_bytes()).expect("Failed to write to the file");
    /// let result = remove_comments::block_comments("example.txt", "/*", "*/", remove_comments::ModeBlock::Nested);
    /// match result {
    ///    0 => println!("Content without block comments: {}", new_content),
    ///   -1 => println!("Error removing block comments: {}", e),
    ///    _ => println!("Unexpected error code: {}", e),
    /// }
    /// }
    /// ```
    /// The result is a string with the content without block comments, handling nested comments correctly.
    /// `resutl: Ok("This is a test  between line/*/*.\n");`
    /// This occurs because inside the function somethings like this:
    /// * `/*/`
    /// * `*/*`
    /// 
    /// are interpreted like a end delimiter, becuause inside the function we resolve this conflicts with this:
    /// ```rust ignore
          /// // --<snip>--
          /// //  let mut indexes_to_delete:Vec<usize> = Vec::new();
          /// // We need to check if the indexes_end are in the indexes vector
          /// // If it does because we need to handle conflicts between, end delimiter and start delimter
          /// // its occurs when the end delimiter and start delimiter superpose, like this '/*/' or this '*/*', 
          /// // in this example the start delimiter are in the first and second character ('/*'), but the end delimiter
          /// // are in the second and third character ('*/'), so, for avoid this problems the code priorize the end delimiter
          /// // and remove the start delimiter from the vector indexes, therefore, this '/*/' are interpreting like a end delimiter
          /// // and the case when the end delimiter has the same index than the start delimiter can't appear.
          /// // The index to remove in the indexes vector is store into the vector indexes_to_delete
          /// // And the line_indexes_start vector ensure this made until on indexes at the same line
          ///for(i, value) in indexes.iter().enumerate(){
           /// if line_indexes_start[i] == line_indexes_end{
           /// if indexes_end.contains(&(*value+delimiter_start.len()-1))|| indexes_end.contains(&(*value-(delimiter_end.len()-1))){
              
           ///     indexes_to_delete.push(i);
          ///    }
          ///  }

         /// }
         /// //This is use for remove the indexes that are in the indexes_to_delete vector
         /// // We need to have this variable because in each remove the index decrement, so we need consider this decrement
         /// let mut decr_index = 0;
         /// // Here remove them index from the vector indexes
         /// for i in indexes_to_delete.iter(){
         ///   indexes.remove(*i-decr_index);
         ///   line_indexes_start.remove(*i-decr_index);
         ///   decr_index += 1;
         /// }
         /// // --<snip>--
        
    /// ```
    /// And this occurs with any end delimiter and start delimiter
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
        let mut line_indexes_end: usize = 0;
        let mut line_indexes_start: Vec<usize> = Vec::new();
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
          line_indexes_end = counter;
          } else{ indexes_end.clear();}
          // If the line contains the start delimiter, find all occurrences of it
          // and store their indexes in the indexes vector
           if line_copy.contains(delimiter_start){
             start = line_copy.find(delimiter_start).unwrap();
            line_num = counter;
            line_content = line.to_string();
            // If a block comment is not already open, push the content before the start delimiter to the new content
            if !in_block_comment{
              new_content.push_str(&line_copy[..start]);
              let index_end = general::all_appears_index(&line_copy[..start], delimiter_end);
              let mut i = 0;
              // If the end delimiter is found before the start delimiter, remove the end delimiters from the indexes_end vector
              while i < index_end.len(){
                 indexes_end.remove(i);
                 i += 1;
              }
            }
            let indexes_start_in_line = general::all_appears_index(&line_copy, delimiter_start);
            for i in indexes_start_in_line.iter(){
              // Store the indexes of the start delimiters in the indexes vector
              indexes.push(*i);
              line_indexes_start.push(counter);
            }
          }
         } 
         else{
          processed = false;
         }
         // Next, we check if the line is not empty and if it contains the start delimiter
         // If it does because we need to handle the block comment
        if indexes.len() > 0 && indexes_end.len() > 0{
          let mut indexes_to_delete:Vec<usize> = Vec::new();
          // We need to check if the indexes_end are in the indexes vector
          // If it does because we need to handle conflicts between, end delimiter and start delimter
          // its occurs when the end delimiter and start delimiter superpose, like this '/*/' or this '*/*', 
          //in this example the start delimiter are in the first and second character ('/*'), but the end delimiter
          // are in the second and third character ('*/'), so, for avoid this problems the code priorize the end delimiter
          // and remove the start delimiter from the vector indexes, therefore, this '/*/' are interpreting like a end delimiter
          // and the case when the end delimiter has the same index than the start delimiter can't appear.
          // The index to remove in the indexes vector is store into the vector indexes_to_delete
          // And the line_indexes_start vector ensure this made until on indexes at the same line
          for(i, value) in indexes.iter().enumerate(){
            if line_indexes_start[i] == line_indexes_end{
            if indexes_end.contains(&(*value+delimiter_start.len()-1))|| indexes_end.contains(&(*value-(delimiter_end.len()-1))){
              
                indexes_to_delete.push(i);
              }
            }

          }
          //This is use for remove the indexes that are in the indexes_to_delete vector
          // We need to have this variable because in each remove the index decrement, so we need consider this decrement
          let mut decr_index = 0;
          // Here remove them index from the vector indexes
          for i in indexes_to_delete.iter(){
            indexes.remove(*i-decr_index);
            line_indexes_start.remove(*i-decr_index);
            decr_index += 1;
          }
          let i = 0;// We need to use this index for the while loop
         block_comment_level = indexes.len();// This is the number of block comments intialize in the line
         // If the block comment level is greater than 0, we need to handle the block
         if block_comment_level > 0 || in_block_comment{
          // We need to handle the end delimiter, because we need to remove the block comment
          // We need to check if the end delimiter is in the indexes_end vector
          while indexes_end.len() > 0 && indexes.len() > 0{

              // If the end delimiter is greater than the start delimiter, we need to handle the block comment
              if indexes.len() > i+1{
                // If the end delimiter is less than the next start delimiter, and not its a nested block comment, or we are not be in a block comment
                // We need to push the content between the end delimiter and the next start delimiter to the new content
                // And remove this level, from the vectors and block_comment_level counter
               if indexes_end[i] < indexes[i+1] && !in_block_comment && line_indexes_start[i+1] == line_indexes_end{
                 new_content.push_str(&line_copy[indexes_end[i]+delimiter_end.len()..indexes[i+1]]);
                 indexes_end.remove(i);
                 indexes.remove(i);
                 line_indexes_start.remove(i);
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
                   line_indexes_start.remove(i);
                   block_comment_level -= 1;
                 }
               }
               // If the indexes are equal 1 or i+1 but i is even 0, 
               // that means that we are in the last layer of the block comments or the first block comment
               // therefore, the end delimiter is the end of the block comment, and can copy the value after this
               else if indexes.len() == 1 && indexes_end.len() >= 1 && block_comment_level == 1{
                 new_content.push_str(&line_copy[indexes_end[i]+delimiter_end.len()..]);
                 new_content.push('\n');
                 indexes_end.remove(i);
                 indexes.remove(i);
                  line_indexes_start.remove(i);
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
        if !in_block_comment && !processed{
         new_content.push_str(&line_copy);
         new_content.push('\n');
       }
     }
        if in_block_comment || block_comment_level > 0{
          println!("Error: Block comment without end delimiter in line '{}': '{}'\n MISSING COMMENTS TO CLOSE: {}", line_num, line_content, block_comment_level);
          return Err(2);
        }
        return Ok(new_content);
    
        
  
  }

//------------------------------------------------------------------------------------------

}
