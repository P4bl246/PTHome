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
      /// * `Some(Vec<Vec<usize>>)` - Vector of vectors with the number of the lines from less than to greather than, and in order to the string in search_this vector.
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
          if search_this.contains(&&(*ch.to_string().as_str())){
            println!("Error: The delimiters '{:?}' cannot be in the ignore characters vector '{:?}'", search_this, ignore_content_between.0);
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
          if search_this.contains(ch){
            println!("Error: The delimiters '{:?}' cannot be in the ignore strings vector '{:?}'", search_this, ignore_content_between.1);
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

      println!("FILTERING LINES");
        let mut lines_slice:Vec<Vec<usize>> = Vec::new();
        let mut delimiter_ignore = String::new();
        let mut in_ignore = false;
        let mut ignore_delimiters = false;
        let mut contains = false;
        let mut last_push = 0;
        let mut counter = 0;
         if !ignore_content_between.0.is_empty() || !ignore_content_between.1.is_empty(){ignore_delimiters = true;}
        let mut index:Vec<usize> = Vec::new();
        let mut value:Vec<usize> = Vec::new();
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
        
          if !in_ignore{
            //Iterate in each search string for search this at the actual line
            for (n , i) in search_this.iter().enumerate(){
                //If the line contains a string to search
                if copy.contains(*i){
                //If the line contains some start ignore delimiter
                    if contains{
                      let verify = remove_comments::content_between(ignore_content_between.0, ignore_content_between.1, &scape_characters, i, &copy);
                      in_ignore = verify.1;
                      delimiter_ignore = verify.0;
                      //If found the string to search
                      if verify.2.len() != copy.len(){
                        index.push(n); //Store the index of the string found
                        value.push(counter);//Put the line, this have the index as a key
                        //remove the index found
                      copy.replace_range(verify.2.len()..verify.2.len()+i.len(), &general::str_of_n_str(" ", copy[verify.2.len()..verify.2.len()+i.len()].len()));
                      last_push = n;
                      }
                      //Else, if the line haven't any string to search
                      
                    }
                    //Else, if the line haven't any start ignore delimiter
                    else{
                       index.push(n);
                        value.push(counter);
                        if let Some(pos) = copy.find(*i){
                    copy.replace_range(pos..pos+i.len(), &general::str_of_n_str(" ", copy[pos..pos+i.len()].len()));
                        }
                    } 
                }
            }
          }
          else{
            index.push(last_push);
            value.push(counter);
          }
        }

      {
        //agrupe the lines with the same delimiter for search
        for i in search_this.iter().enumerate(){
        let mut buffer = Vec::new();//buffer 
        let mut remove = Vec::new();
         // search all the lines with the delimiter
            if index.contains(&i.0){
                for (s, n) in index.iter().enumerate(){
                    if *n == i.0{
                        buffer.push(value[s].clone());
                        remove.push(s);
                    }
                }
                let mut fix_dcr = 0;
                //Remove the indexes processed
                for n in remove{
                    index.remove(n-fix_dcr);
                    value.remove(n-fix_dcr);
                    fix_dcr += 1;
                }
            }
                lines_slice.push(buffer.clone());
            
        }
      }
      println!("LINES FILTERED");

        return Some((lines_slice)); 
    }
    pub fn parse_equialites(){}
    pub fn parse_classes(){}
 }