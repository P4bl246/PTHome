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
    /// let input_file = "example\n \n";
    /// general::remove_empty_lines(input_file);
    /// }
    /// ```
    /// The result is a file withouth empty lines.
    /// # Errors
    /// If the file cannot be read or written, the function will panic with an error message.
   pub fn remove_empty_lines(content: &str) -> String{
    println!("REMOVING VOID LINES");
     let mut new_content = String::new();
     {       
       for line in content.lines(){
         if line.trim().is_empty(){
             continue;
         }
         new_content.push_str(&line.to_string());
         new_content.push('\n');
       }

     }
     println!("VOID LINES REMOVED");
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
    /// let instance = general::NumLines::new("example\n   \nsdf", " - ");
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
        println!("NUMERATING LINES FROM CONTENT");
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
        /// let removed = instace.remove_num_lines();
        /// }   
        /// ```
        /// # Return
        /// String with the `content` cleaned of numeber of lines
        /// The same content if the `content` haven't any delimiter
        pub fn remove_num_lines(&self) -> String{
            println!("REMOVING LINE NUMBERS FROM CONTENT");
            let mut new_content = String::new();
            {
                for line in self.content.lines() {
                    if self.delimiter.is_empty(){
                        if let Some(pos) = line.find(' ') {
                            new_content.push_str(&line[pos + 1..]);
                    }
                    else{
                      new_content.push_str(&line);
                    }
                  }else{
                    if let Some(pos) = line.find(self.delimiter) {
                        new_content.push_str(&line[pos + self.delimiter.len()..]);
                    }
                    else{
                      new_content.push_str(&line);
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
        /// Return the same line if the delimiter is not found.
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
                        return line.to_string();
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
       /// Gets the content.
       /// # Return
       /// Returns the content as a `String`.
       pub fn get_content(&self) -> String{
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
       /// * `new_value: &'a str` - The new content to from make set.
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
//-----------------------------------------------------------------------------------------------
  /// # `sstr_of_n_str`
  /// Create a string from other string with n length
  /// # Arguments
  /// * `str_use:&str` - String to use for generate the new string
  /// * `len_new_str:usize` - Indicate the length of the new string
  /// # Return
  /// A string with n length, all his content are the `str_use`
  /// # Example 
  /// ```rust
  /// mod main_code;
  /// fn main(){
  /// use crate::main_code::utilities::general;
  /// let str = general::str_of_n_str("S", 5);
  /// println!("{}", str); //result is "SSSSS"
  /// }
  /// ``` 
  pub fn str_of_n_str(str_use: &str, len_new_str: usize) -> String{
     let mut new = String::new();
     let mut i = 0;
     while i < len_new_str{
       new.push_str(str_use);
       i+=1;
     }
     return new;
  }
//-----------------------------------------------------------------------------------------------
   /// # `replace_index`
   /// Replace a character with his index in a string
   /// # Arguments
   /// * `str_in:&str` - string to remove the character
   /// * `repalce:&str` - string for replace
   /// * `index:usize` - index to replace
   /// # Return
   /// The input string if `str_in` or `replace` is empty
   /// A String with the index replaced
   ///   /// # Example 
  /// ```rust
  /// mod main_code;
  /// fn main(){
  /// use crate::main_code::utilities::general;
  /// let main_str = "Hello. This is my show?";
  /// let str = general::replace_index(main_str.find("?").unwrap(), "!");
  /// println!("{}", str); //result is "Hello. This is my show!"
  /// }
  /// ``` 
  pub fn replace_index(str_in: &str, replace: &str, index: usize)-> String{
    if str_in.is_empty() || replace.is_empty(){
      return str_in.to_string();
    }
    let mut new_str = String::new();
    if index >= str_in.len(){
      new_str.push_str(&str_in.to_string());
      return new_str;
    }
    for (i, c) in str_in.to_string().chars().enumerate(){
      if i == index{
        new_str.push_str(&replace.to_string());
      }
      else{
        new_str.push(c);
      }
    }
    return new_str;
  }
//-------------------------------------------------------------------------------------------------
#[cfg(test)]
///# Tests
  mod tests{
      use super::*;
      #[test]
       /// # [`super::remove_empty_lines`] Test
       ///The result expected have a '\n' at the end because within the function in each iteration push the content of the line and add '\n' on the end 
        fn test_remove_empty_lines(){
          let str= &"This is my test\n            \n The test is made for ensure the function \n \n remove_empyt_lines".to_string();
          let expected ="This is my test\n The test is made for ensure the function \n remove_empyt_lines\n".to_string();
          assert_eq!(expected, super::remove_empty_lines(str));
        }

      #[test]
      /// # [`super::NumLines::numerate_lines`] Test
       fn test_numerate_lines(){
        let new_instance = super::NumLines::new("This is the content''\n to numerate'\n this is the line three'\n", "");
        assert_eq!("1 This is the content''\n2  to numerate'\n3  this is the line three'\n".to_string(), new_instance.numerate_lines());
      }

      #[test]
      /// # [`super::NumLines::remove_num_lines`] Test
       fn test_remove_num_lines(){
        let mut new_instance = super::NumLines::new("This is the content''\n to numerate'\n this is the line three'\n", "-");
        let numerated = new_instance.numerate_lines();
        new_instance.set_content(&numerated);
        let expected = "This is the content''\n to numerate'\n this is the line three'\n".to_string();
        assert_eq!(expected, new_instance.remove_num_lines());
      }
      #[test]
      /// # [`super::NumLines::remove_num_lines`] Test 2
      /// Here we test the function with a string not numerated before and without de delimiter
      fn test_2_remove_num_lines(){
        let mut new_instance = super::NumLines::new("This is the content''\n to numerate'\n this is the line three'\n", "-");
        //Not set the content
        let expected = "This is the content''\n to numerate'\n this is the line three'\n".to_string();
        assert_eq!(expected, new_instance.remove_num_lines());
      }
      #[test]
      /// # [`super::NumLines::remove_num_lines`] Test 3
      /// Here we test the function with a delimiter just like space (' '), so the function remove all after and the first delmiter appear found for each line
        fn test_3_remove_num_lines(){
        let mut new_instance = super::NumLines::new("This is the content''\n to numerate'\n this is the line three'\n", "");
        //Not set the content
        let expected = "is the content''\nto numerate'\nthis is the line three'\n".to_string();
        assert_eq!(expected, new_instance.remove_num_lines());
      }

      #[test]
      /// # [`super::NumLines::skip_num_line`] Test
       fn test_skip_num_line(){
        let mut new_instance = super::NumLines::new("This is the content''\n to numerate'\n this is the line three'\n", "");
        let numerated = new_instance.numerate_lines();
        let mut num_line_skiped = String::new();
        for line in numerated.lines(){
          num_line_skiped.push_str(&new_instance.skip_num_line(line));
          num_line_skiped.push('\n');
        }
        assert_eq!("This is the content''\n to numerate'\n this is the line three'\n".to_string(),num_line_skiped);
      }
      #[test]
      /// # [`super::NumLines::skip_num_line`] Test 2
      /// Here we test the function with a string without end delimiter use
       fn test_2_skip_num_line(){
        let mut new_instance = super::NumLines::new("This is the content''\n to numerate'\n this is the line three'\n", "-");
      
        let mut num_line_skiped = String::new();
        for line in new_instance.content.lines(){
          num_line_skiped.push_str(&new_instance.skip_num_line(line));
          num_line_skiped.push('\n');
        }
        assert_eq!("This is the content''\n to numerate'\n this is the line three'\n".to_string(),num_line_skiped);
      }

      #[test]
      /// # [`super::NumLines::get_num_line`] Test
       fn test_get_num_line(){
        let mut new_instance = super::NumLines::new("This is the content''\n to numerate'\n this is the line three'\n", "-");
         let numerated = new_instance.numerate_lines();
        let mut num_line:Vec<i32> = Vec::new();
        for line in numerated.lines(){
          num_line.push(new_instance.get_num_line(line));
        }
        assert_eq!([1, 2, 3].to_vec(),num_line);
      }
      #[test]
      /// # [`super::NumLines::get_num_line`] Test  2
      /// test the function without the content numerated before
      /// expected -1 at the end of the vector, because not found in the string a delimiter expected '-'
       fn test_2_get_num_line(){
        let mut new_instance = super::NumLines::new("1-This is the content''\n2- to numerate'\n3 this is the line three'\n", "-");
        let mut num_line:Vec<i32> = Vec::new();
        for line in new_instance.content.lines(){
          num_line.push(new_instance.get_num_line(line));
        }
        assert_eq!([1, 2, -1].to_vec(),num_line);
      }
      #[test]
      #[should_panic]
      /// # [`super::NumLines::get_num_line`] Test  3
      /// test the function without the content numerated before
      /// expected panic because can convert 'This' to i32
       fn test_3_get_num_line(){
        let mut new_instance = super::NumLines::new("This is the content''", "");
        assert_eq!(1,new_instance.get_num_line("This is the content''"));
      }

      #[test]
      /// # [`super::NumLines::get_content`] Test
     fn test_get_content(){
        let new_instance = super::NumLines::new("1-This is the content''", "");
        assert_eq!("1-This is the content''".to_string(), new_instance.get_content());
      }

      #[test]
      /// # [`super::NumLines::get_delimiter`] Test
       fn test_get_delimiter(){
        let new_instance = super::NumLines::new("1-This is the content''", "");
        assert_eq!("".to_string(), new_instance.get_delimiter());
      }
      #[test]
      /// # [`super::NumLines::set_content`] Test
       fn test_set_content(){
        let mut new_instance = super::NumLines::new("1-This is the content''", "");
        new_instance.set_content("Set To This");
        assert_eq!("Set To This".to_string(), new_instance.get_content());
      }
      
      #[test]
      /// # [`super::NumLines::set_delimiter`] Test
       fn test_set_delimiter(){
        let mut new_instance = super::NumLines::new("1-This is the content''", "");
        new_instance.set_delimiter("-");
        assert_eq!("-".to_string(), new_instance.get_delimiter());
      }

      #[test]
      /// # [`super::all_appears_index`] Test
     fn test_all_appears_index(){
        let vec:Vec<usize> = vec![5, 7, 10];
        assert_eq!(vec, super::all_appears_index("This - -  -" , "-"));
      }

      #[test]
      /// # [`super::str_of_n_str`] Test
       fn test_str_of_n_str(){
        assert_eq!("------".to_string(),super::str_of_n_str("-", 6));
      }

      #[test]
      /// # [`super::sub_vec`] Test
       fn test_sub_vec(){
        assert_eq!([6, 8].to_vec(), super::sub_vec(&[-1, 5, 6, 8].to_vec(), 3, 2));
      }
      
      #[test]
      /// # [`super::replace_index`] Test
       fn test_replace_index(){
        let str = "Edit This? 'hi'";
        let index = str.find("?").unwrap();
        assert_eq!("Edit This/Hello 'hi'".to_string(), super::replace_index(str, "/Hello", index));
      }
      #[test]
      /// # [`super::replace_index`] Test 2
      /// test when the index is greather than the str.len()-1
       fn test_2_replace_index(){
        let str = "Edit This? 'hi'";
        let index = str.find("?").unwrap();
        assert_eq!("Edit This? 'hi'".to_string(), super::replace_index(str, "/Hello", str.len()));
      }

  }

}

//------------------------------------------------------------------
/// # Mod `remove_comments` from `utilities.rs`
/// This module provides functions to remove comments from files.   
pub mod remove_comments{
  #![allow(unused)]
    use std::f32::consts::E;
    use std::fs::{self, remove_dir_all};
    use std::io::Write;

    use crate::main_code::utilities::general::str_of_n_str;

    /// # `simple_comments`
    /// Removes simple comments from a string
    /// # Arguments
    /// * `content: &str` - The string which simple comments will be removed.
    /// * `delimiter: &str` - The delimiter used to identify simple comments.
    /// * `ignore_content_between: (&Vec<char>, &Vec<&str>)` - A tuple containing two vectors:
    ///   - A vector of characters that should be ignored the content between this when removing comments.
    ///   - A vector of strings that should be ignored the content between this when removing comments.
    /// * `scape_characters:&Vec<char>` - A vector of chars for define the scape characters for ignore end delimiters  
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
    /// let scape = [].to_vec()
    /// let tuple = (&vec_char, &vec_str):
    /// let new_content = remove_comments::remove_simple_comments(content, "//", tuple, &scape, false);
    /// }
    /// ```
    /// # Errors
    /// If content or delimiter is empty go to panic
    /// # Note
    /// The function will remove everything after the first occurrence of the comment delimiter in each line.
    
     pub fn simple_comments(content: &str, delimiter: &str, ignore_content_between: (&Vec<char>, &Vec<&str>), scape_characters:&Vec<char>,manage_close: bool)-> Option<String>{
       use crate::main_code::utilities::general;
        println!("REMOVING SIMPLE COMMENTS FROM CONTENT: {}", content);
        if delimiter.is_empty() || delimiter.contains(" "){
            panic!("Error: The delimiter cannot be an empty string or contains a space (' ').");
        }
        if content.is_empty(){
          panic!("Error: The content cannot be an empty string.");
        }
        if scape_characters.len()>0{
          if scape_characters.contains(&' '){
            println!("Error: The scape characters vector '{:?}' cannot contains some space character (' ')", scape_characters);
            return None;
          }
        }
        let mut i: usize = ignore_content_between.0.len()/2;
        if !(ignore_content_between.0.is_empty() && ignore_content_between.1.is_empty()){
       if !ignore_content_between.0.is_empty(){
        for ch in ignore_content_between.0{
          if delimiter.contains(*ch){
            println!("Error: The delimiter '{}' cannot be in the ignore characters vector '{:?}'", delimiter, ignore_content_between.0);
            return None;
            }
          if *ch == ' '{
              println!("Error: The ignore delimiter '{}' cannot be a space (' ') the ignore characters vector '{:?}'", *ch, ignore_content_between.0);
               return None;
            }
            if scape_characters.len() >0{
             if scape_characters.contains(ch){
              println!("Error: The ignore delimiter '{}' cannot contains a scape character ('{:?}') the ignore characters vector '{:?}'", *ch, scape_characters, ignore_content_between.0);
               return None;
             }
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
          if ch.contains(" "){
          println!("Error: The ignore delimiter '{}' cannot contains a space (' ') the ignore characters vector '{:?}'", *ch, ignore_content_between.1);
            return None;
          }
           if scape_characters.len() >0{
            for char in ch.chars(){
             if scape_characters.contains(&char){
              println!("Error: The ignore delimiter '{}' cannot contains a scape character ('{:?}') the ignore characters vector '{:?}'", *ch, scape_characters, ignore_content_between.1);
               return None;
             }
            }
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
        
        let mut contains = false;
        for line in content.lines() {    
          contains = false;    
          counter += 1;
          let mut copy = line.to_string();
         if ignore_delimiters{ 
          if in_ignore{
            if let Some(mut end) = copy.find(&delimiter_ignore){
              let mut not_found = false;
              if end > 0{
                if scape_characters.len() > 0{
                  if scape_characters.contains(&line.to_string().chars().nth(end-1).unwrap()){
                  copy.replace_range(..end+delimiter_ignore.len(), &str_of_n_str(" ", copy[..end+delimiter_ignore.len()].len()));
                    loop {
                      if let Some(end2) = copy.find(&delimiter_ignore){
                        if scape_characters.contains(&line.to_string().chars().nth(end2-1).unwrap()){
                          copy.replace_range(..end2+delimiter_ignore.len(), &str_of_n_str(" ", copy[..end2+delimiter_ignore.len()].len()));
                        }else{
                          end = end2;
                          break;
                        }
                        
                      }else{
                        not_found = true;
                        break;
                      }
                    }
                  }
                }
              }
              if !not_found{
              in_ignore = false;
              copy.replace_range(..end+delimiter_ignore.len(), &str_of_n_str(" ", copy[..end+delimiter_ignore.len()].len()));    
              }           
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
            let result = content_between(ignore_content_between.0, ignore_content_between.1, scape_characters, delimiter, &copy);
            delimiter_ignore = result.0;
            in_ignore = result.1;
            new_content.push_str(&line[..result.2.len()]);
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
    /// * `scape_characters:&Vec<char>` - A vector of chars for define the scape characters for ignore end delimiters  
    /// * `delimiter:&str` - comment delimiter
    /// * `line: &str` - line to process
    /// # Return
    /// A tuple with 3 elements 
    /// * Elements:
    /// - `0:String`. Is a void string if the start delimiter ignore are correctly closely in the same line, else is the start delimiter ignore not closed
    /// - `1:bool`. Is `true` if the some ignore pair are be open but not closely, else its `false`
    /// - `2:String`. Is the string result to the process
    /// 
    /// * Panic with a message indicate the error
    /// # Note 
    /// This is use in the functions [`simple_comments`], [`single_mode`] and [`nested_mode`]
    pub fn content_between(delimiters_array_char: &Vec<char>, delimiters_array_str: &Vec<&str>, scape_characters:&Vec<char>, delimiter: &str, line: &str) -> (String, bool, String){

       if delimiter.contains(" "){
            panic!("Error: The delimiter cannot contains a space (' ').");
        }
        if scape_characters.len()>0{
          if scape_characters.contains(&' '){
            panic!("Error: The scape characters vector '{:?}' cannot contains some space character (' ')", scape_characters);
          }
        }
        let mut i: usize = delimiters_array_char.len()/2;
        if !(delimiters_array_char.is_empty() && delimiters_array_str.is_empty()){
       if !delimiters_array_char.is_empty(){
        for ch in delimiters_array_char{
          if delimiter.contains(*ch){
            panic!("Error: The delimiter '{}' cannot be in the ignore characters vector '{:?}'", delimiter, delimiters_array_char);
            }
          if *ch == ' '{
              panic!("Error: The ignore delimiter '{}' cannot be a space (' ') the ignore characters vector '{:?}'", *ch, delimiters_array_char);

            }
            if scape_characters.len() >0{
             if scape_characters.contains(ch){
              panic!("Error: The ignore delimiter '{}' cannot contains a scape character ('{:?}') the ignore characters vector '{:?}'", *ch, scape_characters, delimiters_array_char);
             }
           }  
          }
          //Chekc if the vector delimiters_array_char has an even number of elements
          //Becuase is a pair start-end, so, all the characters must be in pairs, like this: ['{', '}'], ['(', ')'], ['[', ']']
          let i = delimiters_array_char.len();
         if i % 2 != 0{
            panic!("Error: The ignore characters vector '{:?}' must have an even number of elements", delimiters_array_char);
         }
        }
        if !delimiters_array_str.is_empty(){
        for ch in delimiters_array_str{
          if delimiter.contains(*ch){
            panic!("Error: The delimiter '{}' cannot be in the ignore strings vector '{:?}'", delimiter, delimiters_array_str);
          }
          if ch.contains(" "){
          panic!("Error: The ignore delimiter '{}' cannot contains a space (' ') the ignore characters vector '{:?}'", *ch, delimiters_array_str);
          }
           if scape_characters.len() >0{
            for char in ch.chars(){
             if scape_characters.contains(&char){
              panic!("Error: The ignore delimiter '{}' cannot contains a scape character ('{:?}') the ignore characters vector '{:?}'", *ch, scape_characters,delimiters_array_str);
             }
            }
          }
         }
         // Chekc if the vector delimiters_array_str has an even number of elements
        //Becuase is a pair start-end, so, all the strings must be in pairs, like this: ["{", "}"], ["(", ")"], ["[", "]"]
          let i = delimiters_array_str.len();

          if i % 2 != 0{
            panic!("Error: The ignore strings vector '{:?}' must have an even number of elements", delimiters_array_str);
          }
        }
        if !delimiters_array_char.is_empty() && !delimiters_array_str.is_empty(){
        for ch in delimiters_array_char{
          if delimiters_array_str.contains(&&(*ch.to_string())){
            panic!("Error: The ignore characters vector '{:?}' cannot contain the same characters as the ignore strings vector '{:?}'", delimiters_array_char,delimiters_array_str);
          }
        }
       }
      }
       let mut new_line2 = String::new();
       let mut in_ignore = false;
       let mut result:(String, bool, String);
       // If the line contains a comment delimiter start to check this
            let pos = line.find(delimiter).unwrap_or(0); //position of the comment delimiter
             
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
            let result1 = process(in_ignore, &delimiters_array, scape_characters, line, pos, delimiter);
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
    /// * `scape_characters:&Vec<char>` - A vector of chars for define the scape characters for ignore end delimiters  
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

    fn process(mut in_ignore:bool, delimiters_array:&Vec<String>, scape_characters:&Vec<char>, line:&str, mut pos:usize, delimiter:&str)->(String, bool, String){
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
                 if let Some(mut pos_end) = copy2.find(&delimiters_array[n+1]){
                  start_ignore_index.push(pos_start);
                  if scape_characters.len() >0{
                    if pos_end>0{
                    //check if the delimiter are after a scape character 
                    if scape_characters.contains(&line.to_string().chars().nth(pos_end-1).unwrap()){
                      let mut not_found = false;
                      //remove the last value push in the vector
                      copy2.replace_range(pos_start+i.len()..pos_end+delimiters_array[n+1].len(), &general::str_of_n_str(" ", copy2[pos_start+i.len()..pos_end+delimiters_array[n+1].len()].len()));
                      loop{
                        // Search the pos of the end delimiter if have
                          if let Some(pos_end2) = copy2.find(&delimiters_array[n+1]){
                            //check if the delimiter are after a scape character 
                            if scape_characters.contains(&line.to_string().chars().nth(pos_end2-1).unwrap()){
                              copy2.replace_range(pos_start+i.len()..pos_end2+delimiters_array[n+1].len(), &general::str_of_n_str(" ", copy2[pos_start+i.len()..pos_end2+delimiters_array[n+1].len()].len()));
                            }else{
                              pos_end = pos_end2;
                              break;
                            }
                          }else{
                           not_found = true;
                           break;
                          }
                      }
                      if not_found{
                        start_ignore_index.remove(start_ignore_index.len()-1); //remove the last value push in the vector
                        //If not found end delimiter
                        without_end.push(pos_start);
                        expected.push(delimiters_array[n+1].clone()); 
                        copy = copy2.to_string();
                        continue;
                      }

                     }
                    }
                  }
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
               for i in &comment_appears_first{
                 comment_appears.push(i.clone());
                 }
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
                      copy2 = copy2.replacen(&sub_vec2[0], &general::str_of_n_str(" ", sub_vec2[0].len()), 1);
                      //search the end ignore delimiter and remove the content and the delimiters for the line copy
                      if let Some(mut ignore_end) = copy2.find(&sub_vec2[1]){
                        if scape_characters.len() >0{
                          if ignore_end>0{
                    //check if the delimiter are after a scape character 
                          if scape_characters.contains(&line.to_string().chars().nth(ignore_end-1).unwrap()){
                             let mut not_found = false;
                      //remove the last value push in the vector
                            copy2.replace_range(ignore_start+sub_vec2[0].len()..ignore_end+sub_vec2[1].len(), &general::str_of_n_str(" ", copy2[ignore_start+sub_vec2[0].len()..ignore_end+sub_vec2[1].len()].len()));
                            loop{
                        // Search the pos of the end delimiter if have
                          if let Some(pos_end2) = copy2.find(&sub_vec2[1]){
                            //check if the delimiter are after a scape character 
                            if scape_characters.contains(&line.to_string().chars().nth(pos_end2-1).unwrap()){
                            copy2.replace_range(ignore_start+sub_vec2[0].len()..pos_end2+sub_vec2[1].len(), &general::str_of_n_str(" ", copy2[ignore_start+sub_vec2[0].len()..pos_end2+sub_vec2[1].len()].len()));
                            }else{
                              ignore_end = pos_end2;
                              break;
                              }
                            }else{
                           not_found = true;
                           break;
                            }
                          }
                            if not_found{
                              //If not found end delimiter
                                continue;
                              }

                            }
                          }
                        }
                      sub_vec_start = 0;
                      in_ignore = false;
                      copy.replace_range(ignore_start..ignore_end+sub_vec2[1].len(), &general::str_of_n_str(" ", copy[ignore_start..ignore_end+sub_vec2[1].len()].len()));
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
    /// * `ignore_content_between: (&Vec<char>, &Vec<&str>)` - A tuple containing two vectors: `Vec<char>` and `Vec<&str>`.
    /// * `scape_characters:&Vec<char>` - A vector of chars for define the scape characters for ignore end delimiters.
    /// * `mode: ModeBlock` - The mode of block comment removal, either [`ModeBlock::Nested`] or [`ModeBlock::Single`]
    /// * `manage_close: ManageClose` - The mode of ensure the content has his block comments and ignore content correctly close or not, either [`ManageClose::Both`], [`ManageClose::Comment`], [`ManageClose::Ignore`] or [`ManageClose::None`]
    /// # Example
    /// ```rust
    /// mod main_code;
    /// fn main (){
    /// use crate::main_code::utilities::remove_comments;
    /// let content = "example.txt/*fadfjs*/";
    /// let vec_char:Vec<char> = Vec::new();
    /// let vec_str:Vec<String> = Vec::new();
    /// let tuple = (&vec_char, &vec_str);
    /// let scape:Vec<char> = Vec::new();
    /// let result = remove_comments::remove_block_comments(content, "/*", "*/", tuple, &scape, remove_comments::ModeBlock::Single, remove_comments::ManageClose::Both);
    /// }
    /// ```
    /// The result is a file with block comments removed.
    /// # Errors
    /// If the file cannot be read or written, the function will panic with an error message
    /// If there is a block comment without end delimiter, the function will return -1 with an error message.
    /// # Return
    /// * `None` - If there is a block comment without an end delimiter.
    /// * `Some(String)` - If the block comments were successfully removed.
    
    pub fn block_comments(content: &str, start_delimiter: &str, end_delimiter: &str, ignore_content_between: (&Vec<char>, &Vec<&str>), scape_characters:&Vec<char>, mode: ModeBlock, manage_close: ManageClose) -> Option<String>{
      if content.is_empty(){
        panic!("Error: the argument 'conten't is empty");
      }
      if start_delimiter.is_empty() || start_delimiter.contains(" ") || end_delimiter.is_empty() || end_delimiter.contains(" "){
        panic!("Error: start delimiter or end delimiter is empty. Or some comment delimiter contains (' ')");
      }
      if scape_characters.len()>0{
          if scape_characters.contains(&' '){
            println!("Error: The scape characters vector '{:?}' cannot contains some space character (' ')", scape_characters);
            return None;
          }
        }

      if !(ignore_content_between.0.is_empty() && ignore_content_between.1.is_empty()){
       if !ignore_content_between.0.is_empty(){
        for ch in ignore_content_between.0{
          if start_delimiter.contains(*ch)||end_delimiter.contains(*ch){
            println!("Error: The start delimiter '{}' or end delimiter '{}' cannot be in the ignore characters vector '{:?}'", start_delimiter, end_delimiter, ignore_content_between.0);
            return None;
            }
           if *ch == ' '{
            println!("Error: The ignore character '{}' cannot be a space (' ') in the ignore character vector '{:?}'", *ch, ignore_content_between.0);
            return None;
            }
            if scape_characters.len() >0{
             if scape_characters.contains(ch){
              println!("Error: The ignore delimiter '{}' cannot contains a scape character ('{:?}') the ignore characters vector '{:?}'", *ch, scape_characters, ignore_content_between.0);
               return None;
             }
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
          if str.contains(" "){
            println!("Error: The ignore string '{}' cannot contains a space (' ') in the ignore string vector '{:?}'", *str, ignore_content_between.1);
            return None;
          }
          if scape_characters.len() >0{
            for char in str.chars(){
             if scape_characters.contains(&char){
              println!("Error: The ignore delimiter '{}' cannot contains a scape character ('{:?}') the ignore characters vector '{:?}'", *str, scape_characters, ignore_content_between.1);
               return None;
             }
            }
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
        match single_mode(&content, start_delimiter, end_delimiter, ignore_content_between,scape_characters, manage_close){
            Ok(content2) =>  new_content.push_str(&content2) ,
            Err(_) => return None
        }
       }
       ModeBlock::Nested =>{
        match nested_mode(&content, start_delimiter, end_delimiter, ignore_content_between, scape_characters, manage_close){
          Ok(content2) => new_content.push_str(&content2),
          Err(_) => return None
        }
       }
      }
      println!("BLOCK COMMENTS REMOVED FROM CONTENT");
      return Some(new_content);
    }
//------------------------------------------------------------------------------------------

    /// # `ManageClose`
    /// Enum to indicate what type of close you want to verify, and ensure this is correctly close
    /// * Options:
    ///   - `Both`: Ensure the ignore delimiters are correctly close and the block comment are correctly close.
    ///       * Return a error message indicate the ignore delimiter expected if some start delimiter ignore are not close and arrive at the end of input content.
    ///       * Return a error message indicate the line content, and number of line where start the block comment with missing close.
    ///   - `Ignore`: Ensure the ignore delimiters are correctly close.
    ///       * Return a error message indicate the ignore delimiter expected if some start delimiter ignore are not close and arrive at the end of input content.
    ///   - `Comment`: Ensure the block comment are correctly close.
    ///       * Return a error message indicate the line content, and number of line where start the block comment with missing close.
    ///   - `None`: Not make verification. Not recomended 

    pub enum ManageClose{
    Both,
    Comment,
    Ignore, 
    None
   }
    /// # `single_mode`
    /// Removes block comments in single mode from a line.
    /// # Arguments
    /// * `content: &Vec<&str>` - A vector of lines from content from which block comments will be removed.
    /// * `delimiter_end: &str` - The ending delimiter of the block comment.
    /// * `delimiter_start: &str` - The starting delimiter of the block comment.
    /// * `ignore_content_between: (&Vec<char>, &Vec<&str>)` - A tuple containing two vectors: `Vec<char>` and `Vec<&str>`.
    /// * `scape_characters:&Vec<char>` - A vector of chars for define the scape characters for ignore end delimiters.  
    /// * `manage_close: ManageClose` - The mode of ensure the content has his block comments and ignore content correctly close or not, either [`ManageClose::Both`], [`ManageClose::Comment`], [`ManageClose::Ignore`] or [`ManageClose::None`]
    /// * **NOTE:** This is use in his API [`block_comments`] fuction.
    /// # Return
    /// Returns a `Result<String, i32>`:
    /// * `Ok(String)` - If the block comments were successfully removed, returns a `String` with the content without block comments.
    /// * `Err(i32)` - If there is an error, returns an `i32` error code:
    ///   - `-1` - If the start and end delimiters are the same or content vector is empty.
    ///   - `2` - If the block comment are not closed and arrive to the end of content, with an error message indicating the line number and content of the line.
    ///   - `1` - If some ignore start delimiter are not closed and arrive to the end of the content.
 fn single_mode(content: &str, delimiter_start: &str, delimiter_end: &str, ignore_content_between: (&Vec<char>, &Vec<&str>),scape_characters:&Vec<char>, manage_close: ManageClose) -> Result<String, i32>{
      use crate::main_code::utilities::{general, remove_comments::ManageClose};
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
         let mut contains = false; // flag to indicate if the line contains some ignore delimiter
         let mut ignore_delimiter = false; //flag to indicate if have some ignore_delimiter 
         let mut delimiter_ignore = String::new(); // To store the delimiter ignore expected if in_ignore is true
         let mut push = false; //indicate if we push the str before and no need push agains
         let mut some_start_ignore:Vec<String> = Vec::new(); // To store all the start ignore delimiters
         if !ignore_content_between.0.is_empty() || !ignore_content_between.1.is_empty(){ignore_delimiter = true;}
         // Iterate through each line in the content
         // This is a single mode, so we don't need to handle nested comments
         'next: for line in content.lines() {
          push = false;
          counter += 1; // Increment the line counter
          contains = false;
           let mut line_copy= line.to_string(); // copy the line for handle his content
           //If we are in ignore content, search the end of this at the actual line
           if ignore_delimiter{ 
          if in_ignore{
            if let Some(mut end) = line_copy.find(&delimiter_ignore){
              let mut not_found = false;
              if end > 0{
                if scape_characters.len() > 0{
                  if scape_characters.contains(&line.to_string().chars().nth(end-1).unwrap()){
                  line_copy.replace_range(..end+delimiter_ignore.len(), &str_of_n_str(" ", line_copy[..end+delimiter_ignore.len()].len()));
                    loop {
                      if let Some(end2) = line_copy.find(&delimiter_ignore){
                        if scape_characters.contains(&line.to_string().chars().nth(end2-1).unwrap()){
                          line_copy.replace_range(..end2+delimiter_ignore.len(), &str_of_n_str(" ", line_copy[..end2+delimiter_ignore.len()].len()));
                        }else{
                          end = end2;
                          break;
                        }
                        
                      }else{
                        not_found = true;
                        break;
                      }
                    }
                  }
                }
              }
              if !not_found{
              in_ignore = false;
              line_copy.replace_range(..end+delimiter_ignore.len(), &str_of_n_str(" ", line_copy[..end+delimiter_ignore.len()].len()));    
              }               
                  
            }
          }
          //Else, check if the line contains some start ignore delimiter for process
          if !in_ignore{
            some_start_ignore.clear();
            let mut j = 0;
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
              for element in &some_start_ignore{
              if line_copy.contains(element){
                contains = true;
                break;
              }
             }
            }
           }
          }
          //if we aren't in content to ignore
          if !in_ignore{
           // Check if the line contains the start delimiter or if a block comment is already open
           if line_copy.contains(delimiter_start) || block_open{ 

              // While the line contains the start delimiter
               while line_copy.contains(delimiter_start){
                let mut between = false;
                // If a block comment isn't already open, set the line number and content
                // This made for take and store the data for the error message if the block comment isn't closed
                if !block_open{
                  line_num = counter;
                  line_content = line.to_string();
                }
                // Find the position of the start delimiter in the line
               if let Some(mut start_pos) = line_copy.find(delimiter_start){
                let mut no_remove = false;
                // If the start delimiter is found, check if a block comment is already open
                // If not, push the content before the start delimiter to the new content
                if !block_open {
                  //If line contains some start delimiter ignore, check if the start_pos are be into some contento to ignore, and search some start_ignore that not are into ignore content
                  if contains{  
                    let string_before = content_between(ignore_content_between.0, ignore_content_between.1, scape_characters, delimiter_start, &line_copy);
                    //Upload in_ignore flag
                    in_ignore = string_before.1;
                    delimiter_ignore = string_before.0;
                    //If the string contains some start_delimter remove all the content after this for avoid process this
                    if !(string_before.2.len() == line_copy.len()){block_open = true; 
                      line_copy.replace_range(..string_before.2.len(), &str_of_n_str(" ", string_before.2.len()));
                    }
                     
                    //if the line not contains a start delimiter,copy the content and go to the next line
                    else{
                      block_open = false;
                      new_content.push_str(&line.to_string());
                      new_content.push('\n');
                      continue 'next;
                    }
                    //Because if you watch inside the function [`process`] the code just return in_ignore like true if some ignore delimiter start, is not closed and this are before the first delimiter found
                    //so we rewind the block_open flag, and continue to the nex line
                    if in_ignore{
                      line_num = counter;
                      line_content = line.to_string();
                      block_open = false;
                      new_content.push_str(&line.to_string());
                      new_content.push('\n');
                      continue 'next;
                    }
                    start_pos = string_before.2.len(); //because the start delimiter, after upload line_copy move to the position of the len of the string returned from the contet_betwee position or start in this position
                  }
                  //push the content before the start delimiter
                  new_content.push_str(&line[..start_pos]); block_open = true;
                  //remove all before the first start comment
                  line_copy.replace_range(..start_pos, &str_of_n_str(" ", line_copy[..start_pos].len()));
                }
                // If the start delimiter is found, check if the end delimiter is also present in the line
                // We don't make some verify of the end are between string, because the first element or starg ignore delimiter content are before the end delimiter, therefore this start delimiter are into the block comment so we ignore it for this reason
                if let Some(mut end_pos) = line_copy.find(delimiter_end){
                    // For preserved code between comments, but no inside of any of them, in other words, code between start and end block comments delimiters.
                    // The comp its this, becuase the code between comments, is in the start and end of comment, like this '/*thi*/between/*other*/', like we look here, the start delimiter have a greater index than end delimiter
                    //and the content "between" starts after the end delimiter, so we can push en_pos+delimiter_end.len(), and need been not multi-line, because the while loop and all this flux into the for-loop trate with a single line, 
                    //so we need have a way to indicate the comment in some line where open a block comment, is not close, therefore, all after this start must be skiped and ignored.
                    //For avoid problems when the start index and end index superpose like this '*/*' or this '/*/', priorize the end delimiter
                   while end_pos == start_pos+delimiter_start.len()-1 || end_pos+delimiter_end.len()-1 == start_pos{                    
                    //For this case '/*/'
                    if end_pos == start_pos+delimiter_start.len()-1{
                    line_copy = general::replace_index(&line_copy, " ", start_pos);
                    
                   }
                   //For this case '*/*'
                   else{
                    if delimiter_start.len()>1 {
                      line_copy = general::replace_index(&line_copy," ", start_pos+1);
                    }
                    //for case like this '*/' when the start delimtier is '/' and the end is '*', so we no need to remove the start delimiter because no its a "real" superpose
                    else{
                      break;
                    }
                  }
                  //Check if the line contains some start ignore delimiter
                  if !some_start_ignore.is_empty(){
                    for element in &some_start_ignore{
                      if line_copy.contains(element){
                        contains = true;
                        break;
                      }else {contains = false;}
                    }
                  }
                  //If the line contains some start ignore delimiter
                   if contains{
                    let string_before_start = content_between(ignore_content_between.0, ignore_content_between.1,scape_characters, delimiter_start, &line_copy);
                    in_ignore = string_before_start.1;//upload in_ignore flag
                    delimiter_ignore = string_before_start.0;//upload delimiter_ignore
                    //If we are in ignore content that means the start_pos are not found because some start ignore content delimiter already open an not closely in the same line
                    //So not found anyone start delimiter
                    if in_ignore{
                      line_num = counter;
                      line_content = line.to_string();
                      block_open = false;
                      multi_line = false;
                      new_content.push_str(&line[end_pos+delimiter_end.len()..]);
                      new_content.push('\n');
                      continue 'next;
                    }
                    //if not found some start comment delimiter
                    if line_copy.len() == string_before_start.2.len(){
                       start_pos = line_copy.len()+1;
                    }
                    else{
                      start_pos = string_before_start.2.len();
                      //remove all content before start_pos
                      line_copy.replace_range(..start_pos,&str_of_n_str(" ", string_before_start.2.len()));
                    }
                   }
                   //Else don't call content_between
                   else{start_pos = line_copy.find(&delimiter_start).unwrap_or(line_copy.len()+1);}
                   //If not found some start comment delimiter
                   if start_pos == line_copy.len()+1{
                    no_remove = true;
                    start_pos = 0;
                   }
                   else{
                    no_remove = false;
                  }
                }
                  if start_pos > end_pos+delimiter_end.len() && !multi_line{
                    between = true;
                  //get the string after end comment delimiter
                  let string_after = line_copy[end_pos+delimiter_end.len()..].to_string();
                  //call content_between, for aovid start_pos are into ignore content
                  if contains{
                    let verify_ignore = content_between(ignore_content_between.0, ignore_content_between.1, scape_characters, delimiter_start, &string_after);
                  

                  in_ignore = verify_ignore.1;//upload in _ignore
                  delimiter_ignore = verify_ignore.0; //upload delimiter_ignore
                   // if found some start comment delimiter
                    if verify_ignore.2.len() != string_after.len(){
                      start_pos = end_pos+delimiter_end.len()+verify_ignore.2.len();//upload start_pos
                    }
                    //else leave for the loop
                    else{
                      start_pos = line_copy.len();
                      block_open = true;
                      if in_ignore {
                        block_open = false;
                        line_num = counter;
                      line_content = line.to_string();
                      push = true;
                       new_content.push_str(&line[end_pos+delimiter_end.len()..start_pos]);
                       new_content.push('\n');
                      }
                      break;
                    }
                  }
                   //For preserve content between start comments
                    new_content.push_str(&line[end_pos+delimiter_end.len()..start_pos]);
                  
                    // Remove the end delimiter from the line copy to continuing process the next start block comment in the line
                    line_copy.replace_range(end_pos..start_pos, &str_of_n_str(" ", line_copy[end_pos..start_pos].len()));
                    block_open = true;
                     
                     }
                     if !between && !no_remove && (start_pos+delimiter_start.len()< end_pos){line_copy.replace_range(start_pos+delimiter_start.len()..end_pos, &str_of_n_str(" ", line_copy[start_pos+delimiter_start.len()..end_pos].len()));}
                  }
                    // Remove the start delimiter from the line copy, for not process this again, and avoid problems
        
                    if !no_remove {line_copy.replace_range(..start_pos+delimiter_start.len(), &str_of_n_str(" ", line_copy[..start_pos+delimiter_start.len()].len()));}
                }
                
              }
              //if we are not in ignore content
              if !in_ignore{

             //pass here when the line hasn't more start delimiters
              if let Some(mut end_pos) = line_copy.find(delimiter_end){
                  if !some_start_ignore.is_empty(){
                    for element in &some_start_ignore{
                      if line_copy.contains(element){
                        contains = true;
                        break;
                      }else {contains = false;}
                    }
                  }
                // if the line contains some ignore delimiter check this but now with the end comment delimiter
                     if contains{
                      let string_before_first_end = content_between(ignore_content_between.0, ignore_content_between.1, scape_characters, delimiter_end, &line_copy);
                      //if not found a end delimiter in the same line
                      if string_before_first_end.2.len() == line_copy.len(){
                        block_open = true;
                        continue 'next;
                      }
                      end_pos = string_before_first_end.2.len();
                      line_copy.replace_range(..end_pos, &str_of_n_str(" ", string_before_first_end.2.len()));
                     }
                     //Check if the line_copy coitnue contains some start ignore content delimiter
                  if !some_start_ignore.is_empty(){
                    for element in &some_start_ignore{
                      if line_copy.contains(element){
                        contains = true;
                        break;
                      }else {contains = false;}
                    }
                  }
                  //verify all ignore start content delimiter are correctly close
                  if contains{
                    let for_verify_ignore = content_between(ignore_content_between.0, ignore_content_between.1, scape_characters, delimiter_end, &line_copy);
                    in_ignore = for_verify_ignore.1;
                    delimiter_ignore = for_verify_ignore.0;
                    if in_ignore{
                      line_num = counter;
                      line_content = line.to_string();
                    }
                  }
                  
                     
                // If a block comment is open and the end delimiter is found, push the content after the end delimiter to the new content
                // and close the block comment
                if block_open{
                new_content.push_str(&line[end_pos+delimiter_end.len()..]);
                new_content.push('\n');
                block_open = false;
                multi_line = false;
                push = true;
                }
                line_copy.replace_range(..end_pos+delimiter_end.len(), &str_of_n_str(" ", line_copy[..end_pos+delimiter_end.len()].len()));
              }
              //indicate its a multi-line block comment
            else{
                block_open = true;
                multi_line = true;
                continue;
               }
              }
             }
             //verify if some start ignore content delimiter already open
             if contains && !block_open{
             let last_verify_ignore = content_between(ignore_content_between.0, ignore_content_between.1, scape_characters,delimiter_start, &line_copy);
              in_ignore = last_verify_ignore.1;
              delimiter_ignore = last_verify_ignore.0;
              if in_ignore{
                line_num = counter;
                line_content = line.to_string();
              }
            }
             // If the line doesn't contain the start delimiter and a block comment is not open, push the line to the new content
             if (!block_open || in_ignore) && !push {
             new_content.push_str(&line);
             new_content.push('\n');
           }
          }
          //if we are in ignore content push the line in new_content
          else{
            new_content.push_str(&line);
            new_content.push('\n');
          }
         }
         match manage_close{
          ManageClose::Both=>{
               // if some ignore are open after process all the file, print an error
              if in_ignore{
                println!("Error in the line: '{}': '{}'. missing close delimiter: {}", line_num, line_content, delimiter_ignore);
                return Err(1);
              }
              // If a block comment is open at the end of the content, return an error
              if block_open {
                 println!("Error: Block comment without end delimiter in line '{}': '{}'", line_num, line_content);
                 return Err(2);
              }
          }, 
          ManageClose::Comment =>{
            // If a block comment is open at the end of the content, return an error
              if block_open {
                 println!("Error: Block comment without end delimiter in line '{}': '{}'", line_num, line_content);
                 return Err(2);
              }
          }, 
          ManageClose::Ignore  =>{
            if in_ignore{
                println!("Error in the line: '{}': '{}'. missing close delimiter: {}", line_num, line_content, delimiter_ignore);
                return Err(1);
              }
          }, 
          ManageClose::None=>{},
          _ => {panic!("¡FATAL ERROR!: The enum can be 'Ignore', 'Comment' or 'Both'");},
         };

         return Ok(new_content);                
    }

//------------------------------------------------------------------------------------------
    /// # `nested_mode`
    /// Removes block comments in nested mode from a line.
    /// # Arguments
    /// * `content: &str` - A string containing the content from which block comments will be removed.
    /// * `delimiter_start: &str` - The starting delimiter of the block comment.
    /// * `delimiter_end: &str` - The ending delimiter of the block comment.
    /// * `ignore_content_between: (&Vec<char>, &Vec<&str>)` - A tuple containing two vectors: `Vec<char>` and `Vec<&str>`.
    /// * `scape_characters:&Vec<char>` - A vector of chars for define the scape characters for ignore end delimiters.  
    /// * `manage_close: ManageClose` - The mode of ensure the content has his block comments and ignore content correctly close or not, either [`ManageClose::Both`], [`ManageClose::Comment`], [`ManageClose::Ignore`] or [`ManageClose::None`]
    /// * **NOTE:** This is use in his API [`block_comments`] fuction.
    /// # Return
    /// Returns a `Result<String, i32>`:
    /// * `Ok(String)` - If the block comments were successfully removed, returns a `String` with the content without block comments.
    /// * `Err(i32)` - If there is an error, returns an `i32` error code:
    ///   - `-1` - If the start and end delimiters are the same or content vector is empty.
    ///   - `2` - If the block comment are not closed and arrive to the
    ///   - `1` - If some ignore start delimiter are not closed and arrive to the end of the content.
    /// # Use
    /// This function is used to handle nested block comments, where block comments can contain other block comments within them. It ensures that nested comments are properly closed and that the content between comments is preserved.
    /// ## Example
    /// ```rust
    /// mod main_code;
    /// fn main (){
    /// use crate::main_code::utilities::remove_comments;
    /// use std::fs;
    /// use  std::io::Write;
    /// let content = "This is a test /* This is a nested*/ between /* comment /* block comment */*/line/*/*.";
    /// file.write_all(content.as_bytes()).expect("Failed to write to the file");
    /// let vec_char:Vec<char> = Vec::new();
    /// let vec_str:Vec<String> = Vec::new();
    /// let tuple = (&vec_char, &vec_str);
    /// let scape:Vec<char> = Vec::new(); 
    /// let result = remove_comments::block_comments(content, "/*", "*/", tuple, &scape, remove_comments::ModeBlock::Nested, remove_comments::ManageClose::Both);
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
  fn nested_mode(content: &str, delimiter_start: &str, delimiter_end: &str, ignore_content_between: (&Vec<char>, &Vec<&str>), scape_characters:&Vec<char>, manage_close: ManageClose)-> Result<String, i32>{
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
        let mut ignore_delimiter: bool = false;
         if !ignore_content_between.0.is_empty() || !ignore_content_between.1.is_empty(){ignore_delimiter = true;}
         let mut in_ignore = false;
         let mut some_start_ignore: Vec<String> = Vec::new();
         let mut delimiter_ignore = String::new();
         let mut contains = false;
        // Iterate through each line in the content
        // This is a nested mode, so we must to handle nested comments
    for line in content.lines(){
         counter += 1;
         contains = false;
          let mut line_copy: String = line.to_string();

         //If we are in ignore content, search the end of this at the actual line
           if ignore_delimiter{ 
          if in_ignore{
            if let Some(mut end) = line_copy.find(&delimiter_ignore){
              let mut not_found = false;
              if end > 0{
                if scape_characters.len() > 0{
                  if scape_characters.contains(&line.to_string().chars().nth(end-1).unwrap()){
                  line_copy.replace_range(..end+delimiter_ignore.len(), &str_of_n_str(" ", line_copy[..end+delimiter_ignore.len()].len()));
                    loop {
                      if let Some(end2) = line_copy.find(&delimiter_ignore){
                        if scape_characters.contains(&line.to_string().chars().nth(end2-1).unwrap()){
                          line_copy.replace_range(..end2+delimiter_ignore.len(), &str_of_n_str(" ", line_copy[..end2+delimiter_ignore.len()].len()));
                        }else{
                          end = end2;
                          break;
                        }
                        
                      }else{
                        not_found = true;
                        break;
                      }
                    }
                  }
                }
              }
              if !not_found{
              in_ignore = false;
              line_copy.replace_range(..end+delimiter_ignore.len(), &str_of_n_str(" ", line_copy[..end+delimiter_ignore.len()].len()));    
              }               
                  
            }
          }
          //Else, check if the line contains some start ignore delimiter for process
          if !in_ignore{
            some_start_ignore.clear();
            let mut j = 0;
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
              for element in &some_start_ignore{
              if line_copy.contains(element){
                contains = true;
                break;
              }
             }
            }
           }
          }
          if !in_ignore{
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
            let mut found_before = false;
            if !in_block_comment && contains{
              let search_first_start = content_between(ignore_content_between.0, ignore_content_between.1, scape_characters, delimiter_start, &line_copy);
              in_ignore = search_first_start.1;
              delimiter_ignore = search_first_start.0;
              if search_first_start.2.len() == line.len() || in_ignore{
                new_content.push_str(&line.to_string());
                new_content.push('\n');
                continue;
              }
              else{
                found_before = true;
                start = search_first_start.2.len();
              }
            }
             if !found_before {start = line_copy.find(delimiter_start).unwrap();}
            line_num = counter;
            line_content = line.to_string();
            // If a block comment is not already open, push the content before the start delimiter to the new content
            if !in_block_comment{
              new_content.push_str(&line[..start]);
              let index_end = general::all_appears_index(&line_copy[..start], delimiter_end);
              let mut i = 0;
              // If the end delimiter is found before the start delimiter, remove the end delimiters from the indexes_end vector
              while i < index_end.len(){
                 indexes_end.remove(i);
                 i += 1;
              }
              if found_before{line_copy.replace_range(..start, &general::str_of_n_str(" ", line_copy[..start].len()));}
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
              if *value > 0{
            if indexes_end.contains(&(*value+delimiter_start.len()-1))|| indexes_end.contains(&(*value-(delimiter_end.len()-1))){
                if indexes_end.contains(&(*value+delimiter_start.len()-1)){
                  line_copy = general::replace_index(&line_copy, " ", *value+1);
                }
                else{
                  line_copy = general::replace_index(&line_copy, " ", *value);
                }
                indexes_to_delete.push(i);
              }
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
          //Remove the delimiter found in content to ignore
          if contains && !in_block_comment || contains && block_comment_level == 1{
            //remove all before and the first start comment delimiter 
            line_copy.replace_range(..start+delimiter_start.len(), &general::str_of_n_str(" ", line_copy[..start+delimiter_start.len()].len()));
            
            let mut considered_indexes: Vec<usize> = Vec::new();//start delimiter indexes are be out of ignore content
               if indexes_end.len() >0 && indexes.len() > 0{
                //remove all before and the first end comment delimiter
                line_copy.replace_range(..indexes_end[0]+delimiter_end.len(), &general::str_of_n_str(" ", line_copy[..indexes_end[0]+delimiter_end.len()].len()));
                loop{
                  if let Some(mut start_pos) = line_copy.find(delimiter_start){
                    //Check if the start_pos index are in ignore content and fix this when it is
                    //We no need manage case like this '/*hello */ "heilo /*"*/ /* */' because the content_between, return all the line if not found the delimiter start
                    //or the string before the first appear of the start_delimtier and recognize this case in the line '"heilo/*"*/ /*"*/' and know the first appear if in the index 24 an not the index 18
                    let verify_start = content_between(ignore_content_between.0, ignore_content_between.1, scape_characters,delimiter_start, &line_copy);
                    //if not found some start delimiter
                    if verify_start.2.len() == line_copy.len(){
                      //upload in_ignore flag
                      in_ignore = verify_start.1;
                      delimiter_ignore = verify_start.0;
                      break;
                    }
                    else{
                      start_pos = verify_start.2.len();
                      line_copy.replace_range(..start_pos+delimiter_start.len(), &general::str_of_n_str(" ", line_copy[..start_pos+delimiter_start.len()].len()));
                      considered_indexes.push(start_pos);
                    }
                   }else{
                    break;
                   }
                 }
                 let size = considered_indexes.len();
                 //Upload indexes vector 
                 //remove all after the first start delimiter found or the level 1
                 if size > 0{
                 let mut n = 1;
                 if !in_block_comment && block_comment_level == 0{
                  while indexes.len() > 1{                    
                    if line_indexes_start[n] == line_indexes_end{
                      indexes.remove(n);
                      line_indexes_start.remove(n);
                      n= 1;
                    }
                    else{n+=1;}
                   }
                  }
                  else if block_comment_level == 1{
                    n = 0;
                    while indexes.len() > 0{
                    if line_indexes_start[n] == line_indexes_end{
                      indexes.remove(n);
                      line_indexes_start.remove(n);
                      n=0;
                    }
                    n+=1;
                    }
                  }
                 for n in considered_indexes{
                  indexes.push(n);
                  line_indexes_start.push(counter);
                 }
                }
               }
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
                //Here we use indexes 0 or 1 because in each iterate we remove the index 0 of the vectors, and the level 1 or first level of block comment, manage down of this part
                // We just made this comprobation the line becuase we just need make this in the same line, can't have content between comments in multiple lines
               if indexes_end[i] < indexes[i+1] && !in_block_comment && line_indexes_start[i+1] == line_indexes_end{
                let mut remove_end_between: Vec<usize> = Vec::new();
                for (s, n) in indexes_end.iter().enumerate(){
                  if *n < indexes[i+1] && *n > indexes_end[i]{
                    remove_end_between.push(s);
                  }
                }
                //remove all delimiter end between block comments for avoid problems
                let mut removed = 0; //indexes removed for consider the decrement
                for n in remove_end_between{
                  indexes_end.remove(n-removed);
                  removed+=1;
                }
                 new_content.push_str(&line[indexes_end[i]+delimiter_end.len()..indexes[i+1]]);
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
                 new_content.push_str(&line[indexes_end[i]+delimiter_end.len()..]);
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
          //Last verify of ignore delimiters
          let last_verify = content_between(ignore_content_between.0, ignore_content_between.1, scape_characters, delimiter_start, &line_copy);
          in_ignore = last_verify.1;
          delimiter_ignore = last_verify.0;
        }
        if !in_block_comment && !processed{
         new_content.push_str(&line.to_string());
         new_content.push('\n');
       }
       }else{
        new_content.push_str(&line.to_string());
         new_content.push('\n');
       }
     }
         match manage_close{
          ManageClose::Both=>{
               // if some ignore are open after process all the file, print an error
              if in_ignore{
                println!("Error in the line: '{}': '{}'. missing close delimiter: {}", line_num, line_content, delimiter_ignore);
                return Err(1);
              }
              // If a block comment is open at the end of the content, return an error
              if in_block_comment || block_comment_level > 0{
                println!("Error: Block comment without end delimiter in line '{}': '{}'\n MISSING COMMENTS TO CLOSE: {}", line_num, line_content, block_comment_level);
                return Err(2);
              }
          }, 
          ManageClose::Comment =>{
            // If a block comment is open at the end of the content, return an error
              if in_block_comment || block_comment_level > 0{
                println!("Error: Block comment without end delimiter in line '{}': '{}'\n MISSING COMMENTS TO CLOSE: {}", line_num, line_content, block_comment_level);
                return Err(2);
              }
          }, 
          ManageClose::Ignore  =>{
            if in_ignore{
                println!("Error in the line: '{}': '{}'. missing close delimiter: {}", line_num, line_content, delimiter_ignore);
                return Err(1);
              }
          }, 
          ManageClose::None=>{},
          _ => {panic!("¡FATAL ERROR!: The enum can be 'Ignore', 'Comment' or 'Both'");},
         };
        return Ok(new_content);
    
        
  
  }

//------------------------------------------------------------------------------------------
#[cfg(test)]
  mod tests{
     use super::*;
       #[test]
       /// # [`super::simple_comments`] Test
      fn test_simple_comments(){
        let str = "Not remove this // remove this\nother // abcdefghijklm";
        let scape:Vec<char> = Vec::new(); //without scape characters
       let vec_str:Vec<&str> = Vec::new();
       let vec_char:Vec<char> = Vec::new();
       let ignore = (&vec_char, &vec_str); //tuple with empty vectors
       assert_eq!("Not remove this \nother \n".to_string(), super::simple_comments(str, "//", ignore, &scape, true).unwrap());
      }
       
      #[test]
      /// # [`super::simple_comments`] Test 2
      /// test using scape characters and ignore content characters
      fn test_2_simple_comments(){
        let str = "Not remove this 'this is a string// \\'' //remove this\nother // abcdefghijklm";
        let scape:Vec<char> = vec!['\\']; //without scape characters
       let vec_str:Vec<&str> = vec!["'", "'"];
       let vec_char:Vec<char> = vec![];
       let ignore = (&vec_char, &vec_str); //tuple with empty vectors
       assert_eq!("Not remove this 'this is a string// \\'' \nother \n".to_string(), super::simple_comments(str, "//", ignore, &scape, true).unwrap());
      }

      #[test]
      /// # [`super::simple_comments`] Test 3
      /// test where the ignore delimiter is not closed, expect an error
      fn test_3_simple_comments(){
        let str = "Not remove this 'this is a string// \\' //remove this\nother // abcdefghijklm";
        let scape:Vec<char> = vec!['\\']; //without scape characters
       let vec_str:Vec<&str> = vec!["'", "'"];
       let vec_char:Vec<char> = vec![];
       let ignore = (&vec_char, &vec_str); //tuple with empty vectors
       assert_eq!(None, super::simple_comments(str, "//", ignore, &scape, true));
      }

      #[test]
      /// # [`super::simple_comments`] Test 4
      /// test where the ignore delimiter is not closed, but not manage this error
      fn test_4_simple_comments(){
        let str = "Not remove this 'this is a string// \\' //remove this\nother // abcdefghijklm";
        let scape:Vec<char> = vec!['\\']; //without scape characters
       let vec_str:Vec<&str> = vec!["'", "'"];
       let vec_char:Vec<char> = vec![];
       let ignore = (&vec_char, &vec_str); //tuple with empty vectors
       assert_eq!("Not remove this 'this is a string// \\' //remove this\nother // abcdefghijklm\n", super::simple_comments(str, "//", ignore, &scape, false).unwrap());
      }

      #[test]
      /// # [`super::simple_comments`] Test 5
      /// test where the ignore delimiters aren't correctly structured
      fn test_5_simple_comments(){
        let str = "Not remove this 'this is a string// \\' //remove this\nother // abcdefghijklm";
        let scape:Vec<char> = vec!['\\']; //without scape characters
       let vec_str:Vec<&str> = vec!["'"];
       let vec_char:Vec<char> = vec![];
       let ignore = (&vec_char, &vec_str); //tuple with empty vectors
       assert_eq!(None, super::simple_comments(str, "//", ignore, &scape, false));
      }

      #[test]
      /// # [`super::simple_comments`] Test 6
      /// test where the delimiter ignore contains space trigger an error
      fn test_6_simple_comments(){
        let str = "Not remove this 'this is a string// \\' //remove this\nother // abcdefghijklm";
        let scape:Vec<char> = vec!['\\']; //without scape characters
       let vec_str:Vec<&str> = vec!["' ", "'"];
       let vec_char:Vec<char> = vec![];
       let ignore = (&vec_char, &vec_str); //tuple with empty vectors
       assert_eq!(None, super::simple_comments(str, "//", ignore, &scape, true));
      }

      #[test]
      #[should_panic]
      /// # [`super::simple_comments`] Test 7
      /// test where the delimiter contains space trigger an error
      fn test_7_simple_comments(){
        let str = "Not remove this 'this is a string// \\' //remove this\nother // abcdefghijklm";
        let scape:Vec<char> = vec!['\\']; //without scape characters
       let vec_str:Vec<&str> = vec!["'", "'"];
       let vec_char:Vec<char> = vec![];
       let ignore = (&vec_char, &vec_str); //tuple with empty vectors
       assert_eq!(None, super::simple_comments(str, "// ", ignore, &scape, true));
      }

      #[test]
      /// # [`super::simple_comments`] Test 8
      /// test where the scape character contains space trigger an error
      fn test_8_simple_comments(){
        let str = "Not remove this 'this is a string// \\' //remove this\nother // abcdefghijklm";
        let scape:Vec<char> = vec!['\\', ' ']; //without scape characters
       let vec_str:Vec<&str> = vec!["'", "'"];
       let vec_char:Vec<char> = vec![];
       let ignore = (&vec_char, &vec_str); //tuple with empty vectors
       assert_eq!(None, super::simple_comments(str, "//", ignore, &scape, true));
      }

      #[test]
      /// # [`super::content_between`] Test
      fn test_content_between(){
        let str = "Not remove this 'this is a string// \\'' //remove this";
        let scape:Vec<char> = vec!['\\']; //without scape characters
       let vec_str:Vec<&str> = vec!["'", "'"];
       let vec_char:Vec<char> = vec![];
       assert_eq!(("".to_string(), false, "Not remove this 'this is a string// \\'' ".to_string()), super::content_between(&vec_char, &vec_str, &scape, "//", str));
      }

      #[test]
      /// # [`super::content_between`] Test 2
      /// test where the ignore delimiter is not closed
      fn test_2_content_between(){
        let str = "Not remove this 'this is a string// \\' //remove this";
        let scape:Vec<char> = vec!['\\']; //without scape characters
       let vec_str:Vec<&str> = vec!["'", "'"];
       let vec_char:Vec<char> = vec![];
       assert_eq!(("'".to_string(), true, "Not remove this 'this is a string// \\' //remove this".to_string()), super::content_between(&vec_char, &vec_str, &scape, "//", str));
      }

      #[test]
      #[should_panic]
      /// # [`super::content_between`] Test 3
      /// test where occurs an error, because the delimiter contains, or delimiter, o scape characters conatins the space character ' ' or str are empty
      fn test_3_content_between(){
        let str = "Not remove this 'this is a string// \\' //remove this";
        let scape:Vec<char> = vec!['\\']; //without scape characters
       let vec_str:Vec<&str> = vec!["'", "' "];
       let vec_char:Vec<char> = vec![];
       assert_eq!(("'".to_string(), true, "Not remove this 'this is a string// \\' //remove this".to_string()), super::content_between(&vec_char, &vec_str, &scape, "", str));
      }

      #[test]
      /// # [`super::block_comments`] Test 
      fn test_block_comments(){

      }




  }
}
