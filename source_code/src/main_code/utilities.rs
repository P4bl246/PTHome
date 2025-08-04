/// # Mod `general` from `utilities.rs`
/// This module provides general utility functions for file operations use in the soruce code of PTHome aplication.
/// It includes functions to remove empty lines, numerate lines, skip line numbers, and others.
/// # FUNCTIONS
/// * [`remove_empty_lines`] - Removes empty lines from a file.
/// * [`NumLines`] - A struct to hold the configuration for handle `numerate_lines` with methods.
/// * [`NumLines::numerate_lines`] - Numerates the lines of a file.
/// * [`NumLines::remove_num_lines`] - Removes line numbers from a file.
/// * [`NumLines::skip_num_line`] - Skips the line number in a file.
/// * [`NumLines::get_num_line`] - Gets the current line number from a file
/// * [`remove_str`] - Removes a specific string from a file.
pub mod general{
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
    /// A struct to hold the configuration for the `numerate_lines` function and his metods.
    pub struct NumLines {
        pub input_file: &'static str,
        pub delimiter: &'static str,
    }
//---------------------------------------------------------------------
    /// ## `impl num_lines`
    /// This implementation provides metods to handle line numbers in a file.
    /// It includes functions to push and remove line numbers, skip line numbers, and get the current line number.
    impl NumLines{
       /// ### `numerate_lines`
       /// Numerates the lines of a file and rewrites it.
       /// #### Arguments
       /// * `&self` - The configuration in the `num_lines` struct.
       /// #### Example
       /// ```rust
       /// mod main_code;
       /// fn main (){
       /// use crate::main_code::utilities::general;
       /// 
       /// let input_file = general::NumLines{
       ///     input_file: "example.txt",
       ///      delimiter: " - ",
       ///    };
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
        /// ### remove_num_lines
        /// Removes line numbers from a file. Recomended use this, just if before you use the function `numerate_lines`.
        /// #### Arguments
        /// * `&self` - The configuration in the `num_lines` struct.
        /// #### Example
        /// ```rust
        /// mod main_code;
        /// fn main (){
        /// use crate::main_code::utilities::general;
        /// let input_file = general::NumLines{
        ///     input_file: "example.txt",
        ///     delimiter: " - "
        /// };
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
        /// Skips the line number in a file. Use this just if before you use the function `numerate_lines`.
        /// #### Arguments
        /// * `line: &str` - The line from which the line number will be skipped.
        /// #### Example
        /// ```rust
        /// mod main_code;
        /// fn main (){
        /// use crate::main_code::utilities::fn_num_lines;
        /// let input_file = general::NumLines{
        ///     input_file: "example.txt",
        ///      delimiter: " - ",
        /// };
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
        /// #### Example
        /// ```rust
        /// mod main_code;
        ///fn main (){
        /// use crate::main_code::utilities::fn_num_lines;
        /// let input_file = general::NumLines{
        ///     input_file: "example.txt",
        ///      delimiter: " - ",
        ///    };
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
    }
//------------------------------------------------------------------------------------------
    /// ## `remove_str`
    /// Removes a specific string from a file and rewrites it.
    /// ### Arguments
    /// * `remove: &str` - The string to be removed from the file.
    /// * `input_file: &str` - The path to the file from which the string will be removed.
    /// ### Example
    /// ```rust
    /// mod main_code;
    /// fn main (){
    /// use crate::main_code::utilities::general;
    /// let input_file = "example.txt";
    /// general::remove_str("string_to_remove", input_file);
    /// }
    /// ```
    /// The result is a file with the specified string removed.
    /// ### Errors
    /// If the file cannot be read or written, the function will panic with an error message.
  pub fn remove_str(remove: &str, input_file: &str) {
    println!("REMOVING STRING '{}' FROM FILE: {}", remove, input_file);
    let mut new_content = String::new();
    {
        let content = fs::read_to_string(input_file).expect(&format!("Error trying to open the file '{}'", input_file));
        for line in content.lines() {
            if line.contains(remove) {
                let without_string = line.replace(remove, "");
                new_content.push_str(&format!("{}\n", without_string));
                continue;
            }
            new_content.push_str(&format!("{}\n", line));
        }
    }
    fs::remove_file(input_file).expect(&format!("Error trying to remove the file '{}'", input_file));
    let mut new_file = fs::File::create(input_file).expect(&format!("Error trying to create the file '{}'", input_file));
    new_file.write_all(new_content.as_bytes()).expect(&format!("Error trying to write in the file '{}'", input_file));
    println!("STRING '{}' REMOVED FROM FILE: {}", remove, input_file);

  }
//------------------------------------------------------------------------------------------
}

