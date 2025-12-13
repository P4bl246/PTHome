/// # Module `general`
/// Provides general utility functions used in the source code of the PTHome application.
/// Includes functions to manage lines, strings, data structures and vectors.
pub mod general{
  #![allow(unused)]
    use std::fs;
    use std::io::Write;
    /// # `remove_empty_lines`
    /// Removes empty lines from the content.
    /// 
    /// # Arguments
    /// * `content: &str` - The content from which empty lines will be removed.
    /// 
    /// # Example
    /// ```
    /// 
    /// use PTHome::main_code::utilities::general;
    ///  let content = "This is a test.\n\nThis is another test.\n\n\nEnd of test.";
    ///  let result = general::remove_empty_lines(content);
    ///  let expected = "This is a test.\nThis is another test.\nEnd of test.\n";
    ///  assert_eq!(result, expected);
    /// 
    /// ```
    /// 
    /// # Returns
    /// A copy of the content without empty lines.
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
    /// # `NumLines`
    /// A struct to hold the configuration for the `NumLines` instance and its methods.
     pub struct NumLines<'a> {
          content: &'a str,
          delimiter: &'a str,
    }
    
    /// # `impl NumLines`
    /// This implementation provides methods to handle line numbers in a string.
    /// Includes functions to add and remove line numbers, skip line numbers, and get the current line number.
    /// Setters and getters:
    /// * [`crate::main_code::utilities::general::NumLines::get_content`]
    /// * [`crate::main_code::utilities::general::NumLines::get_delimiter`]
    /// * [`crate::main_code::utilities::general::NumLines::set_content`]
    /// * [`crate::main_code::utilities::general::NumLines::set_delimiter`]
    impl<'a> NumLines<'a>{
    /// # `new`
    /// Creates a new instance of `NumLines`.
    /// 
    /// # Arguments
    /// * `content: &'a str` - The string to be processed.
    /// * `delimiter: &'a str` - The delimiter to be used for line numbering.
    /// 
    /// # Example
    /// ```rust
    /// 
    /// use PTHome::main_code::utilities::general;
    ///  let instance = general::NumLines::new("example\n   \nsdf", " - ");
    ///  assert_eq!(instance.get_content(), "example\n   \nsdf");
    /// 
    /// ```
    /// 
    /// # Returns
    /// A new instance of `NumLines` with the specified input and delimiter.
    /// 
    /// # IMPORTANT
    /// If don't want use a delimiter just use an empty string `""`.
    /// 
    /// **NOTE:** The default delimiter is an space `' '`.
    pub fn new(content: &'a str, delimiter: &'a str) -> Self{
      Self {
        content:content,
        delimiter:delimiter,
      }
    }
 //---------------------------------------------------------------------
       /// # `numerate_lines`
       /// Numerates the lines of the `content` and returns a String with the `content` numerated.
       /// 
       /// # Example
       /// ```rust
       /// 
       /// use PTHome::main_code::utilities::general;
       /// let mut instance = general::NumLines::new("example.txt", " - ");
       /// let numerate = instance.numerate_lines();
       /// //Upload content just as here
       /// instance.set_content(&numerate);
       /// assert_eq!(instance.get_content(), "1 - example.txt\n");
       /// 
       /// ```
       /// 
       /// # Returns
       /// A string with the `content` numerated.
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
        /// Removes line numbers from the `content`. It is recommended to use this only if you used the method `numerate_lines` and update the `content`.
        /// 
        /// # Example
        /// ```rust
        /// 
        /// use PTHome::main_code::utilities::general;
        ///  let mut instance = general::NumLines::new("example.txt", " - ");
        ///  let mut rmv_num = instance.numerate_lines();
        ///  //Upload content just as here
        ///  instance.set_content(&rmv_num);
        ///  let removed = instance.remove_num_lines();
        ///  assert_eq!(removed, "example.txt\n");
        /// 
        /// ```
        /// 
        /// # Returns
        /// * A string with the `content` cleaned of line numbers.
        /// * The same content if the `content` hasn't any delimiter.
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
        /// Skips the line number in the string. It is recommended to use this only if you used the method `numerate_lines`.
        /// 
        /// # Arguments
        /// * `line: &str` - The line from which the line number will be skipped.
        /// 
        /// # Example
        /// ```rust
        /// 
        /// use PTHome::main_code::utilities::general;
        /// let ins = general::NumLines::new("example.txt", " - ");
        /// let all_after_num_line = ins.skip_num_line("1 - This is a test line.");
        /// assert_eq!(all_after_num_line, "This is a test line.");
        /// 
        /// ```
        /// 
        /// # Returns
        /// * A `String` without the line number.
        /// * The same line if the delimiter is not found.
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
        /// Gets the current line number from a string.
        /// 
        /// # Arguments
        /// * `line: &str` - The line from which the current line number will be extracted.
        /// 
        /// # Example
        /// ```rust
        /// 
        /// use PTHome::main_code::utilities::general;
        /// let ins = general::NumLines::new("example.txt", " - ");
        /// let current_line = ins.get_num_line("1 - This is a test line.");
        /// assert_eq!(current_line, 1);
        /// 
        /// ```
        /// 
        /// # Returns
        /// * The current line number as a `i32`.
        /// * If the delimiter is not found returns -1.
        /// 
        /// # Errors
        /// * If the line number cannot be convert, the function will returns panic with an error message.
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
       /// # `get_content`
       /// Gets the content.
       /// 
       /// # Returns
       /// The content as a `String`.
       pub fn get_content(&self) -> String{
            self.content.to_string()
        }
 //---------------------------------------------------------------------
       /// # `get_delimiter`
       /// Gets the delimiter used for line numbering.
       /// 
       /// # Returns
       /// The delimiter as a `String`.
       pub fn get_delimiter(&self) -> String{
            self.delimiter.to_string()
        }
 //---------------------------------------------------------------------
       /// # `set_content`
       /// Sets the content stored.
       /// 
       /// # Arguments
       /// * `new_value: &'a str` - The new content.
       /// 
       /// # Example
       /// ```rust
       /// 
       /// use PTHome::main_code::utilities::general;
       /// let mut ins = general::NumLines::new("example.txt", " - ");
       /// ins.set_content("new_example.txt");
       /// assert_eq!(ins.get_content(), "new_example.txt");
       /// 
       /// ```
       pub fn set_content(&mut self, new_value:&'a str){
            self.content = new_value;
        }
 //---------------------------------------------------------------------
       /// # `set_delimiter`
       /// Sets the delimiter.
       /// 
       /// ## Arguments 
       /// * `delimiter: &'a str` - The new delimiter.
       /// 
       /// # Example  
       /// ```rust
       /// 
       /// use PTHome::main_code::utilities::general;
       /// let mut ins = general::NumLines::new("example.txt", " - ");
       /// ins.set_delimiter(" | ");
       /// assert_eq!(ins.get_delimiter(), " | ");
       /// 
       /// ```
       pub fn set_delimiter(&mut self, new_value:&'a str){
            self.delimiter = new_value;
       }
       
    }
 //------------------------------------------------------------------------------------------
    /// # `all_appears_index`
    /// Finds all the appearances of a substring in a string and returns their indexes.
    /// 
    /// # Arguments
    /// * `input_str: &str` - Where search the substring.
    /// * `search_str: &str` - The substring to search for.
    /// 
    /// # Example
    /// ```rust 
    /// 
    /// use PTHome::main_code::utilities::general;
    /// let input_str = "This is a test string. This is another test string.";
    /// let search_str = "test";
    /// let indexes = general::all_appears_index(input_str, search_str);
    /// assert_eq!(indexes, vec![10, 39]);
    /// 
    /// ```
    /// 
    /// # Returns
    /// * A vector of `usize` containing the indexes of all occurrences of the substring in the string.
    /// * If the substring or the string is empty, it returns an empty vector.
    /// 
    /// # Errors
    /// If the input string or the search string was empty, the function will returns an empty vector.
  pub fn all_appears_index(input_str:&str, search_str: &str) -> Vec<usize>{
    let mut indexes = Vec::new();
    let mut copy = input_str.to_string();
    let mut remove = 0;
    if search_str.is_empty() || input_str.is_empty() {
        return indexes;
    }
    else{
      
         while let Some(index) = copy.find(search_str){
          indexes.push(index + remove);
          //remove this appear
          remove += copy[..index+search_str.len()].len();
          copy = copy[index+search_str.len()..].to_string();

         }
       
    }
    return indexes;
     
  }
 //------------------------------------------------------------------------------------------
  /// # `sub_vec`
  /// Returns a sub-vector from a given vector starting at a specified index and taking a specified number of elements.
  /// 
  /// # Arguments
  /// * `vec: &Vec<T>` - The vector from which to extract the sub-vector.
  /// * `num_elements: usize` - The number of elements to take from the vector.
  /// * `start_index: usize` - The index at which to start taking elements from the vector.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let vec = vec![1, 2, 3, 4, 5];
  /// let sub_vec = general::sub_vec(&vec, 3, 1);
  /// assert_eq!(sub_vec, vec![2, 3, 4]); 
  /// 
  /// ```
  /// 
  /// # Returns
  /// A new vector containing the specified number of elements starting from the specified index.
  /// 
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
  /// # `str_of_n_str`
  /// Creates a string of n length from other string.
  /// 
  /// # Arguments
  /// * `str_use:&str` - String which will be used for make the new string.
  /// * `len_new_str:usize` - The length of the new string.
  /// 
  /// # Return
  /// A string with n length, all his content are the `str_use`.
  /// 
  /// # Example 
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let str = general::str_of_n_str("S", 5);
  /// assert_eq!(str, "SSSSS");
  /// 
  /// ```
  ///  
  /// # Returns 
  /// A string with n length, where all its content is made from the `str_use`.
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
   /// Replaces a character in a string.
   /// 
   /// # Arguments
   /// * `str_in:&str` - String which will be replaced the character.
   /// * `repalce:&str` - String for replace.
   /// * `index:usize` -Index to replace.
   /// 
   /// # Example 
   /// ```rust
   /// 
   /// use PTHome::main_code::utilities::general;
   /// let main_str = "Hello. This is my show?";
   /// let mut str = general::replace_index(main_str, "!", main_str.find("?").unwrap());
   /// str = general::replace_index(&str, "the hour of the ", str.find("my").unwrap()); // 'y' is part of "my"; when we replace 'm', the 'y' remains.
   /// assert_eq!(str, "Hello. This is the hour of the y show!");
   /// 
   /// ```
   ///  
   /// # Returns
   /// * The input string if `str_in` or `replace` is empty, or index is greather than the size of the input string.
   /// * A String with the index replaced.
  pub fn replace_index(str_in: &str, replace: &str, index: usize)-> String{
    if str_in.is_empty(){
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
  //// # `ordered_combination`
  /// Creates an ordered combination of two vectors, where each element of the first vector merges with each element of the second vector.
  /// 
  /// # Arguments
  /// * `vecs: (&Vec<String>, &Vec<String>)` - Tuple with two vectors of strings.
  /// - `Vec<String>` - First vector for the combination.
  /// - `Vec<String>` - Second vector for the combination.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let vec1 = vec!["A".to_string(), "B".to_string()];
  /// let vec2 = vec!["1".to_string(), "2".to_string()];
  /// let combined = general::ordered_combination((&vec1, &vec2));
  /// assert_eq!(vec!["A1".to_string(), "A2".to_string(), "B1".to_string(), "B2".to_string()],combined);
  /// 
  /// ```
  /// 
  /// # Returns
  /// A vector of strings with the ordered combination made from the two input vectors.
  pub fn ordered_combination(vecs:(&Vec<String>, &Vec<String>))->Vec<String>{
    let mut sub_vec:Vec<String> = Vec::new();
    if vecs.0.is_empty(){
      return vecs.1.clone();
    }else if vecs.1.is_empty(){
        return vecs.0.clone();
    }
    for i in vecs.0{
      for i2 in vecs.1{
        sub_vec.push(i.clone()+i2);
      }
    }
    return sub_vec;
  }
 //-------------------------------------------------------------------------------------------------
use std::collections::HashMap;
use std::hash::Hash;
use std::collections::VecDeque;
use std::collections::hash_map::{Keys, IterMut, Iter};
use std::rc::Rc;
/// # `Map<T, U, F>`
/// The Map supports multiples values for a same key, with the similar temporary complexity of the HashMaps (O(1) aprox).
/// 
/// ## Note  
/// We call `queue` a vector which stored values for a key, enabling support multiples values for the same key. 
/// The values will be extracted in FIFO order (properties from a `queue`).
 #[derive(Debug)]
 #[derive(Clone)]
pub struct Map<T, U, F>
where 
  T: Clone+ Eq + Hash, 
  U: Clone + PartialEq
  {
  hash: HashMap<T, VecDeque<U>>,
  hash_ref: HashMap<T, VecDeque<Rc<U>>>,
  hash_something: HashMap<T, VecDeque<F>>,
  order: HashMap<T, VecDeque<usize>>, 
  order_o1: HashMap<usize, T>,
  order_hash : bool,
  store_last_insert_key:bool,
  order_hash_ref: bool,
  store_last_insert_ref_key:bool,
  order_hash_something:bool,
  store_last_insert_something_key:bool,
  preserve_before: bool,
  iter: usize,
  counter: usize
  }
 impl<T, U, F> Map<T, U, F>
 where 
  T: Clone+ Eq +Hash, 
  U: Clone + PartialEq
 {
  /// # `new`
  /// Creates a new instance of the Map struct.
  pub fn new() -> Self{
    Self{
      hash: HashMap::new(),
      hash_ref: HashMap::new(),
      hash_something: HashMap::new(),
      order: HashMap::new(),
      order_o1: HashMap::new(),
      order_hash : false, 
      store_last_insert_key:false,
      order_hash_ref:false,
      store_last_insert_ref_key:false,
      store_last_insert_something_key:false,
      order_hash_something:false,
      preserve_before:false,
      iter: 0,
      counter: 0
    }
  }
  
  //--------------------------------------------
  /// # `insert`
  /// Inserts a new element into the HashMap of copies, if the key exists the element to be inserted goes to the end of the queue. 
  /// 
  /// # Arguments
  /// * `key: &T` - Key into which the element will be insert.
  /// * `element: &U` - Element to insert.
  /// 
  /// # Example 
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key1".to_string(), &"value2".to_string());
  /// assert_eq!(map.get(&"key1".to_string()), Some(&"value1".to_string()));
  /// 
  /// ```
  /// 
  /// # IMPORTANT ONLY IF YOU ENABLED THE INSERTION HISTORY 
  /// The order is in the same vector, so if you select put all the inserts of the key in the order and after change for just last insert for key, all elements stored will be removed and just stored the last insertion for each key,
  /// if you want to avoid this behavior enables the 'preserve_before' flag.
  /// The order does not distinguish between insertions in hash of clones, insertions in hash of refs or insertions in hash of something, all that matters is the key.
  pub fn insert(&mut self, key: &T, element: &U){
    if let Some(vec) = self.hash.get_mut(key){
      vec.push_back(element.clone());
    }
    else{
        let mut vec_new = VecDeque::new();
      vec_new.push_back(element.clone());
      self.hash.insert(key.clone(),vec_new);
      }
      if self.order_hash{
      if self.store_last_insert_key{
        if self.preserve_before{
          if let Some(vec) = self.order.get_mut(key){
             vec.push_back(self.counter);
           }
          if let Some(vec) = self.order.get_mut(key){
            let last = vec.back();
            if self.order_o1.contains_key(last.unwrap()){
              self.order_o1.remove(last.unwrap());
            }
            let n = vec.back_mut().unwrap();
            *n = self.counter;
            
          }
          else{
            let mut new_vec = VecDeque::new();
          new_vec.push_front(self.counter);
          self.order.insert(key.clone(), new_vec);            
          }
          self.order_o1.insert(self.counter, key.clone());

        }
        else{
          if let Some(vec) = self.order.get_mut(key){
            for i in vec{
              if self.order_o1.contains_key(i){
                self.order_o1.remove(i);
              }
            }
          }
          let mut new_vec = VecDeque::new();
          new_vec.push_front(self.counter);
          self.order.insert(key.clone(), new_vec); 
          self.order_o1.insert(self.counter, key.clone());
          }
      
    }else{
       if let Some(vec) = self.order.get_mut(key){
        vec.push_back(self.counter);
       }
        else{
          let mut new_vec = VecDeque::new();
          new_vec.push_front(self.counter);
          self.order.insert(key.clone(), new_vec);
        }
        self.order_o1.insert(self.counter, key.clone());
      }
       self.counter+=1;    
   }
  }

  /// # `insert_ref`
  /// Inserts a new element into the HashMap of references, if the key exists the element to be inserted goes to the end of the queue. 
  /// 
  /// # Arguments
  /// * `key: &T` - Key into which the element will be insert.
  /// * `element: &U` - Element to insert.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key1".to_string(), "value2".to_string());
  /// assert_eq!(map.get_ref(&"key1".to_string()), Some(&"value1".to_string()));
  /// 
  /// ```
  /// 
  /// # IMPORTANT ONLY IF YOU ENABLED THE INSERTION HISTORY 
  /// The order is in the same vector, so if you select put all the inserts of the key in the order and after change for just last insert for key, all elements stored will be removed and just stored the last insertion for each key,
  /// if you want to avoid this behavior enables the 'preserve_before' flag.
  /// The order does not distinguish between insertions in hash of clones, insertions in hash of refs or insertions in hash of something, all that matters is the key.
  pub fn insert_ref(&mut self, key: &T, element: U){
    if let Some(vec) = self.hash_ref.get_mut(key){
      vec.push_back(Rc::new(element));
    }
    else{
    let mut vec_new = VecDeque::new();
    let ref_element = Rc::new(element);
      vec_new.push_back(ref_element.clone());
      self.hash_ref.insert(key.clone(),vec_new);
      }
    if self.order_hash_ref{
      if self.store_last_insert_ref_key{
        if self.preserve_before{
          if let Some(vec) = self.order.get_mut(key){
            vec.push_back(self.counter);
           }
          if let Some(vec) = self.order.get_mut(key){
            let last = vec.back();
            if self.order_o1.contains_key(last.unwrap()){
              self.order_o1.remove(last.unwrap());
            }
            let n = vec.back_mut().unwrap();
            *n = self.counter;
            
          }
          else{
            let mut new_vec = VecDeque::new();
          new_vec.push_front(self.counter);
          self.order.insert(key.clone(), new_vec);            
          }
          self.order_o1.insert(self.counter, key.clone());

        }
        else{
          if let Some(vec) = self.order.get_mut(key){
            for i in vec{
              if self.order_o1.contains_key(i){
                self.order_o1.remove(i);
              }
            }
          }
          let mut new_vec = VecDeque::new();
          new_vec.push_front(self.counter);
          self.order.insert(key.clone(), new_vec); 
          self.order_o1.insert(self.counter, key.clone());
          }
      
    }else{
       if let Some(vec) = self.order.get_mut(key){
        vec.push_back(self.counter);
       }
        else{
          let mut new_vec = VecDeque::new();
          new_vec.push_front(self.counter);
          self.order.insert(key.clone(), new_vec);
        }
        self.order_o1.insert(self.counter, key.clone());
      }
       self.counter+=1;    
   }
  }

  /// # `insert_something`
  /// Inserts a new element into the HashMap of some types of elements, if the key exists the element to be inserted goes to the end of the queue. 
  /// 
  /// # Arguments
  /// * `key: &T` - Key to which the element will be insert.
  /// * `element: F` - Element to insert.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key1".to_string(), 20);
  /// assert_eq!(map.get_something(&"key1".to_string()), Some(&10));
  /// 
  /// ```
  /// 
  /// # IMPORTANT ONLY IF YOU ENABLED THE INSERTION HISTORY
  /// The order is in the same vector, so if you select put all the inserts of the key in the order and after change for just last insert for key, all elements stored will be removed and just stored the last insert for each key,
  /// if you want to avoid this behavior enables the 'preserve_before' flag.
  /// The order does not distinguish between insertions in hash of clones, insertions in hash of refs or insertions in hash of something, all that matters is the key.
  pub fn insert_something(&mut self, key:&T, element:F){
      match self.hash_something.get_mut(key){
        None=>{
          let mut new_vec = VecDeque::new();
          new_vec.push_back(element);
          self.hash_something.insert(key.clone(), new_vec);
        }
        Some(i)=>{
          i.push_back(element);
        }
      };
      if self.order_hash_something{
      if self.store_last_insert_something_key{
        if self.preserve_before{
          if let Some(vec) = self.order.get_mut(key){
            vec.push_back(self.counter);
           }
          if let Some(vec) = self.order.get_mut(key){
            let last = vec.back();
            if self.order_o1.contains_key(last.unwrap()){
              self.order_o1.remove(last.unwrap());
            }
            let n = vec.back_mut().unwrap();
            *n = self.counter;
            
          }
          else{
            let mut new_vec = VecDeque::new();
          new_vec.push_front(self.counter);
          self.order.insert(key.clone(), new_vec);            
          }
          self.order_o1.insert(self.counter, key.clone());

        }
        else{
          if let Some(vec) = self.order.get_mut(key){
            for i in vec{
              if self.order_o1.contains_key(i){
                self.order_o1.remove(i);
              }
            }
          }
          let mut new_vec = VecDeque::new();
          new_vec.push_front(self.counter);
          self.order.insert(key.clone(), new_vec); 
          self.order_o1.insert(self.counter, key.clone());
          }
      
    }else{
       if let Some(vec) = self.order.get_mut(key){
        vec.push_back(self.counter);
       }
        else{
          let mut new_vec = VecDeque::new();
          new_vec.push_front(self.counter);
          self.order.insert(key.clone(), new_vec);
        }
        self.order_o1.insert(self.counter, key.clone());
      }
       self.counter+=1;    
   }
  }
  //---------------------------------------------

  //---------------------------------------------
  /// # `get`
  /// Gets the first element from the queue of some key.
  /// 
  /// # Arguments
  /// * `key: &T` - key to get the element.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key1".to_string(), &"value2".to_string());
  /// assert_eq!(map.get(&"key1".to_string()), Some(&"value1".to_string()));
  /// 
  /// ```
  /// 
  /// # Returns    
  /// * `None` - If the key does not exist.
  /// * `&U` - A reference to the element.
  pub fn get(&self, key: &T)-> Option<&U>{
   match &mut self.hash.get(key){
    Some(i) => i.front(),
    None => None
   }
  }
 
  /// # `get_ref`
  /// Gets the first element from the queue of some key in the HashMap of references.
  /// 
  /// # Arguments
  /// * `key: &T` - key to get the element.
  /// 
  /// # Example
  /// ```rust
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key1".to_string(), "value2".to_string());
  /// assert_eq!(map.get_ref(&"key1".to_string()), Some(&"value1".to_string()));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `None` - If the key does not exist.
  /// * `Some(&U)` - A reference to the element.
  pub fn get_ref(&self, key: &T)-> Option<&U>{
   match &mut self.hash_ref.get(key){
    Some(i) => {
      if let Some(val) = i.front(){
        return Some(&**val);
      }else{
        return None;
      }
    },
    None => None
   }
  }

  /// # `get_something`
  /// Gets the frist element from the queue of some key in the HashMap of some types of elements.
  /// 
  /// # Arguments
  /// * `key: &T` - key to get the element.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key1".to_string(), 20);
  /// assert_eq!(map.get_something(&"key1".to_string()), Some(&10)); 
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `None` - If the key does not exist.
  /// * `Some(&F)` - A reference to the element.
  pub fn get_something(&self, key:&T)->Option<&F>{
    if let Some(vec) = self.hash_something.get(key){
      return vec.front();
    }else{
      return None;
    }
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `remove` 
  /// Removes the first element in the HashMap and replaces it with the next element in the queue.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key1".to_string(), &"value2".to_string());
  /// map.remove(&"key1".to_string());
  /// assert_eq!(map.get(&"key1".to_string()), Some(&"value2".to_string()));
  /// 
  /// ```
  /// 
  /// # Arguments  
  /// * `key: &T` - Key to editing its queue.
  pub fn remove(&mut self, key: &T){
    if let Some(replace) = self.hash.get_mut(key){ 
       if replace.len() > 0{replace.pop_front();}
       if replace.len() <= 0 {self.hash.remove(key);} 
    }
  }
  
  /// # `remove_ref` 
  /// Removes the first element in the HashMap of references and replaces it with the next element in the queue.
  /// 
  /// # Example 
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key1".to_string(), "value2".to_string());
  /// map.remove_ref(&"key1".to_string());
  /// assert_eq!(map.get_ref(&"key1".to_string()), Some(&"value2".to_string()));
  /// 
  /// ```
  /// 
  /// # Arguments  
  /// * `key: &T` - Key to editing its queue.
  pub fn remove_ref(&mut self, key: &T){
      if let Some(replace) = self.hash_ref.get_mut(key){ 
       if replace.len() > 0{replace.pop_front();}
       if replace.len() <= 0 {self.hash_ref.remove(key);} 
    }
  }
  
  /// # `remove_something`
  /// Removes the first element in the HashMap of some types of elements and replaces it with the next element in the queue.
  /// 
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new(); 
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key1".to_string(), 20);
  /// map.remove_something(&"key1".to_string());
  /// assert_eq!(map.get_something(&"key1".to_string()), Some(&20));
  /// 
  /// ```
  /// 
  /// # Arguments
  /// * `key: &T` - Key to editing its queue.
  pub fn remove_something(&mut self, key:&T){
    if let Some(vec) = self.hash_something.get_mut(key){
      vec.pop_front();
      if vec.len() <= 0{self.hash_something.remove(key);}
    }
  
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `remove_all`
  /// Removes a key from the HashMap of copies even if that has more elements in its queue.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key1".to_string(), &"value2".to_string());
  /// map.remove_all(&"key1".to_string());
  /// assert_eq!(map.get(&"key1".to_string()), None);
  /// 
  /// ```
  /// 
  /// # Arguments   
  /// * `key: &T` - Key to remove.
  pub fn remove_all(&mut self, key: &T){
    if self.hash.contains_key(key){
      self.hash.remove(key);
    }
  }
  
  /// # `remove_all_ref`
  /// Removes a key from the HashMap of references even if that has more elements in its queue.
  /// 
  /// # Example 
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key1".to_string(), "value2".to_string());
  /// map.remove_all_ref(&"key1".to_string());
  /// assert_eq!(map.get_ref(&"key1".to_string()), None);
  /// 
  /// ```
  /// 
  /// # Arguments   
  /// * `key: &T` - Key to remove.
  pub fn remove_all_ref(&mut self, key: &T){
    if self.hash_ref.contains_key(key){
      self.hash_ref.remove(key);
    }
  }

  /// # `remove_all_something`
  /// Removes a key from the HashMap of some types of elements even if that has more elements in its queue.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key1".to_string(), 20);
  /// map.remove_all_something(&"key1".to_string());
  /// assert_eq!(map.get_something(&"key1".to_string()), None);
  /// 
  /// ```
  /// 
  /// # Arguments
  /// * `key: &T` - Key to remove.
  pub fn remove_all_something(&mut self, key:&T){
    if self.hash_something.contains_key(key){
    self.hash_something.remove(key);
    }
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `get_all`
  /// Gets all the elements in the key queue.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key1".to_string(), &"value2".to_string());
  /// let vec = map.get_all(&"key1".to_string());
  /// assert_eq!(vec, vec!["value1".to_string(), "value2".to_string()]);
  /// 
  /// ```
  /// 
  /// # Arguments
  /// * `key:&T` - key to taking the elements in its queue and putting them into a vector.
  /// 
  /// # Returns
  /// A empty vector if this key does not exist, else a vector with the elements in the key queue.
  pub fn get_all(&mut self, key: &T)-> Vec<U>{
    let mut vec_ret = Vec::new();
    if let Some(i)  = self.hash.get(key){
      for n in i{
          vec_ret.push(n.clone());
      }
     if i.len() <= 0{
      self.hash.remove(key);
     }
    }
    return vec_ret;
  }
  
  /// # `get_all_ref`
  /// Gets all the elements in the key queue (from the HashMap of references).
  /// 
  /// # Arguments
  /// * `key:&T` - key to taking the elements in its queue and putting them into a vector.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key1".to_string(), "value2".to_string());
  /// let vec = map.get_all_ref(&"key1".to_string());
  /// aassert_eq!(vec, vec!["value1".to_string(), "value2".to_string()]);
  /// 
  /// ```
  /// 
  /// # Returns 
  /// A empty vector if the key does not exist, else a vector with the elements in the key queue.
  pub fn get_all_ref(&mut self, key: &T)-> Vec<U>{
    let mut vec_ret = Vec::new();
    
    if let Some(i)  = self.hash_ref.get(key){
      for n in i{
          vec_ret.push((**n).clone());
      }
    }
    if vec_ret.len() <= 0{
        self.hash_ref.remove(key);
      }
    return vec_ret;
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `get_mut_ref_to_all`
  /// Gets a mutable reference to the VecDeque (queue) of the key.
  /// 
  /// # Arguments
  /// ```rust
  /// use std::collections::VecDeque;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key1".to_string(), &"value2".to_string());
  /// let vec_deque = map.get_mut_ref_to_all(&"key1".to_string());
  /// assert_eq!(vec_deque, Some(&mut VecDeque::from(vec!["value1".to_string(), "value2".to_string()])));
  /// 
  /// ```
  /// 
  /// # Arguments
  /// * `key:&T` - Key to getting its VecDeque.
  /// 
  /// # Returns
  /// * `None` - If the key does not exist.
  /// * `Some(&mut VecDeque<U>)` - Mutable reference to the VecDeque.
  /// 
  /// # IMPORTANT
  /// If you use this option, you need to know what is a VecDeque and his propierties, for avoid break the HashMap and the vector.
  pub fn get_mut_ref_to_all(&mut self, key: &T) -> Option<&mut VecDeque<U>>{
    self.hash.get_mut(key)
  }
  
  /// # `get_mut_ref_to_all_ref`
  /// Gets a mutable reference to the VecDeque (queue) of the key.
  /// 
  /// # Arguments
  /// * `key:&T` - Key to getting its VecDeque.
  /// 
  /// # Example 
  /// ```rust
  /// 
  /// use std::rc::Rc;
  /// use std::collections::VecDeque;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key1".to_string(), "value2".to_string());
  /// let vec_deque = map.get_mut_ref_to_all_ref(&"key1".to_string());
  /// assert_eq!(vec_deque, Some(&mut VecDeque::from(vec![Rc::new("value1".to_string()), Rc::new("value2".to_string())])));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `None` - if the key not exist.
  /// * `Some(&mut VecDeque<Rc<U>>) - Mutable reference to VecDeque vector (the values into the VecDeque are Rc<U>).
  /// 
  /// # IMPORTANT
  /// If you use this option, you need to know what is a VecDeque and his propierties and Rc too, for avoid break the HashMap and the vector.
  pub fn get_mut_ref_to_all_ref(&mut self, key: &T) -> Option<&mut VecDeque<Rc<U>>>{
    self.hash_ref.get_mut(key)
  }
  
  /// # `get_mut_ref_to_all_something`
  /// Gets a mutable reference to VecDeque (queue) of the key in the HashMap of random items.
  /// 
  /// # Arguments
  /// * `key: &T` - Key to get the VecDeque.
  /// 
  /// # Example 
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key1".to_string(), 20);
  /// let vec_deque = map.get_mut_ref_to_all_something(&"key1".to_string());
  /// assert_eq!(vec_deque, Some(&mut VecDeque::from(vec![10, 20])));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `None` - If the key not exist.
  /// * `Some(&mut VecDeque<F>)` - Mutable reference to VecDeque vector (the values in the VecDeque are F).
  /// 
  /// # IMPORTANT
  /// If you use this option, you need to know what is a VecDeque and his propierties, for avoid break the HashMap and the vector.
  /// 
  /// ## NOTE
  /// Does not exist a method for get all the key values for the HashMap of some types of elements, because they might not be clonnable.
  pub fn get_mut_ref_to_all_something(&mut self, key:&T)->Option<&mut VecDeque<F>>{
    self.hash_something.get_mut(key)
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `get_ref_to_all`
  /// Gets a reference to the VecDeque for the key.
  /// 
  /// # Arguments
  /// * `key: &T` - Key to get the VecDeque.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key1".to_string(), &"value2".to_string());
  /// let vec_deque = map.get_ref_to_all(&"key1".to_string());
  /// assert_eq!(vec_deque, Some(&VecDeque::from(vec!["value1".to_string(), "value2".to_string()])));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `None` - If the key not exist.
  /// * `Some(&VecDeque<U>)` - A reference to the VecDeque vector.
  /// 
  /// # IMPORTANT
  /// If you use this option, you need to know what is a VecDeque and his propierties.
  /// 
  /// ## NOTE
  /// Not exist a method for get all the key values for the HashMap of some types of elements, because they might not be clonnable.
  pub fn get_ref_to_all(&self, key:&T)->Option<&VecDeque<U>>{
    self.hash.get(key)
  }
  
  /// # `get_ref_to_all_ref`
  /// Gets a reference to VecDeque for that key.
  /// 
  /// # Arguments
  /// * `key: &T` - Key to get the VecDeque.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use std::rc::Rc;
  /// use std::collections::VecDeque;
  /// use PTHome::main_code::utilities::general;
  /// 
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key1".to_string(), "value2".to_string());
  /// let vec_deque = map.get_ref_to_all_ref(&"key1".to_string());
  /// assert_eq!(vec_deque, Some(&VecDeque::from(vec![Rc::new("value1".to_string()), Rc::new("value2".to_string())])));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `None` - If the key not exist.
  /// * `Some(&VecDeque<Rc<U>>)` - A reference to the VecDeque vector (the values in the VecDeque are Rc<U>).
  /// 
  /// # IMPORTANT
  /// If you use this option, you need to know what is a VecDeque and his propierties and Rc too.
  /// 
  /// ## NOTE
  /// Not exist a method for get all the key values for the HashMap of some types of elements, because they might not be clonnable.
  pub fn get_ref_to_all_ref(&self, key: &T) -> Option<&VecDeque<Rc<U>>>{
    self.hash_ref.get(key)
  }
  
  /// # `get_ref_to_all_something`
  /// Gets a reference to VecDeque for that key in the HashMap of random values.
  /// 
  /// # Arguments
  /// * `key: &T` - Key to get the VecDeque.
  ///
  /// # Example
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key1".to_string(), 20);
  /// let vec_deque = map.get_ref_to_all_something(&"key1".to_string());
  /// assert_eq!(vec_deque, Some(&VecDeque::from(vec![10, 20])));
  /// 
  /// ```
  /// 
  /// # Returns 
  /// * `None` - If the key not exist.
  /// * `Some(&VecDeque<F>)` - A reference to the VecDeque vector (the values in the VecDeque are F).
  /// 
  /// # IMPORTANT
  /// If you use this option, you need to know what is a VecDeque and his propierties.
  /// 
  /// ## NOTE
  /// Not exist a method for get all the key values for the HashMap of some types of elements, because they might not be clonnable.
  pub fn get_ref_to_all_something(&mut self, key:&T)->Option<&VecDeque<F>>{
    self.hash_something.get(key)
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `set_value`
  /// Sets all the queue of the key.
  /// 
  /// # Arguments
  /// * `key: &T` - Key for setting its queue.
  /// * `new_vec: Vec<U>` - Vec for replace the queue.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key1".to_string(), &"value2".to_string());
  /// map.set_value(&"key1".to_string(), &vec!["new_value1".to_string(), "new_value2".to_string()]);
  /// assert_eq!(map.get_all(&"key1".to_string()), vec!["new_value1".to_string(), "new_value2".to_string()]);
  /// 
  /// ```
  pub fn set_value(&mut self, key: &T, new_vec: &Vec<U>){
    if let Some(vec) = self.hash.get_mut(key){
      vec.clear();
      for i in new_vec{
        vec.push_back(i.clone());
      }
    }
  }
  
  /// # `set_value_ref`
  /// Sets all the queue of the key in the HashMap of references.
  /// 
  /// # Arguments
  /// * `key: &T` - Key for setting its queue.
  /// * `new_vec: Vec<U>` - Vec for replace the queue.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key1".to_string(), "value2".to_string());
  /// map.set_value_ref(&"key1".to_string(), vec!["new_value1".to_string(), "new_value2".to_string()]);
  /// assert_eq!(map.get_all_ref(&"key1".to_string()), vec!["new_value1".to_string(), "new_value2".to_string()]);
  /// 
  /// ```
  pub fn set_value_ref(&mut self, key: &T, new_vec: Vec<U>){
    if let Some(vec) = self.hash_ref.get_mut(key){
      vec.clear();
      for i in new_vec{
        let rc = Rc::new(i);
        vec.push_back(rc);
      }
    }
  }

  /// # `set_value_something`
  /// Sets all the queue of the key in the HashMap of some types of elements.
  /// 
  /// # Arguments
  /// * `key: &T` - Key for setting its queue.
  /// * `new_vec: Vec<F>` - Vec for replace the queue.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key1".to_string(), 20);
  /// map.set_value_something(&"key1".to_string(), vec![30, 40]);
  /// assert_eq!(map.get_ref_to_all_something(&"key1".to_string()), Some(&VecDeque::from(vec![30, 40])));
  /// 
  /// ```
  pub fn set_value_something(&mut self, key:&T, new_vec:Vec<F>){
    if let Some(vec) = self.hash_something.get_mut(key){
      vec.clear();
      for i in new_vec{
        vec.push_back(i);
      }
    }
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `contains_key`
  /// Indicates if the key exists in the HashMap of copies.
  /// 
  /// # Arguments
  /// * `key:&T` - Key to search.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// assert_eq!(map.contains_key(&"key1".to_string()), true);
  /// 
  /// ```
  /// 
  /// # Returns 
  /// * `true` if exists.
  /// * `false` if does not exist.
  pub fn contains_key(&self, key: &T)-> bool{
    self.hash.contains_key(key)
  }
  
  /// # `contains_key_ref`
  /// Indicates if the key exists in the HashMap of references.
  /// 
  /// # Arguments
  /// * `key:&T` - Key to search.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// assert_eq!(map.contains_key_ref(&"key1".to_string()), true);
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `true` if exists.
  /// * `false` if does not exist.
  pub fn contains_key_ref(&self, key: &T)-> bool{
    self.hash_ref.contains_key(key)
  }

  /// # `contains_key_something`
  /// Indicate if something key exists in the HashMap of random values.
  /// 
  /// # Arguments
  /// * `key:&T` - Key to search.
  /// 
  /// # example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// assert_eq!(map.contains_key_something(&"key1".to_string()), true);
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `true` if exists.
  /// * `false` if does not exist.
  pub fn contains_key_something(&self, key:&T)->bool{
    self.hash_something.contains_key(key)
  }
  //---------------------------------------------

  //---------------------------------------------
  /// # `set_value_element`
  /// Sets the value to some elemenet in the queue of the key (can be 0 or other), if the index is greater than the vector length the change will not be made.
  /// 
  /// # Arguments 
  /// * `key: &T` - Key for change the element in its queue.
  /// * `index: usize` - Index of element to change.
  /// * `new_element` - Element for make the change.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key1".to_string(), &"value2".to_string());
  /// map.set_value_element(&"key1".to_string(), 1, &"new_value".to_string());
  /// assert_eq!(map.get_all(&"key1".to_string()), vec!["value1".to_string(), "new_value".to_string()]);
  /// 
  /// ```
  pub fn set_value_element(&mut self, key: &T, index: usize, new_element: &U) {
    if let Some(vec) = self.hash.get_mut(key) {
        if index <= vec.len()-1 {
          if index == 0{
            vec.push_front(new_element.clone());
          }else{
            vec[index] = new_element.clone();
            }
        }
      }
  }
  
  /// # `set_value_element_ref`
  /// Sets the value to some elemenet in the queue of the key (can be 0 or other), if the index is greater than the vector length the change will not be made.
  /// 
  /// # Arguments 
  /// * `key: &T` - Key for change the element in its queue.
  /// * `index: usize` - Index of element to change.
  /// * `new_element` - Element for make the change.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key1".to_string(), "value2".to_string());
  /// map.set_value_element_ref(&"key1".to_string(), 1, "new_value".to_string());
  /// assert_eq!(map.get_all_ref(&"key1".to_string()), vec!["value1".to_string(), "new_value".to_string()]);
  /// 
  /// ```
  pub fn set_value_element_ref(&mut self, key: &T, index: usize, new_element: U) {
    if let Some(vec) = self.hash_ref.get_mut(key) {
        if index <= vec.len()-1 {
          if index == 0{
            vec.push_front(Rc::new(new_element));
          }else{
            vec[index] = Rc::new(new_element);
            }         
        }
      }
  }
  
  /// # `set_value_element_something`
  /// Sets the value to some elemenet in the queue of the key (can be 0 or other), if the index is greater than the vector length the change will not be made.
  /// 
  /// # Arguments
  /// * `key: &T` - Key for change the element in its queue.
  /// * `index: usize` - Index of element to change.
  /// * `new_element: F` - Element for make the change.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key1".to_string(), 20);
  /// map.set_value_element_something(&"key1".to_string(), 1, 30);
  /// assert_eq!(map.get_ref_to_all_something(&"key1".to_string()), Some(&VecDeque::from(vec![10, 30])));
  /// 
  /// ```
  pub fn set_value_element_something(&mut self, key:&T, index:usize, new_element:F){
    if let Some(vec) = self.hash_something.get_mut(key){
      if index <= vec.len()-1{
        if index == 0{
          vec.push_front(new_element);
        }else{
          vec[index] = new_element;
        }         
      }
    }
  }
 //----------------------------------------------

 //----------------------------------------------
  /// # `enable_global_order`
  /// Enables the global order register for insertions in keys across the `HashMap of refs`, 
  /// `HashMap of some types of elements`, and `HashMap of copies`.
  /// 
  /// To enable the register for a single `HashMap`, use `enable_order_for_ref` (for `HashMap of refs`), 
  /// `enable_order_for_something` (for `HashMap of some types of elements`), or `enable_order` (for `HashMap of copies`).
  /// 
  /// # Arguments 
  /// * `last_insert_of_key: bool` - If `true`, the register stores only the last insertion for each key.
  /// * `preserve_content_before_order: bool` - If `true`, preserves existing registers for each key and applies 
  ///   the `last_insert_of_key` flag only to future insertions (previous insertion history is saved).
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.enable_global_order(false, false);
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert_ref(&"key1".to_string(), "value2".to_string());
  /// map.insert_something(&"key1".to_string(), 10);
  /// assert_eq!(map.get_order(), vec!["key1".to_string(), "key1".to_string(), "key1".to_string()]);
  /// 
  /// ```
  pub fn enable_global_order(&mut self, last_insert_of_key: bool, preserve_content_before_order: bool){
    self.order_hash = true;
    self.order_hash_ref = true;
    self.store_last_insert_key = last_insert_of_key;
    self.store_last_insert_ref_key = last_insert_of_key;
    self.preserve_before = preserve_content_before_order;
    self.order_hash_something = true;
    self.store_last_insert_something_key = last_insert_of_key;
  }
  
  /// # `enable_order`
  /// Enables the global order register for insertion in keys accross the `HashMap of copies`.
  /// To enable the register for all the `HashMaps` use `enable_global_order` (for `HashMap of refs`, `HashMap of copies` and ` HashMap of some types of elements`). 
  /// 
  /// # Arguments
  /// * `last_insert_of_key: bool` - If `true`, the register stores only the last insertion for each key.
  /// * `preserve_content_before_order` - If `true`, preserves existing registers for each key and applies 
  ///   the `last_insert_of_key` flag only to future insertions (previous insertion history is saved).
  ///   (This flags will be aplicate for all the HashMaps (HashMap of references and HashMap of copies and HashMap of some types of elements)).
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.enable_order(false, false);
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key1".to_string(), &"value2".to_string());
  /// assert_eq!(map.get_order(), vec!["key1".to_string(), "key1".to_string()]);
  /// 
  /// ```
  pub fn enable_order(&mut self, last_insert_of_key: bool,  preserve_content_before_order:bool){
    self.order_hash = true;
    self.store_last_insert_key = last_insert_of_key;
    self.preserve_before =  preserve_content_before_order;
  }
  
  /// # `enable_order_for_ref`
  /// Enables the global order register of the insert in keys, for the `HashMap of refs`.
  /// To enable the register for all the `HashMaps` use `enable_global_order` (for `HashMap of refs` and `HashMap of copies` and `HashMap of some types of elements`). 
  /// 
  /// # Arguments
  /// * `last_insert_of_key: bool` - If `true`, the register stores only the last insertion for each key.
  /// * `preserve_content_before_order` - If `true`, preserves existing registers for each key and applies 
  ///   the `last_insert_of_key` flag only to future insertions (previous insertion history is saved).
  ///   (This flags will be aplicate for all the HashMaps (HashMap of references and HashMap of copies and HashMap of some types of elements))
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.enable_order_for_ref(false, false);
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key1".to_string(), "value2".to_string());
  /// assert_eq!(map.get_order(), vec!["key1".to_string(), "key1".to_string()]);
  /// 
  /// ```
  pub fn enable_order_for_ref(&mut self, last_insert_of_key: bool, preserve_content_before_order: bool){
    self.order_hash_ref = true;
    self.store_last_insert_ref_key = last_insert_of_key;
    self.preserve_before =  preserve_content_before_order;
  }

  /// # `enable_order_for_something`
  /// Enables the global order register of the insert in keys, for the `of some types of elements`
  /// To enable the register for all the `HashMaps` use `enable_global_order` (for `HashMap of refs` and `HashMap of copies` and `HashMap of some types of elements`).
  /// 
  /// # Arguments
  /// * `last_insert_of_key: bool` - Indicate if the register just store the last insert for this key
  /// * `preserve_content_before_order` - If `true`, preserves existing registers for each key and applies 
  ///   the `last_insert_of_key` flag only to future insertions (previous insertion history is saved).
  ///   (This flags will be aplicate for all the HashMaps (HashMap of references and HashMap of copies and HashMap of some types of elements))
  /// 
  /// # Example 
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.enable_order_for_something(false, false);
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key1".to_string(), 20);
  /// assert_eq!(map.get_order(), vec!["key1".to_string(), "key1".to_string()]);
  /// 
  /// ```
  pub fn enable_order_for_something(&mut self, last_insert_of_key: bool, preserve_content_before_order:bool){
    self.preserve_before = preserve_content_before_order;
    self.order_hash_something = true;
    self.store_last_insert_something_key = last_insert_of_key;
  }
  //----------------------------------------------

  //----------------------------------------------  
  /// # `disable_global_order`
  /// Disable continue register the global insert order, but the vector for order conserve the values when the order are be enable
  pub fn disable_global_order(&mut self){
    self.order_hash = false;
    self.order_hash_ref = false;
    self.order_hash_something = false;
  }
  
  /// # `disable_order`
  /// Disable continue register the insert order in `HashMap copies`, but the vector for order conserve the values when the order are be enable
  pub fn disable_order(&mut self){
    self.order_hash = false;
  }

  /// # `disable_order_for_ref`
  /// Disable continue register the insert order in `HashMap refs`, but the vector for order conserve the values when the order are be enable
  pub fn disable_order_for_ref(&mut self){
    self.order_hash_ref = false;
  }
 
  /// # `disable_order_for_something`
  /// Disable continue register the insert order in `HashMap of random values`, but the vector for order conserve the values when the order are be enable
  pub fn disable_order_for_something(&mut self){
   self.order_hash_something = false;
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `get_order`
  /// Get the copy of order vector, in order to first-last insert key
  /// # Rerturn
  /// A property of order vector
  pub fn get_order(&self)->Vec<T>{
    let mut order_vec = Vec::new();
    let mut counter2 = 0;
    while counter2 < self.counter{
      match self.order_o1.get(&counter2).clone(){
        None => {counter2+=1; continue;}
        Some(i) =>{
          order_vec.push(i.clone());
        }
      };
      counter2 +=1;
    }
    return order_vec;
  }
  
  /// # `get_order_ref`
  /// Get a reference for the order HashMaps
  /// # Return
  /// A tuple with references for the order HashMap
  /// * `& HashMap<T, VecDeque<usize>>` - HashMap store all the insertion number asociate for key (Here you can search the insertion register for key)
  /// * `& HashMap<usize, T>` - HashMap store all the insertion for a key asociate for a number of insertion (Here you can search the insertion register for insertion number)
  pub fn get_order_ref(&self)-> (&HashMap<T, VecDeque<usize>>, &HashMap<usize, T>){
    (&self.order, &self.order_o1)
  }
  
  /// # `get_order_mut_ref`
  /// Get a mutable reference for the order HashMaps
  /// # Return
  /// A tuple with mutable references for the order HashMap
  /// * `&mut HashMap<T, VecDeque<usize>>` - HashMap store all the insertion number asociate for key (Here you can search the insertion register for key)
  /// * `&mut HashMap<usize, T>` - HashMap store all the insertion for a key asociate for a number of insertion (Here you can search the insertion register for insertion number)
  pub fn get_order_mut_ref(&mut self)->  (&mut HashMap<T, VecDeque<usize>>, &mut HashMap<usize, T>){
    (&mut self.order, &mut self.order_o1)
  }
  //----------------------------------------------

  //----------------------------------------------
  /// #  `remove_order`
  /// Reset the order HashMaps to empty
  pub fn remove_order(&mut self){
    self.order.clear();
    self.order_o1.clear();
    self.counter = 0;
  }
  
  /// # `get_order_num`
  /// Get the number of insertions done
  /// # Return
  /// `usize` - The number of insertions done
  pub fn get_order_num(&self)-> usize{
    self.counter
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `get_key`
  /// Get the keys where something value appears in a HashMap of copies
  /// # Arguments
  /// * `value_search:U` - Value to search
  /// # Return 
  /// Return a vector with references to keys, or a empty vector if this valu not exist
  pub fn get_key(&self, value_search:&U) -> Vec<&T>{
    let mut keys = Vec::new();
      for i in self.hash.keys(){
        let mut vec = self.hash.get(i).unwrap();
        if vec.contains(value_search){
          keys.push(i);
          continue;
        }
    }
    return keys;
  }
  
  /// # `get_key_ref`
  /// Get the keys where something value appears in a HashMap of refs
  /// # Arguments
  /// * `value_search:U` - Value to search
  /// # Return 
  /// Return a vector with a references to keys, or a empty vector if this valu not exist
  pub fn get_key_ref(&self, value_search:U) -> Vec<&T>{
    let mut keys = Vec::new();
      for i in self.hash_ref.keys(){
        let mut vec = self.hash_ref.get(i).unwrap();
        if vec.contains(&Rc::new(value_search.clone())){
          keys.push(i);
          continue;
        }
    }
    return keys;
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `keys`
  /// Return an iterator as a normal method `keys` from `HashMaps` of HashMap of copies
  /// # Return
  /// * `Keys<'_, T, VecDeque<U>>` - Iterator for the HashMap of copies
  pub fn keys(&self)->Keys<'_, T, VecDeque<U>>{
    self.hash.keys()
  }
  
  /// # `keys_ref`
  /// Return an iterator as a normal method `keys` from `HashMaps` of HashMap of refs
  /// # Return
  /// * `Keys<'_, T, VecDeque<Rc<U>>>` - Iterator for the HashMap of references
  pub fn keys_ref(&self) -> Keys<'_, T, VecDeque<Rc<U>>>{
    self.hash_ref.keys()
  }
  
  /// # `keys_something`
  /// Return an iterator as a normal method `keys` from `HashMaps` from HashMap of random values
  /// # Return
  /// * `Keys<'_, T, VecDeque<F>>` - Iterator for the HashMap of random values
  pub fn keys_something(&mut self)->Keys<'_, T, VecDeque<F>>{
    self.hash_something.keys()
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `get_value`
  /// Get a specific value in the queue of the key in the HashMap of copies
  /// # Arguments
  /// * `key: &T` - Key from take the value
  /// * `index:usize` - Index of the value for take
  /// # Return
  ///   * `None` if the key not exist or index are greater than of the queue of that key
  ///   * `Some(&U)` References of the value
  pub fn get_value(&self, key: &T, index:usize)->Option<&U>{
   if let Some(vec)=self.hash.get(key){
     if index > vec.len()-1{return None;}
      Some(&vec[index])
   }else{
    None
   }
 }
  
  /// # `get_value_ref`
  /// Get a specific value in the queue of the key in the HashMap of refs
  /// # Arguments
  /// * `key: &T` - Key from extract the value
  /// * `index:usize` - Index of the value for extract
  /// # Return
  ///   * `None` if the key not exist or index are greater than of the queue of that key
  ///   * `Some(&U)` References of the value
  pub fn get_value_ref(&self, key: &T, index:usize)->Option<&U>{
   if let Some(vec)=self.hash_ref.get(key){
     if index > vec.len()-1{return None;}
      Some(&*vec[index])
   }else{
    None
   }
 }
  
  /// # `get_value_something`
  /// Get a specific value in the queue of the key in the HashMap of random values
  /// # Arguments
  /// * `key: &T` - Key from take the value
  /// * `index:usize` - Index of the value for take
  /// # Return
  ///  * `None` if the key not exist or index are greater than of the queue of that key
  /// * `Some(&F)` References of the value
  pub fn get_value_something(&self, key:&T, index:usize)->Option<&F>{
    if let Some(vec)=self.hash_something.get(key){
      if index > vec.len()-1{return None;}
       Some(&vec[index])
    }else{
     None
    }
  } 
  //----------------------------------------------

  //----------------------------------------------
  /// # `extract_value`
  /// Extracts something value from a key, or pop this, and therefore remove this of the queue of the key from HashMap of copies
  /// # Arguments
  /// * `key:&T` - Key from extract the value
  /// * `index: usize` - index to extract
  /// # Return
  ///  * `None` If the index are greater than the queue of the key, or if the key not exists
  ///  * `Some(U)` The ownership of the value
  pub fn extract_value(&mut self, key: &T, index:usize) -> Option<U>{
   if let Some(vec) = self.hash.get_mut(key){ 
    if index > vec.len()-1{
      return None;
    }
    if index == 0{
      let n = vec.pop_front().unwrap();
      if vec.len() <= 0{self.hash.remove(key);}
     return Some(n);
    }
    else if index == vec.len()-1{
      let n = vec.pop_back().unwrap();
      if vec.len() <= 0{self.hash.remove(key);}
     return Some(n);
    }
     let n = vec[index].clone();
     vec.remove(index);
     if vec.len() <= 0{self.hash.remove(key);}
     return Some(n);
   }else {return None;}  
 }
 
  /// # `extract_value_ref`
  /// Extracts something value from a key, or pop this, and therefore remove this of the values of the key from HashMap of refs
  /// # Arguments
  /// * `key:&T` - Key from extract the value
  /// * `index: usize` - index to extract
  /// # Return
  ///  * `None` If the index are greater than the queue of the key, or if the key not exists
  ///  * `Some(Rc<U>)` The ownership of the value
  pub fn extract_value_ref(&mut self, key: &T, index:usize) -> Option<Rc<U>>{
   if let Some(vec) = self.hash_ref.get_mut(key){ 
    if index > vec.len()-1{
      return None;
    }
    if index == 0{
      let n = vec.pop_front().unwrap();
      if vec.len() <= 0{self.hash_ref.remove(key);}
     return Some(n);
    }
    else if index == vec.len()-1{
      let n = vec.pop_back().unwrap();
      if vec.len() <= 0{self.hash_ref.remove(key);}
     return Some(n);
    }
     let n = vec[index].clone();
     vec.remove(index);
     if vec.len() <= 0{self.hash_ref.remove(key);}
     return Some(n);
   }else {return None;}  
 }
 
  /// # `extract_value_something`
  /// Extracts something value from a key, or pop this, and therefore remove this of the values of the key from HashMap of random values
  /// # Arguments
  /// * `key:&T` - Key from extract the value
  /// * `index: usize` - index to extract
  /// # Return
  /// * `None` If the index are greater than the queue of the key, or if the key not exists
  /// * `Some(F)` The ownership of the value
  pub fn extract_value_something(&mut self, key:&T, index:usize) -> Option<F>{
    if let Some(vec) = self.hash_something.get_mut(key){ 
     if index > vec.len()-1{
       return None;
     }
     if index == 0{
       let n = vec.pop_front().unwrap();
       if vec.len() <= 0{self.hash_something.remove(key);}
      return Some(n);
     }
     else if index == vec.len()-1{
       let n = vec.pop_back().unwrap();
       if vec.len() <= 0{self.hash_something.remove(key);}
      return Some(n);
     }
     let n = vec.remove(index).unwrap();
      if vec.len() <= 0{self.hash_something.remove(key);}
      return Some(n);
    }else {return None;}  
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `lifo` 
  /// Get the last element in the key and remove it
  /// # Arguments 
  /// * `key:&T` - Key from extract the value
  /// # Return 
  ///  * `None` if the key not exists or the element not exist
  ///  * `Some(U)` The ownership of the value
  pub fn lifo(&mut self, key:&T)->Option<U>{
   if let Some(last) = self.hash.get_mut(key){
     match last.pop_back(){
      None => None,
      Some(i) => {
        if last.len() <= 0{
          self.hash.remove(key);
        }
        return Some(i);
      }
     }
   }else {return None;}
 }
  
  /// # `lifo_ref` 
  /// Get the last element in the key and remove it
  /// # Arguments 
  /// * `key:&T` - Key from extract the value
  /// # Return 
  ///  * `None` if the key not exists or the element not exist
  ///  * `Some(Rc<U>)` The ownership of the value
  pub fn lifo_ref(&mut self, key:&T)->Option<Rc<U>>{
   if let Some(last) = self.hash_ref.get_mut(key){
     match last.pop_back(){
      None => None,
      Some(i) => {
        if last.len() <= 0{
          self.hash_ref.remove(key);
        }
        return Some(i);
      }
     }
   }else {return None;}
 }
  
  /// # `lifo_something`
  /// Get the last element in the key and remove it
  /// # Arguments
  /// * `key:&T` - Key from extract the value
  /// # Return
  /// * `None` if the key not exists or the element not exist
  /// * `Some(F)` The ownership of the value
  pub fn lifo_something(&mut self, key:&T)->Option<F>{
    if let Some(last) = self.hash_something.get_mut(key){
     match last.pop_back(){
      None => None,
      Some(i) => {
        if last.len() <= 0{
          self.hash_something.remove(key);
        }
        return Some(i);
      }
     }
   }else {return None;}
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `peek`
  /// Look the next value in the queue of a key, with that replace the first value when use [`remove`] method 
  /// # Arguments
  /// * `key: &T` - Key for search the next value
  /// # Return 
  /// * `None` - If not exist the key or not exist any next value
  /// * `Some(&U)` - References to the next value in the queue of that key
 pub fn peek(&self, key:&T)->Option<&U>{
    if let Some(next) = self.hash.get(key){
      if next.len() > 1{
      return Some(&next[1]);
      }else {return None;}
    }else {return None;}
 }
 
  /// # `peek_ref`
  /// Look the next value in the queue of a key, with that replace the first value when use [`remove_ref`] method 
  /// # Arguments
  /// * `key: &T` - Key for search the next value
  /// # Return 
  /// * `None` - If not exist the key or not exist any next value
  /// * `Some(&U)` - References to the next value in the queue of that key
 pub fn peek_ref(&self, key:&T)->Option<&U>{
    if let Some(next) = self.hash_ref.get(key){
      if next.len() > 1 {return Some(&next[1]);}
      else {return None;}
    }else {return None;}
 }
 
  /// # `peek_something`
  /// Look the next value in the queue of a key, with that replace the first value when use [`remove_something`] method
  /// # Arguments
  /// * `key: &T` - Key for search the next value
  /// # Return
  /// * `None` - If not exist the key or not exist any next value
  /// * `Some(&F)` - References to the next value in the queue of that key
  pub fn peek_something(&self, key: &T)-> Option<&F>{
     if let Some(next) = self.hash_something.get(key){
      if next.len() > 1 {return Some(&next[1]);}
      else {return None;}
    }else {return None;}
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `clear_hash`
  /// Clear all the hash  of copies of values and keys
  pub fn clear_hash(&mut self){
   self.hash.clear();
 }
  
  /// # `clear_hash_ref`
  /// Clear all the hash of refs of values and keys
  pub fn clear_hash_ref(&mut self){
   self.hash_ref.clear();
 }
  
  /// # `clear_hash_something`
  /// Clear all the hash of random values and keys
  pub fn clear_hash_something(&mut self){
      self.hash_something.clear();
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `iter_mut` 
  /// Return an iterator mutable for the HashMap of copies
  /// # Return
  /// * `IterMut<'_, T, VecDeque<U>>` - Iterator mutable for the HashMap of copies
  pub fn iter_mut(&mut self)-> IterMut<'_, T, VecDeque<U>>{
    self.hash.iter_mut()
  }
  
  /// # `iter_mut_ref`
  /// Return an iterator mutable for the HashMap of references
  /// # Return
  /// * `IterMut<'_, T, VecDeque<Rc<U>>>` - Iterator mutable for the HashMap of references
  pub fn iter_mut_ref(&mut self)->IterMut<'_, T, VecDeque<Rc<U>>>{
    self.hash_ref.iter_mut()
  }

  /// # `iter_mut_something`
  /// Return an iterator mutable for the HashMap of random values
  /// # Arguments
  /// * `key:&T` - Key for get his queue
  /// # Return
  /// * `IterMut<'_, T, VecDeque<F>>` - Iterator mutable for the HashMap of random values
  pub fn iter_mut_something(&mut self)->IterMut<'_, T, VecDeque<F>>{
    self.hash_something.iter_mut()
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `iter`
  /// Return an iterator for the HashMap of copies  
  /// # Return
  /// * `Iter<'_, T, VecDeque<U>>` - Iterator for the HashMap of copies
  pub fn iter(&self)->Iter<'_, T, VecDeque<U>>{
    self.hash.iter()
  }

  /// # `iter_ref`
  /// Return an iterator for the HashMap of references
  /// # Return
  /// * `Iter<'_, T, VecDeque<Rc<U>>>` - Iterator for
  pub fn iter_ref(&self)->Iter<'_, T, VecDeque<Rc<U>>>{
    self.hash_ref.iter()
  }
    
  /// # `iter_something`
  /// Return an iterator for the HashMap of random values
  /// # Return
  /// * `Iter<'_, T, VecDeque<F>>` - Iterator for the HashMap of random values
  pub fn iter_something(&self)->Iter<'_, T, VecDeque<F>>{
    self.hash_something.iter()
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `get_mut_value`
  /// Get a mutable reference for a specific value in the queue of the key in the HashMap of copies
  /// # Arguments
  /// * `key: &T` - Key from take the value
  /// * `index:usize` - Index of the value for take
  /// # Return
  /// * `None` if the key not exist or index are greater than of the queue of that key
  /// * `Some(&mut U)` Mutable References of the value
  pub fn get_mut_value(&mut self, key:&T, index:usize)->Option<&mut U>{
    if let Some(n) = self.hash.get_mut(key){
      return n.get_mut(index);
    }else {return None;}
  }
  
  /// # `get_mut_value_ref`
  /// Get a mutable reference for a specific value in the queue of the key in the HashMap of refs
  /// # Arguments
  /// * `key: &T` - Key from take the value
  /// * `index:usize` - Index of the value for take
  /// # Return
  /// * `None` if the key not exist or index are greater than of the queue of that key
  /// * `Some(&mut Rc<U>)` Mutable References of the value
  pub fn get_mut_value_ref(&mut self, key:&T, index:usize)->Option<&mut Rc<U>>{
    if let Some(n) = self.hash_ref.get_mut(key){
      return n.get_mut(index);
    }else {return None;}
  }

  /// # `get_mut_value_something`
  /// Get a mutable reference for a specific value in the queue of the key in the HashMap of random values
  /// # Arguments
  /// * `key: &T` - Key from take the value
  /// * `index:usize` - Index of the value for take
  /// # Return 
  /// * `None` if the key not exist or index are greater than of the queue of that key
  /// * `Some(&mut F)` Mutable References of the value
  pub fn get_mut_value_something(&mut self, key:&T, index:usize)->Option<&mut F>{
    if let Some(n) = self.hash_something.get_mut(key){
      return n.get_mut(index);
    }else {return None;}
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `remove_value`
  /// Remove a specific value in the queue of that key in the HashMap of copies, if the index is 0 remove the first element, if the index is equal to len-1 remove the last element, else remove the element in that index
  /// # Arguments
  /// * `key:&T` - Key for search the value
  /// * `index:usize` - Index of the value to remove
  pub fn remove_value(&mut self, key:&T, index:usize){
    if let Some(q) = self.hash.get_mut(key){
      if q.len() <= 0{
        self.hash.remove(key);
      }else{
      if index == 0{
        q.pop_front();
      }
      else if index ==q.len()-1{
        q.pop_back();
      }else if index <= q.len()-1{
        q.remove(index);
      }
      if q.len() <=0{
        self.hash.remove(key);
      } 
      }
    } 
  }

  /// # `remove_value_from_ref`
  /// Remove a specific value in the queue of that key in the HashMap of refs, if the index is 0 remove the first element, if the index is equal to len-1 remove the last element, else remove the element in that index
  /// # Arguments
  /// * `key:&T` - Key for search the value
  /// * `index:usize` - Index of the value to remove
  pub fn remove_value_from_ref(&mut self, key:&T, index:usize){
    if let Some(q) = self.hash_ref.get_mut(key){
      if index == 0{
        q.pop_front();
      }
      else if index ==q.len()-1{
        q.pop_back();
      }else if index <= q.len()-1{
        q.remove(index);
      }
      if !q.len() >0{
        self.hash.remove(key);
      } 
    } 
  }

  /// # `remove_value_from_something`
  /// Remove a specific value in the queue of that key in the HashMap of random values, if the index is 0 remove the first element, if the index is equal to len-1 remove the last element, else remove the element in that index
  /// # Arguments
  /// * `key:&T` - Key for search the value
  /// * `index:usize` - Index of the value to remove
  pub fn remove_value_from_something(&mut self, key:&T, index:usize){
   if let Some(q) = self.hash_something.get_mut(key){
      if index == 0{
        q.pop_front();
      }
      else if index ==q.len()-1{
        q.pop_back();
      }else if index <= q.len()-1{
        q.remove(index);
      }
      if !q.len() >0{
        self.hash.remove(key);
      } 
    } 
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `reset_all`
  /// Reset all the struct to the orignial values, as a new instance of the struct 
  pub fn reset_all(&mut self){
   self.hash_ref.clear();
   self.hash.clear();
   self.hash_something.clear();
   self.order.clear();
   self.order_o1.clear();
   self.order_hash =false;
   self.store_last_insert_key=false;
   self.order_hash_ref=false;
   self.order_hash_something=false;
   self.store_last_insert_ref_key=false;
   self.store_last_insert_something_key=false;
   self.preserve_before=false;
   self.iter= 0;
   self.counter= 0;

  }

  /// # `is_empty`
  /// Indicate if the struct is empty, for all the HashMaps
  /// # Return
  /// * `true` if all the HashMaps are empty
  /// * `false` if something HashMap have something value
  pub fn is_empty(&self)->bool{
      return self.hash.is_empty() && self.hash_ref.is_empty() && self.hash_something.is_empty(); 
  }
  
  /// # `clear`
  /// Clear all the HashMaps in the struct
  pub fn clear(&mut self){
    self.hash.clear();
    self.hash_ref.clear();
    self.hash_something.clear();
    self.order.clear();
    self.order_o1.clear();
    self.counter = 0;
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `len`
  /// Get the length of the HashMap of copies
  /// # Return
  /// * `usize` - Length of the HashMap of copies
  pub fn len(&self)->usize{
    self.hash.len()
  }
  
  /// # `len_ref`
  /// Get the length of the HashMap of references
  /// # Return
  /// * `usize` - Length of the HashMap of references
  pub fn len_ref(&self)->usize{
    self.hash_ref.len()
    }

  /// # `len_something`
  /// Get the length of the HashMap of random values
  /// # Return
  /// * `usize` - Length of the HashMap of random values
  pub fn len_something(&self)->usize{
    self.hash_something.len()
    }
  
  /// # `total_len`
  /// Get the total length of all the HashMaps
  /// # Return
  /// * `usize` - Total length of all the HashMaps
  pub fn total_len(&self)->usize{
    self.hash.len() + self.hash_ref.len() + self.hash_something.len()
    }
  
  /// # `order_len`
  /// Get the length of the order HashMaps
  /// # Return
  /// * `(usize, usize)` - A tuple with the lengths of both order HashMaps
  /// * `usize` - Length of the HashMap store all the insertion number asociate for key
  /// * `usize` - Length of the HashMap store all the insertion for a key asociate for a number of insertion
  pub fn order_len(&self)->(usize, usize){
    (self.order.len(), self.order_o1.len())
  }
  //----------------------------------------------

  }
  /* 
   impl<T, U, F> Iterator for Map<T, U, F>
    where 
  T: Clone+ Eq +Hash, 
  U: Clone + PartialEq
  {

    type Item = (Rc<HashMap<T, VecDeque<U>>>, Rc<HashMap<T, VecDeque<Rc<U>>>>, Rc<HashMap<T, VecDeque<F>>>,Rc<HashMap<T, VecDeque<usize>>>, Rc<HashMap<usize, T>>);
    /// # `next`
    /// Return a tuple with copies of all the HashMaps in the struct include the history HashMaps, only once, except HashMap of random values because isn't clonable
    /// # Return
    /// * `Some((HashMap<T, VecDeque<U>>, HashMap<T, VecDeque<Rc<U>>>,  HashMap<T, VecDeque<usize>>,  HashMap<usize, T>))` - A tuple with copies of all the HashMaps in the struct
    /// * `None` - If the iterator have been used before in the same iter
      fn next(&mut self)-> Option<Self::Item>{
        use std::rc::Rc;
        use Map::self;
        if self.iter <= 0{
          self.iter+=1;
          Some( (Rc::hash,  Rc::hash_ref, Rc::hash_something,  Rc::order, Rc::order_o1))
        }
        else {
          self.iter = 0;
          None
        }
      }
  }
  */
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
    ///   - A vector of characters that should be ignored the content between this when removing comments.(can be empty)
    ///   - A vector of strings that should be ignored the content between this when removing comments.(can be empty)
    /// * `scape_characters:&Vec<char>` - A vector of chars for define the scape characters for ignore end delimiters (can be empty)  
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
        if !first_comprobation(ignore_content_between.0, ignore_content_between.1, scape_characters, &[delimiter].to_vec()){
          return None;
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
            contains = contains_ignore(ignore_content_between.0, ignore_content_between.1, &copy);
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
    /// * `delimiters_array_char: &Vec<char>` - Array of chars to indicate pairs that indicate a start and end delimiter of a conent must be are ignored (can be empty)
    /// * `delimiters_array_str: &Vec<&str>` - Array of Strings to indicate pairs that indicate a start and end delimiter of a conent must be are ignored (can be empty)
    /// * `scape_characters:&Vec<char>` - A vector of chars for define the scape characters for ignore end delimiters  (can be empty)
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

       if !first_comprobation(delimiters_array_char, delimiters_array_str, scape_characters, &[delimiter].to_vec()){
        panic!("Error in the parameters");
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
    /// * `delimiters_array:&Vec<String>` - Array that contains the delimiters to indicate when the content are must be ignored (can be empty)
    /// * `scape_characters:&Vec<char>` - A vector of chars for define the scape characters for ignore end delimiters  (can be empty)
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
      use std::collections::HashMap;
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
      let mut map_end = HashMap::new();
        j= 0;//reset j
        // Fill some_start_ignore
        while j<= delimiters_array.len()-1{
          let mut sub_vec = general::sub_vec(delimiters_array, 2, j);
        some_start_ignore.push(sub_vec[0].to_string());
        map_end.insert(sub_vec[0].to_string(), sub_vec[1].to_string());
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
                 if let Some(mut pos_end) = copy2.find(map_end.get(i).unwrap()){
                  start_ignore_index.push(pos_start);
                  if scape_characters.len() >0{
                    if pos_end>0{
                    //check if the delimiter are after a scape character 
                    if scape_characters.contains(&line.to_string().chars().nth(pos_end-1).unwrap()){
                      let mut not_found = false;
                      //remove the last value push in the vector
                      copy2.replace_range(pos_start+i.len()..pos_end+map_end.get(i).unwrap().len(), &general::str_of_n_str(" ", copy2[pos_start+i.len()..pos_end+map_end.get(i).unwrap().len()].len()));
                      loop{
                        // Search the pos of the end delimiter if have
                          if let Some(pos_end2) = copy2.find(map_end.get(i).unwrap()){
                            //check if the delimiter are after a scape character 
                            if scape_characters.contains(&line.to_string().chars().nth(pos_end2-1).unwrap()){
                              copy2.replace_range(pos_start+i.len()..pos_end2+map_end.get(i).unwrap().len(), &general::str_of_n_str(" ", copy2[pos_start+i.len()..pos_end2+map_end.get(i).unwrap().len()].len()));
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
                        expected.push(map_end.get(i).unwrap().clone()); 
                        copy = copy2.to_string();
                        continue;
                      }

                     }
                    }
                  }
                  end_ignore_index.push(pos_end);
                  removed = copy[pos_start..pos_end+map_end.get(i).unwrap().len()].len();
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
                 expected.push(map_end.get(i).unwrap().clone()); 
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
    /// * `ignore_content_between: (&Vec<char>, &Vec<&str>)` - A tuple containing two vectors: `Vec<char>` and `Vec<&str>`. (the vectors can be empty)
    /// * `scape_characters:&Vec<char>` - A vector of chars for define the scape characters for ignore end delimiters. (can be empty)
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
    /// # Return
    /// * `Err(2)` - If there is a block comment without an end delimiter.
    /// * `Err(1)` - If there is a ignore content without close
    /// * `Err(-1)` - If some parameter are corrupted
    /// * `Ok(String)` - If the block comments were successfully removed.
    
    pub fn block_comments(content: &str, start_delimiter: &str, end_delimiter: &str, ignore_content_between: (&Vec<char>, &Vec<&str>), scape_characters:&Vec<char>, mode: ModeBlock, manage_close: ManageClose) -> Result<String, i32>{
      if content.is_empty(){
        panic!("Error: the argument 'conten't is empty");
      }
      if start_delimiter.is_empty() || start_delimiter.contains(" ") || end_delimiter.is_empty() || end_delimiter.contains(" "){
        panic!("Error: start delimiter or end delimiter is empty. Or some comment delimiter contains (' ')");
      }
     if !first_comprobation(ignore_content_between.0, ignore_content_between.1, scape_characters, &[start_delimiter, end_delimiter].to_vec()){
      return Err(-1);
     }
      println!("REMOVING BLOCK COMMENTS FROM CONTENT: {}", content);
      let mut new_content = String::new();
      match mode{
      ModeBlock::Single =>{
        match single_mode(&content, start_delimiter, end_delimiter, ignore_content_between,scape_characters, manage_close){
            Ok(content2) =>  new_content.push_str(&content2) ,
            Err(i) => return Err(i)
        }
       }
       ModeBlock::Nested =>{
        match nested_mode(&content, start_delimiter, end_delimiter, ignore_content_between, scape_characters, manage_close){
          Ok(content2) => new_content.push_str(&content2),
          Err(i) => return Err(i)
        }
       }
      }
      println!("BLOCK COMMENTS REMOVED FROM CONTENT");
      return Ok(new_content);
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
    /// * `ignore_content_between: (&Vec<char>, &Vec<&str>)` - A tuple containing two vectors: `Vec<char>` and `Vec<&str>`. (the vectors can be empty)
    /// * `scape_characters:&Vec<char>` - A vector of chars for define the scape characters for ignore end delimiters. (can be empty)
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
    /// * `ignore_content_between: (&Vec<char>, &Vec<&str>)` - A tuple containing two vectors: `Vec<char>` and `Vec<&str>`. (vectors can be empty)
    /// * `scape_characters:&Vec<char>` - A vector of chars for define the scape characters for ignore end delimiters.  (can be empty)
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
  /// # `first_comprobation`
  /// Contains a necesary comprobation when use content_between function for avoid problems after as a panic
  /// # Arguments
  /// * `delimiters_array_char: &Vec<char>` - Vector of chars for make the comprobation. (can be empty)
  /// * `delimiters_array_str: &Vec<&str>` - Vector of string for make the comprobation. (can be empty)
  /// * `scape_character: &Vec<char>` - Vector of chars for make the comprobation. (can be empty)
  /// * `delimiter: &Vec<&str>` - delimiter for comprobation. (can be empty)
  /// # Return
  /// - `true` if all is fine
  /// - `false` if occurs some error
  /// # Note 
  /// Use in internal functions, not recommended edit
  pub fn first_comprobation(delimiters_array_char: &Vec<char>, delimiters_array_str: &Vec<&str>, scape_characters:&Vec<char>, delimiter: &Vec<&str>)-> bool{
    let mut err = false;
    if delimiter.contains(&&" "){
            println!("Error: The delimiter cannot contains a space (' ').");
            err = true;
        }
        if scape_characters.len()>0{
          if scape_characters.contains(&' '){
            println!("Error: The scape characters vector '{:?}' cannot contains some space character (' ')", scape_characters);
            err = true;
          }
        }
        let mut i: usize = 0;
        if !(delimiters_array_char.is_empty() && delimiters_array_str.is_empty()){
       if !delimiters_array_char.is_empty(){
        let mut err_char = false;
        for ch in delimiters_array_char{
          if delimiter.contains(&&*ch.to_string()){
            println!("Error: The delimiter or delimiters '{:?}' cannot be in the ignore characters vector '{:?}'", delimiter, delimiters_array_char);
             err = true;
             err_char = true;
            }
          if *ch == ' '{
              println!("Error: The ignore delimiter '{}' cannot be a space (' ') the ignore characters vector '{:?}'", *ch, delimiters_array_char);
              err = true;
              err_char = true;
            }
            if scape_characters.len() >0{
             if scape_characters.contains(ch){
              println!("Error: The ignore delimiter '{}' cannot contains a scape character ('{:?}') the ignore characters vector '{:?}'", *ch, scape_characters, delimiters_array_char);
              err = true;
              err_char = true;
             }
           }
           if err_char{break;}  
          }
          //Chekc if the vector delimiters_array_char has an even number of elements
          //Becuase is a pair start-end, so, all the characters must be in pairs, like this: ['{', '}'], ['(', ')'], ['[', ']']
          let i = delimiters_array_char.len();
         if i % 2 != 0{
            println!("Error: The ignore characters vector '{:?}' must have an even number of elements", delimiters_array_char);
          err = true;
         }
        }
        if !delimiters_array_str.is_empty(){
          let mut err_str = false;
        for ch in delimiters_array_str{
          if delimiter.contains(ch){
            println!("Error: The delimiter or delimiters '{:?}' cannot be in the ignore strings vector '{:?}'", delimiter, delimiters_array_str);
          err = true;
          err_str = true;
          }
          if ch.contains(" "){
          println!("Error: The ignore delimiter '{}' cannot contains a space (' ') the ignore characters vector '{:?}'", *ch, delimiters_array_str);
          err = true;
           err_str = true;
          }
           if scape_characters.len() >0{
            for char in ch.chars(){
             if scape_characters.contains(&char){
              println!("Error: The ignore delimiter '{}' cannot contains a scape character ('{:?}') the ignore characters vector '{:?}'", *ch, scape_characters,delimiters_array_str);
              err = true;
               err_str = true;
             }
            }
          }
          if err_str{break;}
         }
         // Chekc if the vector delimiters_array_str has an even number of elements
        //Becuase is a pair start-end, so, all the strings must be in pairs, like this: ["{", "}"], ["(", ")"], ["[", "]"]
          let i = delimiters_array_str.len();

          if i % 2 != 0{
            println!("Error: The ignore strings vector '{:?}' must have an even number of elements", delimiters_array_str);
            err = true;
          }
        }
        if !delimiters_array_char.is_empty() && !delimiters_array_str.is_empty(){
        for ch in delimiters_array_char{
          if delimiters_array_str.contains(&&(*ch.to_string())){
            println!("Error: The ignore characters vector '{:?}' cannot contain the same characters as the ignore strings vector '{:?}'", delimiters_array_char,delimiters_array_str);
            err = true;
            break;
          }
        }
       }
      }
      if err{return false;}
    return true;
  }
//------------------------------------------------------------------------------------------
  /// # `contains_ignore`
  /// Verify if the line contains some ignore delimiter
  /// # Arguments
  /// * `delimiters_array_char: &Vec<char>` - Vector of chars. (can be empty)
  /// * `delimiters_array_str: &Vec<&str>` - Vector of strings. (can be empty)
  /// * `line: &str` - line to verify if contains ignore delimiter. 
  /// # Return 
  /// - `true` if contains some ignore delimiter
  /// - `false` if not contains any ignore delimiter
  pub fn contains_ignore(delimiters_array_char: &Vec<char>, delimiters_array_str: &Vec<&str>, line: &str) -> bool{
    use crate::main_code::utilities::general;
    let mut contains = false;
    let mut j = 0;
            let mut some_start_ignore:Vec<String> = Vec::new();
            if !delimiters_array_char.is_empty(){
             while j <= delimiters_array_char.len()-1{
              let mut sub_vec = general::sub_vec(&delimiters_array_char, 2, j);
              some_start_ignore.push(sub_vec[0].to_string());
              sub_vec.clear();
              j+=2;
              }
             }
             j= 0;
             if !delimiters_array_str.is_empty(){
             while j <= delimiters_array_str.len()-1{
              let mut sub_vec = general::sub_vec(&delimiters_array_str, 2, j);
              some_start_ignore.push(sub_vec[0].to_string());
              sub_vec.clear();
              j+=2;
               } 
             }
            if !some_start_ignore.is_empty(){
              for element in some_start_ignore{
              if line.contains(&element){
                contains = true;
                break;
              }
             }
            }
            return contains;
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
//---------------------------------------------------------------------------------------------------
pub mod formats{
   #![allow(unused)]
  use crate::main_code::utilities::general;
  use std::collections::VecDeque;
  use std::collections::HashMap;
  /// # `Strict`
  /// This struct is used for manage and create strict formats 
  /// Predeterminates values:
  /// * scape characters: ['\\'] (can be modified but cannot be any in or_characters for avoid ambiguities)
  /// * chars reserved for strict format: ['<', '>', '\'', '\''] 
  /// * or character: '|' (can be modified but cannot be '?' or any in scape_characters for avoid ambiguities)
  /// # Note:
  /// The properties and characteristics of a `strict formats` are in the `formats` folder at the repository where you found this crate
  pub struct Strict{
    map: general::Map<String, String, i32>,
    format_from_str: HashMap<String, String>,
    specials_for_str: general::Map<String, char, i32>,
    specials_for_format: general::Map<String, char, i32>,
    scape: VecDeque<char>,
    chars_predet: Vec<char>,
    or: char
  }
  impl Strict{
    /// # `new`
    /// Create a new instance of the struct Strict with the predeterminate values
    /// * scape characters: ['\\'] (can be modified but cannot be any in or_characters for avoid ambiguities)
    /// * chars reserved for strict format: ['<', '>', '\'', '\'']
    /// * or character: '|' (can be modified but cannot be '?' or any in scape_characters for avoid ambiguities)
    /// # Return
    /// - New instance of the struct Strict
    pub fn new()->Self{
      let mut vec = VecDeque::new();
      vec.push_back('\\');
      let vec2 = vec!['<', '>', '\'', '\''];
      Self{
        map:general::Map::new(),
        format_from_str:HashMap::new(),
        specials_for_str:general::Map::new(), 
        specials_for_format:general::Map::new(),
        scape: vec,
        chars_predet:vec2,
        or: '|'
      }
    }
//-------------------------------------------------------------------
    /// # `strict`
    /// This function is used for create a strict format from a string
    /// # Arguments
    /// * `str_from_take_strict_format: &str` - String from which the strict format will be created
    /// * `sensible_to_upper: bool` - If true, the format will be sensible to upper and lower case
    /// * `specials: &Vec<char>` - Vector of special characters that will be used in the format
    /// * `store: bool` - If true, the format will be stored in the map
    /// * `is_a_strict_format: bool` - If true, the string is already a strict format
    /// * `without_order_mode: bool` - If true, the format will be created or proccess like a without order mode strict format (Recomended see the documentation for more information)
    /// # Return
    /// - `Ok(Some(String))` - If the format was created successfully, returns a string with the format generate from the example string
    /// - `Ok(None)` - If the string is empty and is_a_strict_format is false
    /// - `Err((String, VecDeque<String>, String))` - If there was an error, returns a tuple with the error message, the warnings and the errors (This occurs just when is_a_strict_format is true)
    /// # Notes
    /// - Use this function for create a strict format from a string
    /// - If the string is already a strict format, set is_a_strict_format to true
    /// - If the string is not a strict format, set is_a_strict_format to false
    /// - If the string is not a strict format, the function will create the strict format and store it in the map if store is true
    /// - If the string is already a strict format, the function will just verify if the format is correct and return the different options of the format and store it in the map if store is true
    /// - If the string is empty and is_a_strict_format is false, the function will return Ok(None)
    /// - If the string is empty and is_a_strict_format is true, the function will return Err with an error message
    /// - If the specials vector is empty and the string contains 'S' or 's' and is_a_strict_format is true, the function will return Err with an error message
    /// * Err tuple:
    /// - `1: String`: Serious Errors than can make a unexpected format
    /// - `2: VecDeque<String>`: Warnings Than indicate a posible unexpected format
    /// - `3: String`: Information than can be useful here return the format result of the proccess, with all problems and ambiguities solved and the format stored if store is true
    pub fn strict(&mut self, str_from_take_strict_format: &str, sensible_to_upper: bool, specials: &Vec<char>, store: bool, is_a_strict_format:bool, without_order_mode: bool)-> Result<Option<String>, (String, VecDeque<String>, String)>{
           let mut format = String::new();// This string will contain the strict format
      if !str_from_take_strict_format.is_empty() && !is_a_strict_format{
      let mut in_scape = false;// This flag is used for know if we are in a scape character
      let mut end = HashMap::new();// This map will contain the end characters for the flexible format
      let mut in_literal = false;// This flag is used for know if we are in a literal
      let mut flexible_slice = false; // This flag is used for know if we are in a flexible slice in the format
      let mut strict_slice = true;// This flag is used for know if we are in a strict slice in the format
      let mut last_flexible= '\0'; // This char is used for know the last flexible character added to the format for avoid duplicates and stay the properties of a flexible formats, in this case flexible slice
      {
      let mut j = 0;
      // This loop is used for create the end map from the chars_predet vector that contains the start predeterminate characters with his correpondent end character
      while j <= self.chars_predet.len()-1{
        let mut vec = general::sub_vec(&self.chars_predet, 2, j);
        end.insert(vec[0].clone(), vec[1].clone());
        j+=2;
      } 
     }
     // Iterate in the chars from a input string
      for i in str_from_take_strict_format.chars(){
        // First we manage the scape characters, if we are in a scape character, we add this character to the format, change the flag to false and continue to the next character
        if in_scape{
          if !in_literal{
          if strict_slice{format.push(identify_strict(i, sensible_to_upper, specials));}
          else{format.push(identify_flexible(i, sensible_to_upper, specials));}
          in_scape = false;
          continue;
          }else{
            format.push(i);
            continue;
          }
        }
        // If the character is a scape character, we change the flag to true and continue to the next character
        if self.scape.contains(&i){
          in_scape = true;
          continue;
        }
        // Now we manage the flexible slice, strict slice and literals

        // If we are in a flexible slice
        if flexible_slice{
          // Check if we are in a literal or a simbol than indicate the end of a literal slice
          if in_literal{
            // If we are in a literal, we check if the character is the end character for the literal slice
            if let Some(s) = end.get(&'\''){
              if i == *s {
                format.push(*end.get(&'\'').unwrap());
              in_literal = false;
              continue;
            }
            // If the character is not the end character for the literal slice, we add the literal character to the format and continue to the next character
            else{
              format.push(i);
              continue;
            }
            }else{
              format.push(i);
              continue;
            }
          }
          // If we are not in a literal, we check if the character is the end character for the flexible slice
          if let Some(s) = end.get(&'<'){
            if i == *s {
              flexible_slice = false;
            strict_slice = true;
            last_flexible = '\0';
            continue;
            }
          }
          // If the character is not the end character for the flexible slice, we check if the character is the start of other change of mode or '<' than indicate that, so we continue because cannot change the mode inside a flexible slice
          if i == '<'{continue;}
          // If the character is the start of a literal, we change the flag to true and add the character to the format
          else if i == '\''{in_literal = true; format.push('\''); continue;}
          // If the character is the or character, we add this character to the format and continue to the next character
          if i == self.or{format.push(i); continue;}
          // If the character is a flexible character, we add this character to the format, but first we check if the last flexible character added is the same than this, for avoid duplicates and stay the properties of a flexible formats, in this case flexible slice
          let n = identify_flexible(i, sensible_to_upper, specials);
          if last_flexible != n{
          format.push(n);
          last_flexible = n;
          }
          continue;
        }
        // If we are in a strict slice and not in a literal
        if !in_literal && strict_slice{
          // Check if the character is the start character for a flexible slice
        if i == '<'{
          if strict_slice{flexible_slice=true; strict_slice = false;}
         continue;
        }else if i == '\''{
          format.push('\'');
          in_literal = true;
          continue;
        }
        if i == self.or{format.push(i); continue;}
        //we add the strict character to the format
         format.push(identify_strict(i, sensible_to_upper, specials));
         }
         // If we are in a literal
         else if in_literal{
          if i == '\''{in_literal = false;}
          format.push(i);
         }
        }
      // If the without_order_mode is true, we create a without order mode strict format from the or_options
       if store{
        // Store the format in the map
       if let Some(s) = self.map.get_ref_to_all(&format){
        if !s.contains(&str_from_take_strict_format.to_string()){
          self.map.insert(&format, &str_from_take_strict_format.to_string());
        }
       }else{
       self.map.insert(&format, &str_from_take_strict_format.to_string());
       }
          self.format_from_str.insert(str_from_take_strict_format.to_string(), format.to_string());
        
       
       for i in specials{
       match self.specials_for_str.get_ref_to_all(&str_from_take_strict_format.to_string()){
         None => {self.specials_for_str.insert(&str_from_take_strict_format.to_string(), i);},
         Some(r) =>{
          if !r.contains(i){
            self.specials_for_str.insert(&str_from_take_strict_format.to_string(), i);
          }
         }
        };
        } 
       
      }
      return Ok(Some(format));
     }else if is_a_strict_format{
      return Self::handle_format(self, str_from_take_strict_format, specials, store);
     }else{return Ok(None);}
     
    }
//-------------------------------------------------------------------
     /// # `handle_format`
     /// This function is used for handle a strict format from a string
     /// # Arguments
     /// * `str_from_take_strict_format: &str` - String from which the strict format will be handled
     /// * `specials: &Vec<char>` - Vector of special characters that will be used in the format
     /// * `store: bool` - If true, the format will be stored in the map
     /// # Return
     /// - `Ok(Some(String))` - If the format was handled successfully, returns a format result of the proccess with all problems and ambiguities solved and the format stored if store is true
     /// - `Err((String, VecDeque<String>, String))` - If there was an error, returns a tuple with the error message, the warnings and the errors
     fn handle_format(&mut self, str_from_take_strict_format:&str, specials: &Vec<char>, store:bool)->Result<Option<String>, (String, VecDeque<String>, String)>{
      let mut panic_err = String::new();
      let mut warns = VecDeque::new();
      if str_from_take_strict_format.contains(&"S") || str_from_take_strict_format.contains(&"s") && specials.is_empty(){
        panic_err = "Error: can't recognize the specials in the format passed if you not pass them specials, so the 'S' or 's' in the format are not be considered".to_string();
      }
      let mut sub_format = String::new();
      let mut back = '\0';
      let mut last_flexible = '\0';
  
      let mut liter_str = String::new();
      let mut temp = str_from_take_strict_format.trim().chars().peekable();
      while let Some(i) = temp.next(){
           
          if i != 'S' && i != 's' && i!='L' && i!='l' && i!='W' && i!='w' && i!='U' && i!='u' && i!='N' && i!='n' && i != '|'{
           warns.push_back("PRIORITY WARNING!: Ignore characters into format because not correspond to the formats characters".to_string());
           continue;
          }
          if !panic_err.is_empty(){
            if i == 's' || i == 'S'{continue;} 
          }
          if i == 'S' || i == 'L' || i=='W' || i=='N' || i=='U'{
            if i == last_flexible && temp.peek().unwrap_or(&'?') != &'|' && back != '|'{
              warns.push_back("WARNING!: Ignore repeated characters into format because is 'S', 'L', 'W', 'N' or 'U' than means flexible kinds and remove this for avoid ambiguities".to_string());
              continue;
            }
            else{last_flexible = i.clone();}
          }
          else{
            last_flexible = '\0';//reset last_flexible
          }
          sub_format.push(i.clone());
        back = i.clone();
      }
      if store{
        // Store the format in the map
       if !self.map.contains_key(&sub_format){
        self.map.insert(&sub_format, &"".to_string());
       }

       for i in specials{
       match self.specials_for_str.get_ref_to_all(&sub_format){
         None => {self.specials_for_format.insert(&sub_format, i);},
         Some(r) =>{
          if !r.contains(i){
            self.specials_for_format.insert(&sub_format, i);
          }
         }
        };
        } 
       
      }
      if panic_err.is_empty() && warns.is_empty(){return Ok(Some(sub_format));}
      else{return Err((panic_err, warns, sub_format));}
     }
//-------------------------------------------------------------------
     /// # `get_format`
     /// This function is used for get a strict format from a string
     /// # Arguments
     /// * `example: &str` - String from which the strict format will be get
     /// # Return
     /// - `Option<&String>` - If the format exists, returns a reference to the format get from the string, else return None
     pub fn get_format(&self, example: &str)->Option<&String>{
      self.format_from_str.get(&example.to_string())
     }
//-------------------------------------------------------------------
     /// # `get_str`
     /// This function is used for get all strings linked to a strict format
     /// # Arguments
     /// * `format: &str` - Strict format from which the string will be get
     /// # Return
     /// - `Option<&String>` - If the string exists, returns a reference to a vector of strings with all the strings that have this format, else returns None
     pub fn get_str(&self, format: &str) ->Option<&VecDeque<String>>{
      self.map.get_ref_to_all(&format.to_string())
      }
//-------------------------------------------------------------------
      /// # `get_specials`
      /// This function is used for get all special characters linked to a string
      /// # Arguments
      /// * `string_in: &str` - String from which the special characters will be get
      /// # Return
      /// - `Option<&VecDeque<char>>` - If the string exists, returns a reference to a vector of chars with all the special characters that have this string, else returns None
      pub fn get_specials(&self, string_in: &str)->Option<&VecDeque<char>>{
        self.specials_for_str.get_ref_to_all(&string_in.to_string())
      }
//-------------------------------------------------------------------
      /// # `get_specials_from_a_format`
      /// This function is used for get all special characters linked to a strict format, this is useful when you before push and store a format directly and this had specials, not function with formats got from a string example
      /// # Arguments
      /// * `format: &str` - Strict format from which the special characters will be get
      /// # Return
      /// - `Option<&VecDeque<char>>` - If the format exists, returns a reference to a vector of chars with all the special characters that have this format, else returns None
      pub fn get_specials_from_a_format(&self, format:&str)->Option<&VecDeque<char>>{
        self.specials_for_format.get_ref_to_all(&format.to_string())
      }
//-------------------------------------------------------------------
      /// # `get_mut_format`
      /// This function is used for get a mutable reference to a strict format from a string
      /// # Arguments
      /// * `example: &str` - String from which the strict format will be get
      /// # Return
      /// - `Option<&mut String>` - If the format exists, returns a mutable reference to the String than contains the format that have this string, else returns None
      pub fn get_mut_format(&mut self, example: &str)->Option<&mut String>{
        self.format_from_str.get_mut(example)
      }
//-------------------------------------------------------------------
      /// # `get_mut_str`
      /// This function is used for get a mutable reference to all strings linked to a strict format
      /// # Arguments
      /// * `format: &str` - Strict format from which the strings will be get
      /// # Return
      /// - `Option<&mut VecDeque<String>>` - If the format exists, returns a mutable reference to a vector of strings with all the strings that have this format, else returns None
      pub fn get_mut_str(&mut self, format:&str)->Option<&mut VecDeque<String>>{
        self.map.get_mut_ref_to_all(&format.to_string())
      }
//-------------------------------------------------------------------
      /// # `get_mut_specials`
      /// This function is used for get a mutable reference to all special characters linked to a string
      /// # Arguments
      /// * `string_in: &str` - String from which the special characters will be get
      /// # Return
      /// - `Option<&mut VecDeque<char>>` - If the string exists, returns a mutable reference to a vector of chars with all the special characters that have this string, else returns None
      pub fn get_mut_specials(&mut self, string_in:&str)->Option<&mut VecDeque<char>>{
        self.specials_for_str.get_mut_ref_to_all(&string_in.to_string())
      }
//-------------------------------------------------------------------
      /// # `get_mut_specials_from_a_format`
      /// This function is used for get a mutable reference to all special characters linked to a strict format, this is useful when you before push and store a format directly and this had specials, not function with formats got from a string example
      /// # Arguments
      /// * `format: &str` - Strict format from which the special characters will be get
      /// # Return
      /// - `Option<&mut VecDeque<char>>` - If the format exists, returns a mutable reference to a vector of chars with all the special characters that have this format, else returns None
      pub fn get_mut_specials_from_a_format(&mut self, format:&str)->Option<&mut VecDeque<char>>{
        self.specials_for_format.get_mut_ref_to_all(&format.to_string())
      }
//-------------------------------------------------------------------
      /// # `get_map_format`
      /// This function is used for get a reference to the map that contains all the strict formats
      /// # Return
      /// - `&general::Map<String, String, i32>` - Returns a reference to the map that contains all the strict formats
      pub fn get_map_format(&self)->&general::Map<String, String, i32>{
        &self.map
      }
//-------------------------------------------------------------------
      /// # `get_mut_map_format`
      /// This function is used for get a mutable reference to the map that contains all the strict formats
      /// # Return
      /// - `&mut general::Map<String, String, i32>` - Returns a mutable reference to the map that contains all the strict formats
      pub fn get_mut_map_format(&mut self)-> &general::Map<String, String, i32>{
        &mut self.map
      }
//-------------------------------------------------------------------
      /// # `get_map_str`
      /// This function is used for get a reference to the map that contains all the strings linked to a strict format
      /// # Return
      /// - `&HashMap<String, String>` - Returns a reference to the HashMap that contains all the strings linked to a strict format
      pub fn get_map_str(&self)->&HashMap<String, String>{
        &self.format_from_str
      }
//-------------------------------------------------------------------
      /// # `get_mut_map_str`
      /// This function is used for get a mutable reference to the map that contains all the strings linked to a strict format
      /// # Return
      /// - `&mut HashMap<String, String>` - Returns a mutable reference to the HashMap that contains all the strings linked to a strict format
      pub fn get_mut_map_str(&mut self)->&mut HashMap<String, String>{
        & mut self.format_from_str
      }
//-------------------------------------------------------------------
      /// # `get_map_specials`
      /// This function is used for get a reference to the map that contains all the special characters linked to a string
      /// # Return
      /// - `&general::Map<String, char, i32>` - Returns a reference to the map that contains all the special characters linked to a string
      pub fn get_map_specials(&self)->&general::Map<String, char, i32>{
        &self.specials_for_str
      }
//-------------------------------------------------------------------
      /// # `get_mut_map_specials`
      /// This function is used for get a mutable reference to the map that contains all the special characters linked to a string
      /// # Return
      /// - `&mut general::Map<String, char, i32>` - Returns a mutable reference to the map that contains all the special characters linked to a string
      pub fn get_mut_map_speciasl(&mut self)->&mut general::Map<String, char, i32>{
        &mut self.specials_for_str
      }
//-------------------------------------------------------------------
      /// # `get_scape`
      /// This function is used for get a reference to the scape characters vector
      /// # Return
      /// - `&VecDeque<char>` - Returns a reference to the vector that contains all the scape characters
      pub fn get_scape(&self)->&VecDeque<char>{
        &self.scape
      }
//-------------------------------------------------------------------
      /// # `set_scape`
      /// This function is used for set the scape characters vector 
      /// # Arguments
      /// * `new_scape: &Vec<char>` - Vector of characters that will be set as scape characters
      /// # Notes
      /// - The scape characters cannot contain the or character or some reserved character like '<', '>', '\'' and '\'', for avoid ambiguities
      /// - If the scape characters contain the or character  or some reserved character like
      pub fn set_scape(&mut self, new_scape: &Vec<char>){
        if new_scape.contains(&self.or) || self.chars_predet.iter().any(|x| new_scape.contains(x)){
          println!("ERROR: The scape characters cannot contain the 'or' character or '?' or some reserved character like '<', '>', \"'\" and \"'\", the change was not made");
        }else{
        self.scape.clear();
        for i in new_scape{
          self.scape.push_back(*i);
        }
       }
      }
//-------------------------------------------------------------------
      /// # `get_or`
      /// This function is used for get the or character
      /// # Return
      /// - `&char` - Returns a reference to the or character
      pub fn get_or(&self)->&char{
        &self.or
      }
//-------------------------------------------------------------------
      /// # `set_or`
      /// This function is used for set the or character
      /// # Arguments
      /// * `new_or: char` - Character that will be set as or character
      /// # Notes
      /// - The or character cannot be '?' or any scape character for avoid ambiguities
      /// - If the or character is '?' or any scape character, the change will not be made and an error message will be printed
      pub fn set_or(&mut self, new_or: char){
        if new_or == '?' || self.scape.contains(&new_or) || self.chars_predet.contains(&new_or){
          println!("ERROR: The 'or' character cannot be '?' or any scape character or some reserved character like '<', '>', \"'\" and \"'\", the change was not made");
        }
        else{self.or = new_or;}
      }
//-------------------------------------------------------------------
      /// # `reset`
      /// This function is used for reset all the properties of the struct to the predeterminate values
      /// * scape characters: ['\'] (can be modified but cannot be any in or_characters for avoid ambiguities)
      /// * or: '|' (can be modified but cannot be '?' or any in scape_characters for avoid ambiguities)
      /// # Notes
      /// - This function will clear all the maps and set the scape characters and or character to the predeterminate values
      pub fn reset(&mut self){
        let mut vec = VecDeque::new();
      vec.push_back('\\');
     
        self.map.clear();
        self.format_from_str.clear();
        self.specials_for_str.clear();
        self.scape = vec;
        self.or = '|';
      }
//--------------------------------------------------------------------
      /// # `get_reserved_chars`
      /// This function is used for get a reference to the reserved characters vector
      /// # Return
      /// - `&Vec<char>` - Returns a reference to the vector that contains all the reserved characters
      pub fn get_reserved_chars(&self)->&Vec<char>{
        &self.chars_predet
      }
  }


//---------------------------------------------------------------------------------------------------

  /// # `Flexible`
  /// This struct is used for manage and create flexible formats 
  /// Predeterminates values:
  /// * scape characters: ['\\'] (can be modified but cannot be any in or_characters for avoid ambiguities)
  /// * chars reserved for strict format: ['<', '>', '\'', '\''] 
  /// * or character: '|' (can be modified but cannot be '?' or any in scape_characters for avoid ambiguities)
  /// # Note:
  /// The properties and characteristics of a `flexible formats` are in the `formats` folder at the repository where you found this crate
 pub struct Flexible{
    map: general::Map<String, String, i32>,
    format_from_str: HashMap<String, String>,
    specials_for_str: general::Map<String, char, i32>,
    scape: VecDeque<char>,
    chars_predet: Vec<char>,
    or: char
  }
  impl Flexible{
    /// # `new`
    /// Create a new instance of the struct Flexible with the predeterminate values
    pub fn new()->Self{
      let mut vec = VecDeque::new();
      vec.push_back('\\');
      let vec2 = vec!['<', '>', '\'', '\''];
      Self{
        map:general::Map::new(),
        format_from_str:HashMap::new(),
        specials_for_str:general::Map::new(), 
        scape: vec,
        chars_predet:vec2,
        or: '|'
      }
    }
//-------------------------------------------------------------------
    /// # `flexible`
    /// This function is used for create a flexible format from a string
    /// # Arguments
    /// * `str_from_take_flexible_format: &str` - String from which the flexible format will be created
    /// * `sensible_to_upper: bool` - If true, the format will be sensible to upper and lower case
    /// * `specials: &Vec<char>` - Vector of special characters that will be used in the format
    /// * `store: bool` - If true, the format will be stored in the map
    /// * `is_a_flexible_format: bool` - If true, the string is already as a flexible format
    /// # Return
    /// - `Ok(Some(String))` - If the format was created successfully, returns a string with the format generate from the example string
    /// - `Ok(None)` - If the string is empty and is_a_flexible_format is false
    /// - `Err((String, VecDeque<String>, String))` - If there was an error, returns a tuple with the error message, the warnings and the errors (This occurs just when is_a_flexible_format is true)
    /// # Notes
    /// - Use this function for create a flexible format from a string
    /// - If the string is already a flexible format, set is_a_flexible_format to true
    /// - If the string is not a flexible format, set is_a_flexible_format to false
    /// - If the string is not a flexible format, the function will create the flexible format and store it in the map if store is true
    /// - If the string is already a flexible format, the function will just verify if the format is correct and return the different options of the format and store it in the map if store is true
    /// - If the string is empty and is_a_flexible_format is false, the function will return Ok(None)
    /// - If the string is empty and is_a_flexible_format is true, the function will return Err with an error message
    /// - If the specials vector is empty and the string contains 'S' or 's' and is_a_flexible_format is true, the function will return Err with an error message
    /// * Err tuple:
    /// - `1: String`: Serious Errors than can make a unexpected format
    /// - `2: VecDeque<String>`: Warnings Than indicate a posible unexpected format
    /// - `3: String`: Information than can be useful here return the format result of the proccess, with all problems and ambiguities solved and the format stored if store is true
    pub fn flexible(&mut self, str_from_take_flexible_format: &str, sensible_to_upper: bool, specials: &Vec<char>, store: bool, is_a_flexible_format:bool)-> Result<Option<String>, (String, VecDeque<String>, String)>{
           let mut format = String::new();
      if !str_from_take_flexible_format.is_empty() && !is_a_flexible_format{
      let mut in_scape = false;
      let mut end = HashMap::new();
      let mut in_literal = false;
      let mut flexible_slice = true;
      let mut strict_slice = false;
      let mut last_flexible= '\0';
      let mut j = 0;
      while j <= self.chars_predet.len()-1{
        let mut vec = general::sub_vec(&self.chars_predet, 2, j);
        end.insert(vec[0].clone(), vec[1].clone());
        j+=2;
      } 
      for i in str_from_take_flexible_format.chars(){

        if in_scape{
          if !in_literal{
          if strict_slice{format.push(identify_strict(i, sensible_to_upper, specials));}
          else{format.push(identify_flexible(i, sensible_to_upper, specials));}
          in_scape = false;
          continue;
          }else{
            format.push(i);
            continue;
          }
        }
        if self.scape.contains(&i){
          in_scape = true;
          continue;
        }

        if strict_slice{
          if in_literal{
            if let Some(s) = end.get(&'\''){
              if i == *s {
                format.push('\'');
              in_literal = false;
              continue;
            }else{
              format.push(i);
              continue;
            }
            }else{
              format.push(i);
              continue;
            }
          }
          if let Some(s) = end.get(&'<'){
            if i == *s {
              flexible_slice = true;
            strict_slice = false;
            continue;
            }
          }
          if i == '<'{continue;}
          else if i == '\''{in_literal = true; format.push('\''); continue;}
          if i == '|'{format.push(i); continue;}
          let n = identify_strict(i, sensible_to_upper, specials);
          format.push(n);
          continue;
        }
        if !in_literal && flexible_slice{
        if i == '<'{
          if flexible_slice{flexible_slice=false; strict_slice = true;}
         continue;
        }else if i == '\''{
          format.push('\'');
          in_literal = true;
          last_flexible = '\0';
          continue;
        }
        if i == '|'{format.push(i); last_flexible = '\0'; continue;}
        let mut n = identify_flexible(i, sensible_to_upper, specials);
        if last_flexible != n{
          format.push(n);
          last_flexible = n;
          }
         }else if in_literal{
          if i == '\''{in_literal = false;}
          format.push(i);
         }
        }

       if store{
    
       if let Some(s) = self.map.get_ref_to_all(&format){
        if !s.contains(&str_from_take_flexible_format.to_string()){
          self.map.insert(&format, &str_from_take_flexible_format.to_string());
        }
       }else{
       self.map.insert(&format, &str_from_take_flexible_format.to_string());
       }
       self.format_from_str.insert(str_from_take_flexible_format.to_string(), format.to_string());
       for i in specials{
       match self.specials_for_str.get_ref_to_all(&str_from_take_flexible_format.to_string()){
         None => {self.specials_for_str.insert(&str_from_take_flexible_format.to_string(), i);},
         Some(r) =>{
          if !r.contains(i){
            self.specials_for_str.insert(&str_from_take_flexible_format.to_string(), i);
          }
         }
        };
        } 
       
      }
      return Ok(Some(format));
     }else if is_a_flexible_format{
      return Self::handle_format(self, str_from_take_flexible_format, specials, store);
     }else{return Ok(None);}
     
    }
//-------------------------------------------------------------------
      /// # `handle_format`
      /// This function is used for handle a flexible format from a string
      /// # Arguments
      /// * `str_from_take_flexible_format: &str` - String from which the flexible format will be handled
      /// * `specials: &Vec<char>` - Vector of special characters that will be
      /// * `store: bool` - If true, the format will be stored in the map
      /// # Return
      /// - `Ok(Some(String))` - If the format was handled successfully, returns a format result of the proccess with all problems and ambiguities solved and the format stored if store is true
      /// - `Err((String, VecDeque<String>, String))` - If there was an error, returns a tuple with the error message, the warnings and the errors
     fn handle_format(&mut self, str_from_take_flexible_format:&str, specials: &Vec<char>, store:bool)->Result<Option<String>, (String, VecDeque<String>, String)>{
      let mut panic_err = String::new();
      let mut warns = VecDeque::new();
      if str_from_take_flexible_format.contains(&"S") || str_from_take_flexible_format.contains(&"s") && specials.is_empty(){
        panic_err = "Error: can't recognize the specials in the format passed if you not pass them specials, so the 'S' or 's' in the format are not be considered".to_string();
      }
      let mut sub_format = String::new();
      let mut back = '\0';
      let mut last_flexible = '\0';
  
      let mut liter_str = String::new();
      let mut temp = str_from_take_flexible_format.trim().chars().peekable();
      while let Some(i) = temp.next(){
           
          if i != 'S' && i != 's' && i!='L' && i!='l' && i!='W' && i!='w' && i!='U' && i!='u' && i!='N' && i!='n' && i != '|'{
           warns.push_back("PRIORITY WARNING!: Ignore characters into format because not correspond to the formats characters".to_string());
           continue;
          }
          if !panic_err.is_empty(){
            if i == 's' || i == 'S'{continue;} 
          }
          if i == 'S' || i == 'L' || i=='W' || i=='N' || i=='U'{
            if i == last_flexible && temp.peek().unwrap_or(&'?') != &'|' && back != '|'{continue;}
            else{last_flexible = i.clone();}
          }
          else{
            last_flexible = '\0';//reset last_flexible
          }
          sub_format.push(i.clone());
        back = i.clone();
        }
      if store{
       if let Some(s) = self.map.get_ref_to_all(&sub_format){
        if !s.contains(&str_from_take_flexible_format.to_string()){
          self.map.insert(&sub_format, &str_from_take_flexible_format.to_string());
        }
       }else{
       self.map.insert(&sub_format, &str_from_take_flexible_format.to_string());
       }
       self.format_from_str.insert(str_from_take_flexible_format.to_string(), sub_format.clone());
       for i in specials{
       match self.specials_for_str.get_ref_to_all(&str_from_take_flexible_format.to_string()){
         None => {self.specials_for_str.insert(&str_from_take_flexible_format.to_string(), i);},
         Some(r) =>{
          if !r.contains(i){
            self.specials_for_str.insert(&str_from_take_flexible_format.to_string(), i);
           }
          }
         };
        }
       
      }
      if panic_err.is_empty() && warns.is_empty(){return Ok(Some(sub_format));}
      else{return Err((panic_err, warns, sub_format));}
     }
//-------------------------------------------------------------------
      /// # `get_format`
      /// This function is used for get a flexible format from a string
      /// # Arguments
      /// * `example: &str` - String from which the flexible format will be get
      /// # Return
      /// - `Option<&String>` - If the format exists, returns a reference to the format get from the string, else return None
     pub fn get_format(&self, example: &str)->Option<&String>{
      self.format_from_str.get(example)
     }
//-------------------------------------------------------------------
     pub fn get_str(&self, format: &str) ->Option<&VecDeque<String>>{
      self.map.get_ref_to_all(&format.to_string())
      }
//-------------------------------------------------------------------
      pub fn get_specials(&self, string_in: &str)->Option<&VecDeque<char>>{
        self.specials_for_str.get_ref_to_all(&string_in.to_string())
      }
//-------------------------------------------------------------------
      pub fn get_mut_format(&mut self, example: &str)->Option<&mut String>{
        self.format_from_str.get_mut(example)
      }
//-------------------------------------------------------------------
      pub fn get_mut_str(&mut self, format:&str)->Option<&mut VecDeque<String>>{
        self.map.get_mut_ref_to_all(&format.to_string())
      }
//-------------------------------------------------------------------
      pub fn get_mut_specials(&mut self, string_in:&str)->Option<&mut VecDeque<char>>{
        self.specials_for_str.get_mut_ref_to_all(&string_in.to_string())
      }
//-------------------------------------------------------------------
      pub fn get_map_format(&self)->&general::Map<String, String, i32>{
        &self.map
      }
//-------------------------------------------------------------------
      pub fn get_mut_map_format(&mut self)-> &general::Map<String, String, i32>{
        &mut self.map
      }
//-------------------------------------------------------------------
      pub fn get_map_str(&self)->&HashMap<String, String>{
        &self.format_from_str
      }
//-------------------------------------------------------------------
      pub fn get_mut_map_str(&mut self)->&mut HashMap<String, String>{
        & mut self.format_from_str
      }
//-------------------------------------------------------------------
      pub fn get_map_specials(&self)->&general::Map<String, char, i32>{
        &self.specials_for_str
      }
//-------------------------------------------------------------------
      pub fn get_mut_map_specials(&mut self)->&mut general::Map<String, char, i32>{
        &mut self.specials_for_str
      }

  }
  fn identify_strict(identify:char, sensible_to_upper: bool, specials: &Vec<char>)-> char{
     if specials.contains(&identify){
          return 's';
        }
     else if identify <= '9' && identify>= '0'{
            return 'n';
          }else if identify <= 'Z' && identify >='A'{
            if sensible_to_upper{
              return 'u';
            }else{return 'l';}
          }else if identify <= 'z' && identify >='a'{
            if sensible_to_upper{
              return 'w';
            }else{return 'l';}
          }else {
              return 'l';
          }
    }
//-------------------------------------------------------------------
  fn identify_flexible(identify:char, sensible_to_upper: bool, specials: &Vec<char>)->char{
    if specials.contains(&identify){
          return 'S';
        }
     else if identify <= '9' && identify>= '0'{
            return 'N';
          }else if identify <= 'Z' && identify >='A'{
            if sensible_to_upper{
              return 'U';
            }else{return 'L';}
          }else if identify <= 'z' && identify >='a'{
            if sensible_to_upper{
              return 'W';
            }else{return 'L';}
          }else {
              return 'L';
          }
  }
//------------------------------------------------------------------
  pub fn handle_random_order(){
    
  }
//------------------------------------------------------------------
/* 
  pub fn compare_formats(formats_for_compare:Vec<&String>, format_to_compare:&String, or_delimiter: &char, literal_delimiters: &Vec<char>)/*->Option<(bool, Vec<usize>)>*/{
    if (*or_delimiter!='\0' && *or_delimiter != '\r' && *or_delimiter!='\n') && !literal_delimiters.is_empty(){
      if literal_delimiters.contains(or_delimiter){
        println!("ERROR: The or delimiter cannot be a literal delimiter for avoid ambiguities");
        return;
      }
    }
    if !literal_delimiters.is_empty(){
      if literal_delimiters.len() % 2 != 0{
        println!("ERROR: The literal delimiters vector must be even, for generate the pair of delimiters when the right is the open delimiter and the left is the closed delimiter");
        return;
      }
    }
    for i in formats_for_compare{
      let mut in_or = false;
      let mut in_literal = false;
      let mut position_options:general::Map<usize, String, usize> = general::Map::new();
      let mut copy = i.to_string().chars().peekable();
      let mut position = 0;
      for (i2, n) in copy.enumerate(){
        if n == *or_delimiter && !in_literal && !in_or{
          position = i2;
          in_or = true;
        }
        if in_or{
          if n != *or_delimiter{
            position_options.insert(&position, &n.to_string());
            in_or = false;
          }
          continue;
        }else{
            position_options.insert(&i2, &n.to_string());
        }
      }
    }
  }
  */
}
