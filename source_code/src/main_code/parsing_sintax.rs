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
        let mut map:general::Map<usize, usize, i32> = general::Map::new();
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
use std::collections::HashMap;
use std::collections::VecDeque;
  pub struct Tree
  {
    branch: HashMap<VecDeque<String>, fn(&str, &VecDeque<String>)->(bool, usize, VecDeque<String>, VecDeque<String>, String, String)>
  }
  impl Tree{
    pub fn new()->Self{
      Self{
        branch:HashMap::new()
      }
    }
    pub fn insert(&mut self, key:VecDeque<String>, value:fn(&str, &VecDeque<String>)->(bool, usize, VecDeque<String>, VecDeque<String>, String, String)){
      self.branch.insert(key, value);
    }
    pub fn parse(content:&str){
      let mut n = String::new();
      let mut tree_hashs = Tree::new();
      init_hashs(&mut tree_hashs);
      while n.len() > 0{

      }

    }
  }
  pub fn init_hashs(tree_for_push_hash: &mut Tree){
   /* let wave_keys = wave_keys;
    {
    let mut vec = VecDeque::new();
    vec.push_back("{".to_string());
    vec.push_back("}".to_string());
    tree_for_push_hash.insert(vec, wave_keys);
    }*/

    {
      let mut vec = VecDeque::new();
    vec.push_back("'".to_string());
    vec.push_back("'".to_string());
    let literal = verify_liter;
    tree_for_push_hash.insert(vec, literal);
    }

    {
      let mut vec = VecDeque::new();
      vec.push_back("[".to_string());
      vec.push_back("]".to_string());
      let el_pos = verify_el_pos;
      tree_for_push_hash.insert(vec, el_pos);
    }

    {}

    {}

    {}

    {}

    {}

    {}

    {}

    {}

    {}
  }
  /*pub fn wave_keys(content: &str, vec:&VecDeque<String>)->(bool, usize, VecDeque<String>, VecDeque<String>, String, String){   
    
    return (false, 0, "".to_string(), "".to_string(), "".to_string())
  }*/
  
  pub fn verify_liter(content: &str, vec:&VecDeque<String>)->(bool, usize, VecDeque<String>, VecDeque<String>, String, String){
    let mut errs = VecDeque::new();
    let mut warns = VecDeque::new();
    if let Some(end) = content.find("'"){
      let mut after = content.to_string();
      after.replace_range(..end+"'".len(), "");
      return (true, content[..end+"'".len()].len() ,errs, warns, "".to_string(), after);  
    }
    errs.push_back("FATAL ERROR:Not found end delimiter for the simbol \" ' \" than indicate the end of a ".to_string());
   return (false, 0,errs , warns, "'".to_string(), "".to_string());
  }
  
  /*pub fn verify_roudn_keys(content: &str, vec:&VecDeque<String>)->(bool, usize, VecDeque<String>, VecDeque<String>, String, String){
    return (false, 0, "".to_string(), "".to_string(), "".to_string());
  }*/

  pub fn verify_el_pos(content: &str, elements: &VecDeque<String>)->(bool, usize, VecDeque<String>, VecDeque<String>, String, String){
      let num_elem = elements.len();
      let mut chars = content.chars();
      let mut el = String::new();
      let mut pos = String::new();
      let mut in_pos = false;
      let mut vec_warn = VecDeque::new();
      let mut vec_errs = VecDeque::new();
      let mut found_end = false;
        vec_errs.push_back(format!("FATAL ERROR: The [El:Pos] '{}' instrucion haven't the close expected ']'", content.to_string()));
    while let Some(c) = chars.next() {
          if c != ':' && c != ']' && c != ',' && !in_pos{
           el.push(c.clone());
          }else if c == ':' && !in_pos{
            in_pos = true;
            match pos.parse::<usize>(){
              Ok(digit) =>if digit > num_elem{
            vec_warn.push_back(format!("PRIORITY WARNING!: In the [El:Pos] '{}' instruction, the `El` is: '{}', but the max of elements is '{}' ", content.to_string(),digit, num_elem));
              },
             
             Err(_) => {vec_errs.push_back(format!("ERROR: The [El:Pos] '{}' instruction, the `El` contains a character diferent to digit", content.to_string()));}
            };
          }
          if c != ':' && c != ']' && c != ',' && in_pos{
            pos.push(c.clone());
          }else if c == ']'{
              match pos.parse::<usize>(){
              Ok(digit) =>if digit > num_elem{
            vec_warn.push_back(format!("PRIORITY WARNING!: In the [El:Pos] '{}' instruction, the `Pos` is: '{}', but the max of positions is '{}' ", content.to_string(),digit, num_elem));
              },
             
             Err(_) => {vec_errs.push_back(format!("ERROR: The [El:Pos] '{}' instruction, the `Pos` contains a character diferent to digit", content.to_string()));}
            
          };
            found_end = true;
            break;
          }else if c ==  ','{
             in_pos = false;
          }   
      }
      if found_end{
        vec_errs.pop_front();
        return (true, content.len(), vec_errs, vec_warn, "".to_string(), "".to_string());
      }
      return (false, content.len(), vec_errs, vec_warn, "".to_string(), "".to_string())

  }
  
 /* pub fn verify_oblig_appears(content: &str, elements: &VecDeque<String>)->(bool, usize, VecDeque<String>, VecDeque<String>, String, String){
     
      }*/
  //------------------------------------------------------------------------------------
    pub fn parser_equ(equalities_map: &general::Map<String, String, i32>, classes_map: &general::Map<String, String, i32>){
      for key in equalities_map.iter(){
        for i in key.1 {
          let mut value = i.trim().to_string();
          //let all_fine = tree.parse(&value);
        }
      }
    }

 //--------------------------------------------------------------------------------------   
    pub fn slice_classes_equ(content: &str)-> Option<(VecDeque<general::Map<String, String, i32>>, VecDeque<general::Map<VecDeque<String>, String, i32>>)>{
      let mut n = extract_str_before(content, &["=", ":", "_"].to_vec(),&['\\'].to_vec(), 
      (&['"', '"', '$', '$', '`', '`', '&', '&'].to_vec(),&["'", "'", "{", "}", "(", ")", "[", "]" ,"/", "/", "--", "--"].to_vec()));
      match n {
        None => None,
        Some(mut i) => {
          if !i.1.is_empty(){
            let mut map: general::Map<String, String, i32> = general::Map::new();
            let mut map_eq:general::Map<String, String, i32> = general::Map::new();
            let mut map_for_buffer:general::Map<VecDeque<String>, String, i32> = general::Map::new();
            let mut identificators_value:general::Map<String, Vec<&str>, i32> = general::Map::new();
            let mut identificators_for_map:general::Map<String, Vec<&str>, i32> = general::Map::new();
            let mut values_map:general::Map<String, Vec<&str>, i32> = general::Map::new();
            let mut index = 0;
             remove_duplis(&mut i, &["=".to_string(), ":".to_string(), "_".to_string()].to_vec());
                 loop{
                  let mut without_eq = false;
                  let mut without_class = false;
                  let mut without_map_buffer = false;                
                  {
                      match i.0.get_value(&"=".to_string(), index){
                        None => {without_eq = true;},
                          Some(s) => {
                          if s.contains(","){
                             identificators_value.insert_ref(&"=".to_string(), (s.split(",").collect::<Vec<_>>().clone()));
                          }else{identificators_value.insert_ref(&"=".to_string(), [s.as_str()].to_vec().clone());}
                  
                        }
                      };
                  
                  }
                  {
                   
                   
                  match i.0.get_value(&":".to_string(), index){
                    None =>{without_class = true;},
                    Some(s2) =>{
                      if s2.contains(","){
                    identificators_value.insert_ref(&":".to_string(), (s2.split(",").collect::<Vec<_>>()));
                      }else{identificators_value.insert_ref(&":".to_string(), [s2.as_str()].to_vec());}
                     
                    }
                  };
                 
                }
                 
                 {

                  match i.0.get_value(&"_".to_string(), index){
                    None => {without_map_buffer = true;},
                    Some(mut s3)=>{
                      if s3.contains(","){
                    identificators_for_map.insert_ref(&"_".to_string(), (s3.split(",").collect::<Vec<_>>()));
                      }else{identificators_for_map.insert_ref(&"_".to_string(), [s3.as_str()].to_vec());}
                     
                    }
                   };
                  }
                  index += 1;
                  if without_map_buffer && without_class && without_eq{break;} 
                 
                 
                
                }
                
                let mut values:general::Map<String, Vec<&str>, i32> = general::Map::new();
                index = 0;
                loop{
                  let mut without_map_buffer =false;
                  let mut without_class = false;
                  let mut without_eq = false;
                  match i.2.get_value(&"=".to_string(), index){
                    None => {without_eq = true;},
                    Some(s)=>{
                      if s.contains(","){
                      values.insert_ref(&"=".to_string(), (s.split(",").collect::<Vec<_>>()));
                      }else{values.insert_ref(&"=".to_string(), [s.as_str()].to_vec());}
                    }
                  };
                  match i.2.get_value(&":".to_string(), index){
                    None => {without_class = true;},
                    Some(s)=>{
                      if s.contains(","){
                      values.insert_ref(&":".to_string(), (s.split(",").collect::<Vec<_>>()));
                      }else{values.insert_ref(&":".to_string(), [s.as_str()].to_vec());}
                    }
                  };
                  match i.2.get_value(&"_".to_string(), index){
                    None => {without_map_buffer = true;},
                    Some(s)=>{
                      if s.contains(","){
                      values_map.insert_ref(&"_".to_string(), (s.split(",").collect::<Vec<_>>()));
                      }else{values_map.insert_ref(&"_".to_string(), [s.as_str()].to_vec());}
                    }
                  };
                  index += 1;
                  if without_map_buffer && without_class && without_eq{break;}
                }
                loop{
                  let mut without_eq = false;
                  let mut without_class = false;
                  let mut without_map_buffer = false;
                  match identificators_value.get_ref(&"=".to_string()){
                    None=>{without_eq = true;},
                    Some(s)=>{
                      match values.get_ref(&"=".to_string()){
                        None=>{},
                        Some(s2)=>{
                          for (i, l) in s.iter().enumerate(){
                            for i2 in s2{

                              if let Some(r) = map_eq.get_ref_to_all(&l.trim().to_string()){
                                if !r.contains(&i2.trim().to_string()){map_eq.insert(&l.trim().to_string(), &i2.trim().to_string());}
                              }else{
                                map_eq.insert(&l.trim().to_string(), &i2.trim().to_string());
                              }
                            }
                            
                          }
                        }
                      };
                      identificators_value.remove_ref(&"=".to_string());
                      values.remove_ref(&"=".to_string());
                    }
                  };
                  match identificators_value.get_ref(&":".to_string()){
                    None=>{without_class = true;},
                    Some(s)=>{
                      match values.get_ref(&":".to_string()){
                        None=>{},
                        Some(s2) => {
                          for (i, l) in s.iter().enumerate(){
                            for i2 in s2{
                              if let Some(r) = map.get_ref_to_all(&l.trim().to_string()){
                                if !r.contains(&i2.trim().to_string()){map.insert(&l.trim().to_string(), &i2.trim().to_string());}
                              }else{
                                map.insert(&l.trim().to_string(), &i2.trim().to_string());
                              }
                            }
                          }
                        }
                      };
                      identificators_value.remove_ref(&":".to_string());
                      values.remove_ref(&":".to_string());
                    }
                  };
                  match identificators_for_map.get_ref(&"_".to_string()){
                    
                    None=>{without_map_buffer = true;},
                    Some(s)=>{
                      let mut vec = VecDeque::new();
                      for i in s{
                        vec.push_back(i.to_string());
                      }
                      match values_map.get_ref(&"_".to_string()){
                        None=>{},
                        Some(s2)=>{
                          for  l in s2{
                            if let Some(r) = map_for_buffer.get_ref_to_all(&vec){
                                if !r.contains(&l.trim().to_string()){map_for_buffer.insert(&vec, &l.trim().to_string());}
                              }else{
                                map_for_buffer.insert(&vec, &l.trim().to_string());
                              }
                            
                          }
                        }
                      };
                      identificators_for_map.remove_ref(&"_".to_string());
                      values_map.remove_ref(&"_".to_string());
                    }
                  };
                   if without_map_buffer && without_class && without_eq{break;}
                }
                let mut vec = VecDeque::new();
                vec.push_back(map);
                vec.push_back(map_eq);
                let mut vec_map = VecDeque::new();
                vec_map.push_back(map_for_buffer);
                return Some((vec, vec_map));
               }
               else{return None;}
              }
            }
    }
//------------------------------------------------------------------------------------
use std::rc::Rc;
   /// # `remove_duplis`
   /// Remove content duplicated in vectores, priorized some Vec of String about other Vec of other String.
   /// # Arguments 
   /// * `vecs: &mut (Map<String, String, i32>, Map<String, usize, i32>, Map<String, String, i32>)` - Tuple of Maps where:
   ///     * `Map<String, String, i32>` - Is the Map of the content after some delimiter
   ///     * `Map<String, usize, i32>` - Is the Map of the line when found some delimiter
   ///     * `Map<String, String, i32>` - Is the Map of the content after some delimiter
   /// - This pattern is the same return for the function [`extract_str_before`], because the function expected you use this when before use the funciton [`extract_str_before`]
   /// * `prirority:&Vec<String>` - Vec of Strings, where define the priority of a String, where the priority are decided by the index when appears the String in the vector.
   pub fn remove_duplis(vecs: &mut (general::Map<String, String, i32>,general::Map<String, usize, i32>, general::Map<String, String, i32>), priority: &Vec<String>){
      let mut index = 0;
      loop{      
        if index == priority.len()-1 {break;}
         if let Some(vec) = vecs.1.clone().get_ref_to_all(&priority[index]){
          
           for (n, i) in vec.iter().enumerate(){
            if index == priority.len() {index += 1; break;}
            for (s, l) in priority.iter().enumerate(){
              let mut remove:VecDeque<usize> = VecDeque::new();
              if s >= index+1{
                if let Some(next) = vecs.1.clone().get_mut_ref_to_all(l){
                  if next.contains(i){
                  for (s2, l2) in next.iter().enumerate(){
                    if l2 == i{remove.push_back(s2);}
                  }
                  let mut dcr = 0;
                  for el in remove.iter(){
                     vecs.0.remove_value(l, el-dcr);
                     vecs.1.remove_value(l, el-dcr);
                     vecs.2.remove_value(l, el-dcr);
                    dcr+=1;
                  }
                }
               }
              }
            }
           }
           index += 1;
         }
      }
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
      /// * `Some((Map<String,VecDeque<String>, i32>, Map<String, VecDeque<usize>, i32>, Map<String, VecDeque<String>, i32> ))` - tuple of vectors:
      ///   * `Map<String, String, i32>` - Map of content before extracting, with some delimiter as a key
      ///   * `Map<String, usize, i32>` - Map of number of line where found that delimiter and extract that string, with some delimiter as a key.
      ///   * `Map<String, String, i32>` - Map of content after,with some delimiter as a key
      /// # Example 
      /// ```rust
      /// mod main_code;
      /// fn main(){
      /// use crate::main_code::parsing_sintax;
      /// let content = "Hello, this is\nHello, this is\nChao";
      /// let scape:Vec<char> = Vec::new();
      /// let indexes = parsing_sintax::extract_str_before(content, &["t"].to_vec(), &scape, (&[].to_vec(), &[].to_vec())).unwrap();
      /// println!("{:#?}", indexes);
      /// }
      /// ```
    pub fn extract_str_before(content: &str, delimiter_slice:&Vec<&str>, scape_characters: &Vec<char>, ignore_content_between:(&Vec<char>, &Vec<&str>)) -> 
    Option<(general::Map<String,String, i32>, general::Map<String, usize, i32>, general::Map<String, String, i32>)>{
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
        let mut content_before:general::Map<String,String, i32> = general::Map::new();
        let mut content_after:general::Map<String, String, i32>  = general::Map::new();
        let mut in_ignore = false;
        let mut delimiter_ignore = String::new();

        let mut num_line: general::Map<usize, usize, i32> = general::Map::new();
        let mut indexes:general::Map<String, usize, i32> = general::Map::new();
        let mut map:general::Map<usize, String, i32> = general::Map::new();
        let mut map_after:general::Map<usize, String, i32> = general::Map::new();
        let mut multi_line = false;
        let mut last = 0;
        let mut last_line = 0;
        for line in content.lines(){
            counter += 1;
            contains = false;
         if !multi_line{
          last_line = counter;
        }   
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
            if !in_ignore && !multi_line{
             contains = remove_comments::contains_ignore(ignore_content_between.0, ignore_content_between.1, &copy);
          }
          if !in_ignore && !multi_line{
            for (i, delimiter) in delimiter_slice.iter().enumerate(){
             if copy.contains(delimiter){
              last = i;
                if contains{
                  let verify = remove_comments::content_between(ignore_content_between.0, ignore_content_between.1, scape_characters, delimiter, &copy);
                  in_ignore = verify.1;
                  delimiter_ignore = verify.0;
                  if verify.2.len() != copy.len(){
                    map.insert(&i, &line[..verify.2.len()].to_string());
                    num_line.insert(&i, &counter);
                    
                    let mut stop = remove_comments::content_between(ignore_content_between.0, ignore_content_between.1, scape_characters, ";", &(general::str_of_n_str(" ", verify.2.len()+delimiter.len())+&line[verify.2.len()+delimiter.len()..]));
                    if stop.2.len() != copy.len(){
                      map_after.insert(&last, &line[verify.2.len()+delimiter.len()..stop.2.len()].to_string());
                    }
                    else{
                      stop = remove_comments::content_between(ignore_content_between.0, ignore_content_between.1, scape_characters, "...", &(general::str_of_n_str(" ", verify.2.len()+delimiter.len())+&line[verify.2.len()+delimiter.len()..]));
                      if stop.2.len() != copy.len(){
                      map_after.insert(&last, &line[verify.2.len()+delimiter.len()..stop.2.len()].to_string());
                      multi_line = true;
                    }else{
                      map_after.insert(&i, &line[verify.2.len()+delimiter.len()..].to_string());
                    }
                    }
                    
                  }
                }else{
                  if let Some(pos) = copy.find(delimiter){
                    map.insert(&i,& line[..pos].to_string());
                    num_line.insert(&i, &counter);
                    let mut stop = remove_comments::content_between(ignore_content_between.0, ignore_content_between.1, scape_characters, ";", &(general::str_of_n_str(" ", line[..pos+delimiter.len()].len())+&line[pos+delimiter.len()..]));
                    if stop.2.len() != line.len(){
                      map_after.insert(&last, &line[pos+delimiter.len()..stop.2.len()].to_string());
                    }
                    else{
                      stop = remove_comments::content_between(ignore_content_between.0, ignore_content_between.1, scape_characters, "...", &(general::str_of_n_str(" ", line[..pos+delimiter.len()].len())+&line[pos+delimiter.len()..]));
                      if stop.2.len() != line.len(){
                      map_after.insert(&last, &line[pos+delimiter.len()..stop.2.len()].to_string());
                      multi_line = true;
                    }else{
                      map_after.insert(&i, &line[pos+delimiter.len()..].to_string());
                    }
                    }
                    
                  }
                }
              }
            }
          }else if multi_line{
          let mut stop = remove_comments::content_between(ignore_content_between.0, ignore_content_between.1, scape_characters, ";", &copy);
            if stop.2.len() != copy.len(){
              if let Some(n) = map_after.get_mut_ref_to_all(&last){
                let n2 = n.back_mut().unwrap();
                n2.push_str(&line[..stop.2.len()]);
                
              }
              else{map_after.insert(&last, &line[..stop.2.len()].to_string());}
              multi_line = false;
            }
            else {
                stop = remove_comments::content_between(ignore_content_between.0, ignore_content_between.1, scape_characters, "...", &copy);
                
                if stop.2.len() != copy.len(){
                  if let Some(n) = map_after.get_mut_ref_to_all(&last){
                let n2 = n.back_mut().unwrap();
                n2.push_str(&line[..stop.2.len()]);
                
                }
                    else{map_after.insert(&last, &line[..stop.2.len()].to_string());}
                 
               }else{
                if let Some(n) = map_after.get_mut_ref_to_all(&last){
                let n2 = n.back_mut().unwrap();
                n2.push_str(&line[..stop.2.len()]);
                
                }
                else{map_after.insert(&last, &line[..stop.2.len()].to_string());}
                  multi_line = false;
               }
            }
          }
        }
        {
          
          for (i, s) in delimiter_slice.iter().enumerate(){         

            //Agrupe strings before and the num of line where it found
            loop{
               match map.get(&i){
                Some(n) => {content_before.insert(&s.to_string(),&n); map.remove(&i);},
                None => {break;},
               };
              }
            loop{  
              match num_line.get(&i){
                Some(n) => {indexes.insert(&s.to_string(),&n); num_line.remove(&i);},
                None => {break;}
              };
            }
            loop{
              match map_after.get(&i){
                Some(n) => {content_after.insert(&s.to_string(),&n); map_after.remove(&i);},
                None => {break;}
              }

            }
            }
            
          }
        println!("CONTENT BEFORE DELIMITERS EXTRACTED");
         
         return Some((content_before, indexes, content_after));
    }
//------------------------------------------------------------------------------------
    pub fn parser_equialites(){}
 }