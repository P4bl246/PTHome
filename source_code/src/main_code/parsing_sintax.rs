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
            Ok(i) => processed = i,
            Err(_)=> return None,
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
      #![allow(unused)]
      use std::fs;
      use crate::main_code::utilities::{general, remove_comments};
      /// # `filter_lines`
      /// Search strings in the content and filter lines with the string and without this.
      /// # Arguments
      /// * `content:&str` - string from filter the content.
      /// * `search_this: &Vec<&str>` - vector of strings for filter the content.
      /// * `scape_characters` - vector of character than indicate the scape character for ignore end delimiters. (can be empty)
      /// * `ignore_content_between:(&Vec<char>, &Vec<&str>)` - delimiters than indicate content to ignore between those. (the vectors can be empty)
      /// # Return 
      /// * Panic if the search_this or content parameter are empty or search_this contains some space.
      /// * `None` - if occurs some recupareble errors on the parameters.
      /// * `Some(Vec<Vec<usize>>)` - Vector of vectors with the number of the lines from less to greater, and in order to the string in search_this vector.
      /// # Example 
      /// ```rust
      /// mod main_code;
      /// fn main(){
      /// use crate::main_code::parsing_sintax;
      /// let content = "Hello, this is\nHello, this is\nChao";
      /// let scape:Vec<char> = Vec::new();
      /// let indexes = parsing_sintax::filter_lines(content, &["Hello", "Chao", "-"].to_vec(), &scape, (&[].to_vec(), &[].to_vec())).unwrap();
      /// println!("{:?}", indexes);
      /// }
      /// ```
    pub fn filter_lines(content:&str, search_this: &Vec<&str>, scape_characters: &Vec<char>, ignore_content_between:(&Vec<char>, &Vec<&str>)) -> Option<Vec<Vec<usize>>>{
    if  search_this.is_empty(){
            panic!("Error: The delimiter to search cannot be empty.");
        }
        if search_this.contains(&&" "){
            panic!("Error: The delimiter to search cannot contains a space (' ').");
        }
        if content.is_empty(){
          panic!("Error: The content cannot be an empty string.");
        }
        if !remove_comments::first_comprobation(ignore_content_between.0, ignore_content_between.1, scape_characters, search_this){
            return None;
        }

      println!("FILTERING LINES");
        let mut lines_slice:Vec<Vec<usize>> = Vec::new();
        let mut delimiter_ignore = String::new();
        let mut in_ignore = false;
        let mut ignore_delimiters = false;
        let mut contains = false;
        let mut last_push = 0;
        let mut counter = 0;
         if !ignore_content_between.0.is_empty() || !ignore_content_between.1.is_empty(){ignore_delimiters = true;}
        let mut map:general::Map<usize, usize> = general::Map::new();
        //iterate in each line
        for line in content.lines(){
            counter += 1;
            contains = false;
        let mut copy = line.to_string();
        //if we are into ignore content search the end delimiter ignore
        if in_ignore{
            if let Some(mut end) = copy.find(&delimiter_ignore){
              let mut not_found = false;
              if end > 0{
                if scape_characters.len() > 0{
                  if scape_characters.contains(&line.to_string().chars().nth(end-1).unwrap()){
                  copy.replace_range(..end+delimiter_ignore.len(), &general::str_of_n_str(" ", copy[..end+delimiter_ignore.len()].len()));
                    loop {
                      if let Some(end2) = copy.find(&delimiter_ignore){
                        if scape_characters.contains(&line.to_string().chars().nth(end2-1).unwrap()){
                          copy.replace_range(..end2+delimiter_ignore.len(), &general::str_of_n_str(" ", copy[..end2+delimiter_ignore.len()].len()));
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
              copy.replace_range(..end+delimiter_ignore.len(), &general::str_of_n_str(" ", copy[..end+delimiter_ignore.len()].len()));    
              }           
            }
          }
          //Else, check if the line contains some ignore delimiter
            if !in_ignore{
             contains = remove_comments::contains_ignore(ignore_content_between.0, ignore_content_between.1, &copy);
          }
        
          if !in_ignore{
            //Iterate in each search string for search this at the actual line
            for (n , i) in search_this.iter().enumerate(){
                //If the line contains some start ignore delimiter
                    if contains{
                      let verify = remove_comments::content_between(ignore_content_between.0, ignore_content_between.1, &scape_characters, i, &copy);
                      in_ignore = verify.1;
                      delimiter_ignore = verify.0;
                      //If found the string to search
                      if verify.2.len() != copy.len(){
                        map.insert(&n, &counter); //Store the index of the string found and the number of line
                      }
                    }
                    //Else, if the line haven't any start ignore delimiter
                    else{
                       map.insert(&n, &counter);
                    } 
                }
            }
        }

      {
        //agrupe the lines with the same delimiter for search
        for i in search_this.iter().enumerate(){
        let mut buffer = Vec::new();//buffer 
         // search all the lines with the delimiter
              loop{
                let tempo = map.get(&i.0);
                match tempo{
                  Some(n) => {buffer.push(*n); map.remove(&i.0);},
                  None => {break;}
                }
              }
              lines_slice.push(buffer);
        }
      }
      println!("LINES FILTERED");

        return Some((lines_slice)); 
    }
//------------------------------------------------------------------------------------
    pub fn parser_classes(content: &str){
      
    }
//------------------------------------------------------------------------------------
    /// # `extract_str_before`
      /// Search strings delimiters and extract the string or content before the first delimiter appear.
      /// # Arguments
      /// * `content:&str` - string from filter the content.
      /// * `delimiter_slice: &Vec<&str>` - vector of strings than indicate the delimiters.
      /// * `scape_characters` - vector of character than indicate the scape character for ignore end delimiters. (can be empty)
      /// * `ignore_content_between:(&Vec<char>, &Vec<&str>)` - delimiters than indicate content to ignore between those. (the vectors can be empty)
      /// # Return 
      /// * Panic if the delimiter_slice or content parameter are empty or delimiter_slice contains some space.
      /// * `None` - if occurs some recupareble errors on the parameters.
      /// * `Some((Vec<Vec<String>>, Vec<Vec<usize>>))` - tuple of vectors:
      ///   * `Vec<Vec<String>>` - Vector of content before extracting, in order of the delimiter_slice vector.
      ///   * `Vec<Vec<usize>>` - Number of line where found that delimiter and extract that string. (in order of delimiter_slice vector)
      /// # Example 
      /// ```rust
      /// mod main_code;
      /// fn main(){
      /// use crate::main_code::parsing_sintax;
      /// let content = "Hello, this is\nHello, this is\nChao";
      /// let scape:Vec<char> = Vec::new();
      /// let indexes = parsing_sintax::extract_str_before(content, &["t"].to_vec(), &scape, (&[].to_vec(), &[].to_vec())).unwrap();
      /// println!("{:?}", indexes);
      /// }
      /// ```
    pub fn extract_str_before(content: &str, delimiter_slice:&Vec<&str>, scape_characters: &Vec<char>, ignore_content_between:(&Vec<char>, &Vec<&str>)) -> Option<(Vec<Vec<String>>, Vec<Vec<usize>>)>{
         if content.is_empty(){
            panic!("Error:The content cannot be an empty string");
         }
         if delimiter_slice.is_empty(){
            panic!("Error: The indicator of the classes cannot be empty")
         }
         if delimiter_slice.contains(&&" "){
            panic!("Error: The indicator of the classes '{:?}' cannot contains space", delimiter_slice);
         }
        if !remove_comments::first_comprobation(ignore_content_between.0, ignore_content_between.1, scape_characters, delimiter_slice){
            return None;
        }
        println!("EXTRACTING CONTENT BEFORE DELIMITERS");
        let mut contains = false;
        let mut counter = 0;
        let mut content_before:Vec<Vec<String>> = Vec::new();
        let mut in_ignore = false;
        let mut delimiter_ignore = String::new();

        let mut num_line: general::Map<usize, usize> = general::Map::new();
        let mut indexes: Vec<Vec<usize>> = Vec::new();
        let mut map:general::Map<usize, String> = general::Map::new();
        for line in content.lines(){
            counter += 1;
            contains = false;
        let mut copy = line.to_string();
        //if we are into ignore content search the end delimiter ignore
            if in_ignore{
            if let Some(mut end) = copy.find(&delimiter_ignore){
              let mut not_found = false;
              if end > 0{
                if scape_characters.len() > 0{
                  if scape_characters.contains(&line.to_string().chars().nth(end-1).unwrap()){
                  copy.replace_range(..end+delimiter_ignore.len(), &general::str_of_n_str(" ", copy[..end+delimiter_ignore.len()].len()));
                    loop {
                      if let Some(end2) = copy.find(&delimiter_ignore){
                        if scape_characters.contains(&line.to_string().chars().nth(end2-1).unwrap()){
                          copy.replace_range(..end2+delimiter_ignore.len(), &general::str_of_n_str(" ", copy[..end2+delimiter_ignore.len()].len()));
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
              copy.replace_range(..end+delimiter_ignore.len(), &general::str_of_n_str(" ", copy[..end+delimiter_ignore.len()].len()));    
              }           
            }
          }
          //Else, check if the line contains some ignore delimiter
            if !in_ignore{
             contains = remove_comments::contains_ignore(ignore_content_between.0, ignore_content_between.1, &copy);
          }
          if !in_ignore{
            for (i, delimiter) in delimiter_slice.iter().enumerate(){
             if copy.contains(delimiter){
                if contains{
                  let verify = remove_comments::content_between(ignore_content_between.0, ignore_content_between.1, scape_characters, delimiter, &copy);
                  in_ignore = verify.1;
                  delimiter_ignore = verify.0;
                  if verify.2.len() != copy.len(){
                    map.insert(&i, &line[..verify.2.len()].to_string());
                    num_line.insert(&i, &counter);
                  }
                }else{
                  if let Some(pos) = copy.find(delimiter){
                    map.insert(&i,& line[..pos].to_string());
                    num_line.insert(&i, &counter);
                  }
                }
              }
            }
          }
        }
        {
          
          for (i, s) in delimiter_slice.iter().enumerate(){
            let mut buffer: Vec<usize> = Vec::new();
            let mut buffer_values: Vec<String> = Vec::new();

            //Agrupe strings before and the num of line where it found
            loop{
               match map.get(&i){
                Some(n) => {buffer_values.push(n.clone()); map.remove(&i);},
                None => {break;},
               };
              }
            loop{  
              match num_line.get(&i){
                Some(n) => {buffer.push(n.clone()); num_line.remove(&i);},
                None => {break;}
              };
            }
              content_before.push(buffer_values.clone());
            indexes.push(buffer.clone());
            }
            
          }
        println!("CONTENT BEFORE DELIMITERS EXTRACTED");
         
         return Some((content_before, indexes));
    }
//------------------------------------------------------------------------------------
    pub fn parser_equialites(){}
 }