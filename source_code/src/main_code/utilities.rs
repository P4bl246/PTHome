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
    /// ## Notes 
    /// - The default delimiter is an space `' '`.
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
/// ## Notes
/// - We call `queue` a vector which stored values for a key, enabling support multiples values for the same key. 
/// - The values will be extracted in FIFO order (properties from a `queue`).
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
  /// ## Notes
  /// - Does not exist a method for get all the key values for the HashMap of some types of elements, because they might not be clonnable.
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
  /// ## Notes
  /// - Does not exist a method for get all the key values for the HashMap of some types of elements, because they might not be clonnable.
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
  /// ## Notes
  /// - Does not exist a method for get all the key values for the HashMap of some types of elements, because they might not be clonnable.
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
  /// ## Notes
  /// - Does not exist a method for get all the key values for the HashMap of some types of elements, because they might not be clonnable.
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
  /// Disables tracking of insertions for all HashMaps. The order vector retains previously recorded insertions
  /// from when the tracking was enabled.
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.enable_global_order(false, false);
  /// 
  /// ```
  pub fn disable_global_order(&mut self){
    self.order_hash = false;
    self.order_hash_ref = false;
    self.order_hash_something = false;
  }
  
  /// # `disable_order`
  /// Disables tracking of insertions for the `HashMap of copies`. The order vector retains previously recorded insertions 
  /// from when the tracking was enabled.
  /// 
  /// # Example 
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.enable_order(false, false);
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.disable_order();
  /// map.insert(&"key2".to_string(), &"value2".to_string());
  /// assert_eq!(map.get_order(), vec!["key1".to_string()]);
  /// 
  /// ```
  pub fn disable_order(&mut self){
    self.order_hash = false;
  }

  /// # `disable_order_for_ref`
  /// Disables tracking of insertions for the `HashMap of refs`. The order vector retains previously recorded insertions
  /// from when the tracking was enabled.
  /// 
  /// # Example 
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.enable_order_for_ref(false, false);
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.disable_order_for_ref();
  /// map.insert_ref(&"key2".to_string(), "value2".to_string());
  /// assert_eq!(map.get_order(), vec!["key1".to_string()]);
  /// 
  /// ```
  pub fn disable_order_for_ref(&mut self){
    self.order_hash_ref = false;
  }
 
  /// # `disable_order_for_something`
  /// Disables tracking of insertions for the `HashMap of some types of elements`. The order vector retains previously recorded insertions 
  /// from when the tracking was enabled.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.enable_order_for_something(false, false);
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.disable_order_for_something();
  /// map.insert_something(&"key2".to_string(), 20);
  /// assert_eq!(map.get_order(), vec!["key1".to_string()]);
  /// 
  /// ```
  pub fn disable_order_for_something(&mut self){
   self.order_hash_something = false;
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `get_order`
  /// Gets the insertion order (from first to last inserted key).
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.enable_global_order(false, false);
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert_ref(&"key2".to_string(), "value2".to_string());
  /// map.insert_something(&"key1".to_string(), 10);
  /// assert_eq!(map.get_order(), vec!["key1".to_string(), "key2".to_string(), "key1".to_string()]);
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `Vec<T>` - Vector with the order of insertions done.
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
  /// Gets the HashMaps used to save the insertion order.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.enable_global_order(false, false);
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert_ref(&"key2".to_string(), "value2".to_string());
  /// map.insert_something(&"key1".to_string(), 10);
  /// let (order_map, order_map_o1) = map.get_order_ref();
  /// assert_eq!(order_map.get(&"key1".to_string()), Some(&VecDeque::from(vec![0, 2])));
  /// assert_eq!(order_map_o1.get(&1), Some(&"key2".to_string()));
  /// 
  /// ```
  /// 
  /// # Returns
  /// A tuple with references to the HashMaps used to save the insertion order.
  /// * `&HashMap<T, VecDeque<usize>>` - HashMap that stores keys with their insertion numbers (search by key to get insertion history).
  /// * `&HashMap<usize, T>` - HashMap that stores insertion numbers with their keys (search by insertion number to find the key).

  pub fn get_order_ref(&self)-> (&HashMap<T, VecDeque<usize>>, &HashMap<usize, T>){
    (&self.order, &self.order_o1)
  }
  
  /// # `get_order_mut_ref`
  /// Gets mutable references to the HashMaps used to save the insertion order.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.enable_global_order(false, false);
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert_ref(&"key2".to_string(), "value2".to_string());
  /// map.insert_something(&"key1".to_string(), 10);
  /// let (order_map, order_map_o1) = map.get_order_mut_ref();
  /// assert_eq!(order_map.get(&"key1".to_string()), Some(&VecDeque::from(vec![0, 2])));
  /// assert_eq!(order_map_o1.get(&1), Some(&"key2".to_string()));
  /// 
  /// ```
  /// 
  /// # Returns
  /// A tuple with mutable references to the HashMaps used to save the insertion order.
  /// * `&mut HashMap<T, VecDeque<usize>>` - HashMap that stores keys with their insertion numbers (search by key to get insertion history).
  /// * `&mut HashMap<usize, T>` - HashMap that stores insertion numbers with their keys (search by insertion number to find the key).
  pub fn get_order_mut_ref(&mut self)->  (&mut HashMap<T, VecDeque<usize>>, &mut HashMap<usize, T>){
    (&mut self.order, &mut self.order_o1)
  }
  //----------------------------------------------

  //----------------------------------------------
  /// #  `remove_order`
  /// Clear the HashMaps used to save the insertion order, and resets the insertions counter (to 0).
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.enable_global_order(false, false);
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert_ref(&"key2".to_string(), "value2".to_string());
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.remove_order();
  /// assert_eq!(map.get_order(), vec![]);
  /// 
  /// ```
  pub fn remove_order(&mut self){
    self.order.clear();
    self.order_o1.clear();
    self.counter = 0;
  }
  
  /// # `get_order_num`
  /// Gets the number of insertions done (the insertions counter).
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.enable_global_order(false, false);
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert_ref(&"key2".to_string(), "value2".to_string());
  /// map.insert_something(&"key1".to_string(), 10);
  /// assert_eq!(map.get_order_num(), 3);
  /// 
  /// ```
  /// 
  /// # Returns
  /// `usize` - The number of insertions done.
  pub fn get_order_num(&self)-> usize{
    self.counter
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `get_key`
  /// Gets the keys where the value appears (in the HashMap of copies).
  /// # Arguments
  /// * `value_search:U` - Value to search.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key2".to_string(), &"value2".to_string());
  /// map.insert(&"key1".to_string(), &"value2".to_string());
  /// assert_eq!(map.get_key(&"value2".to_string()), vec![&"key2".to_string(), &"key1".to_string()]);
  /// 
  /// ```
  /// 
  /// # Returns
  /// A vector with the references to the keys, or an empty vector if this value does not exist.
  /// 
  /// ## Notes
  /// - No equivalent method exists for the HashMap of some types of elements, because
  /// those types might not be comparable.
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
  /// Gets the keys where the value appears (in the HashMap of references).
  /// # Arguments
  /// * `value_search:U` - Value to search.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key2".to_string(), "value2".to_string());
  /// map.insert_ref(&"key1".to_string(), "value2".to_string());
  /// assert_eq!(map.get_key_ref("value2".to_string()), vec![&"key2".to_string(), &"key1".to_string()]);
  /// 
  /// ```
  /// 
  /// # Returns
  /// A vector with a references to the keys, or an empty vector if this value does not exist.
  /// 
  /// ## Notes
  /// - No equivalent method exists for the HashMap of some types of elements, because
  /// those types might not be comparable.
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
  /// Returns an iterator over the keys (like the standard `keys()` method) from the HashMap of copies.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use std::collections::hash_map::Keys;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key2".to_string(), &"value2".to_string());
  /// let keys: Vec<String> = map.keys().cloned().collect();
  /// assert_eq!(keys, vec!["key1".to_string(), "key2".to_string()]);
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `Keys<'_, T, VecDeque<U>>` - Iterator over keys in the HashMap of copies.
  pub fn keys(&self)->Keys<'_, T, VecDeque<U>>{
    self.hash.keys()
  }
  
  /// # `keys_ref`
  /// Returns an iterator over the keys (like the standard `keys()` method) from the HashMap of references.
  /// 
  /// # Example 
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use std::collections::hash_map::Keys;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key2".to_string(), "value2".to_string());
  /// let keys: Vec<String> = map.keys_ref().cloned().collect();
  /// assert_eq!(keys, vec!["key1".to_string(), "key2".to_string()]);
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `Keys<'_, T, VecDeque<Rc<U>>>` - Iterator over keys in the HashMap of copies.
  pub fn keys_ref(&self) -> Keys<'_, T, VecDeque<Rc<U>>>{
    self.hash_ref.keys()
  }
  
  /// # `keys_something`
  /// Returns an iterator over the keys (like the standard `keys()` method) from the HashMap of some types of elements.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use std::collections::hash_map::Keys;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key2".to_string(), 20);
  /// let keys: Vec<String> = map.keys_something().cloned().collect();
  /// assert_eq!(keys, vec!["key1".to_string(), "key2".to_string()]);
  /// 
  /// ``` 
  /// 
  /// # Returns
  /// * `Keys<'_, T, VecDeque<F>>` - Iterator over keys in the HashMap of copies.
  pub fn keys_something(&mut self)->Keys<'_, T, VecDeque<F>>{
    self.hash_something.keys()
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `get_element`
  /// Gets a specific element in the queue of the key in the HashMap of copies.
  /// 
  /// # Arguments
  /// * `key: &T` - Key for search the element.
  /// * `index:usize` - Index of the element.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"element1".to_string());
  /// map.insert(&"key1".to_string(), &"element2".to_string());
  /// assert_eq!(map.get_element(&"key1".to_string(), 1), Some(&"element2".to_string()));
  /// 
  /// ```
  /// 
  /// # Returns
  ///   * `None` if the key does not exist or the index are greater than the queue length.
  ///   * `Some(&U)` References to the element.
  pub fn get_element(&self, key: &T, index:usize)->Option<&U>{
   if let Some(vec)=self.hash.get(key){
     if index > vec.len()-1{return None;}
      Some(&vec[index])
   }else{
    None
   }
 }
  
  /// # `get_element_ref`
  /// Gets a specific element in the queue of the key in the HashMap of refs.
  /// 
  /// # Arguments
  /// * `key: &T` - Key for search the element.
  /// * `index:usize` - Index of the element.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "element1".to_string());
  /// map.insert_ref(&"key1".to_string(), "element2".to_string());
  /// assert_eq!(map.get_element_ref(&"key1".to_string(), 1), Some(&"element2".to_string()));
  ///
  /// ```
  /// 
  /// # Returns
  ///   * `None` if the key does not exist or the index are greater than of the queue length.
  ///   * `Some(&U)` References of the element.
  pub fn get_element_ref(&self, key: &T, index:usize)->Option<&U>{
   if let Some(vec)=self.hash_ref.get(key){
     if index > vec.len()-1{return None;}
      Some(&*vec[index])
   }else{
    None
   }
 }
  
  /// # `get_element_something`
  /// Gets a specific element in the queue of the key in the HashMap of some types of elements.
  /// 
  /// # Arguments
  /// * `key: &T` - Key for search the element.
  /// * `index:usize` - Index of the element.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key1".to_string(), 20);
  /// assert_eq!(map.get_element_something(&"key1".to_string(), 1), Some(&20));
  /// 
  /// ```
  /// 
  /// # Returns
  ///  * `None` if the key does not exist or the index are greater than of the queue length.
  /// * `Some(&F)` References of the element.
  pub fn get_element_something(&self, key:&T, index:usize)->Option<&F>{
    if let Some(vec)=self.hash_something.get(key){
      if index > vec.len()-1{return None;}
       Some(&vec[index])
    }else{
     None
    }
  } 
  //----------------------------------------------

  //----------------------------------------------
  /// # `extract_element`
  /// Extracts some element from a key, therefore, removes it of the queue of the key in the HashMap of copies.
  /// 
  /// # Arguments
  /// * `key:&T` - Key to extracts the element.
  /// * `index: usize` - index to extract.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"element1".to_string());
  /// map.insert(&"key1".to_string(), &"element2".to_string());
  /// assert_eq!(map.extract_element(&"key1".to_string(), 0), Some("element1".to_string()));
  /// 
  /// ```
  /// 
  /// # Returns
  ///  * `None` If the index are greater than the queue length, or if the key does not exist.
  ///  * `Some(U)` The ownership of the element.
  pub fn extract_element(&mut self, key: &T, index:usize) -> Option<U>{
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
 
  /// # `extract_element_ref`
  /// Extracts some element from a key, therefore, removes it of the elements of the key in the HashMap of refs.
  /// 
  /// # Arguments
  /// * `key:&T` - Key to extract the element.
  /// * `index: usize` - index to extract.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "element1".to_string());
  /// map.insert_ref(&"key1".to_string(), "element2".to_string());
  /// assert_eq!(map.extract_element_ref(&"key1".to_string(), 0), Some(Rc::new("element1".to_string())));
  /// 
  /// ```
  /// 
  /// # Returns
  ///  * `None` If the index are greater than the queue of the key, or if the key does not exists.
  ///  * `Some(Rc<U>)` The ownership of the element.
  pub fn extract_element_ref(&mut self, key: &T, index:usize) -> Option<Rc<U>>{
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
 
  /// # `extract_element_something`
  /// Extracts something element from a key, therefore, removes it of the elements of the key in the HashMap of some types of elements.
  /// 
  /// # Arguments
  /// * `key:&T` - Key to extract the element.
  /// * `index: usize` - index to extract.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key1".to_string(), 20);
  /// assert_eq!(map.extract_element_something(&"key1".to_string(), 0), Some(10));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `None` If the index are greater than the queue of the key, or if the key does not exists.
  /// * `Some(F)` The ownership of the element.
  pub fn extract_element_something(&mut self, key:&T, index:usize) -> Option<F>{
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
  /// Returns the last element in the key and removes it.
  /// 
  /// # Arguments 
  /// * `key:&T` - Key to extract the element.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"element1".to_string());
  /// map.insert(&"key1".to_string(), &"element2".to_string());
  /// assert_eq!(map.lifo(&"key1".to_string()), Some("element2".to_string()));
  /// 
  /// ```  
  /// # Returns
  ///  * `None` - If the key does not exist or the element does not exist.
  ///  * `Some(U)` - The ownership of the element.
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
  /// Returns the last element in the key and removes it.
  /// 
  /// # Arguments 
  /// * `key:&T` - Key to extract the element.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use std::rc::Rc;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "element1".to_string());
  /// map.insert_ref(&"key1".to_string(), "element2".to_string());
  /// assert_eq!(map.lifo_ref(&"key1".to_string()), Some(Rc::new("element2".to_string())));
  /// 
  /// ```
  /// 
  /// # Returns
  ///  * `None` - If the key does not exist or the element does not exist.
  ///  * `Some(Rc<U>)` - The ownership of the element.
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
  /// Returns the last element in the key and removes it.
  /// 
  /// # Arguments
  /// * `key:&T` - Key to extract the element.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key1".to_string(), 20);
  /// assert_eq!(map.lifo_something(&"key1".to_string()), Some(20));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `None` if the key does not exist or the element does not exist.
  /// * `Some(F)` The ownership of the element.
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
  /// Shows the next element in the queue of the key.
  /// 
  /// # Arguments
  /// * `key: &T` - Key for search the element.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"element1".to_string());
  /// map.insert(&"key1".to_string(), &"element2".to_string());
  /// assert_eq!(map.peek(&"key1".to_string()), Some(&"element2".to_string()));
  /// 
  /// ```
  /// 
  /// # Returns 
  /// * `None` - If does not exist the key or does not exist any element.
  /// * `Some(&U)` - A reference to the next element in the queue.
 pub fn peek(&self, key:&T)->Option<&U>{
    if let Some(next) = self.hash.get(key){
      if next.len() > 1{
      return Some(&next[1]);
      }else {return None;}
    }else {return None;}
 }
 
  /// # `peek_ref`
  /// Shows the next element in the queue of the key.
  /// 
  /// # Arguments
  /// * `key: &T` - Key for search the element.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "element1".to_string());
  /// map.insert_ref(&"key1".to_string(), "element2".to_string());
  /// assert_eq!(map.peek_ref(&"key1".to_string()), Some(&"element2".to_string()));
  /// 
  /// ```
  /// 
  /// # Returns 
  /// * `None` - If does not exist the key or does not exist any element.
  /// * `Some(&U)` - A reference to the next element in the queue.
 pub fn peek_ref(&self, key:&T)->Option<&U>{
    if let Some(next) = self.hash_ref.get(key){
      if next.len() > 1 {return Some(&next[1]);}
      else {return None;}
    }else {return None;}
 }
 
  /// # `peek_something`
  /// Shows the next element in the queue of the key.
  /// 
  /// # Arguments
  /// * `key: &T` - Key for search the element.
  /// 
  /// # Example 
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key1".to_string(), 20);
  /// assert_eq!(map.peek_something(&"key1".to_string()), Some(&20));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `None` - If does not exist the key or does not exist any element.
  /// * `Some(&F)` - A references to the next element in the queue of that key.
  pub fn peek_something(&self, key: &T)-> Option<&F>{
     if let Some(next) = self.hash_something.get(key){
      if next.len() > 1 {return Some(&next[1]);}
      else {return None;}
    }else {return None;}
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `clear_hash`
  /// Clears the HashMap of copies of all the keys.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key2".to_string(), &"value2".to_string());
  /// map.clear_hash();
  /// assert_eq!(map.len(), 0);
  /// 
  /// ```
  pub fn clear_hash(&mut self){
   self.hash.clear();
 }
  
  /// # `clear_hash_ref`
  /// Clears the HashMap of references of all the keys.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key2".to_string(), "value2".to_string());
  /// map.clear_hash_ref();
  /// assert_eq!(map.len_ref(), 0);
  /// 
  /// ```
  pub fn clear_hash_ref(&mut self){
   self.hash_ref.clear();
 }
  
  /// # `clear_hash_something`
  /// Clears the HashMap of some types of elements of all the keys.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key2".to_string(), 20);
  /// map.clear_hash_something();
  /// assert_eq!(map.len_something(), 0);
  /// 
  /// ```
  pub fn clear_hash_something(&mut self){
      self.hash_something.clear();
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `iter_mut` 
  /// Returns a mutable iterator over the HashMap of copies.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use std::collections::hash_map::IterMut;
  /// use PTHome::main_code::utilities::general;  
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key2".to_string(), &"value2".to_string());
  /// let mut iter: IterMut<'_, String, VecDeque<String>> = map.iter_mut();
  /// assert_eq!(iter.next(), Some((&"key2".to_string(), &mut VecDeque::from(vec!["value2".to_string()]))));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `IterMut<'_, T, VecDeque<U>>` - The mutable iterator over the HashMap of copies.
  /// 
  /// ## Notes
  /// - The order of the iteration is arbitrary (non-deterministic), therefore, you should take the `example` as a demonstration of how the iteration works.
  /// - The test might fail.
  pub fn iter_mut(&mut self)-> IterMut<'_, T, VecDeque<U>>{
    self.hash.iter_mut()
  }
  
  /// # `iter_mut_ref`
  /// Returns a mutable iterator over the HashMap of references.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use std::collections::hash_map::IterMut;
  /// use std::rc::Rc;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key2".to_string(), "value2".to_string());
  /// let mut iter: IterMut<'_, String, VecDeque<Rc<String>>> = map.iter_mut_ref();
  /// assert_eq!(iter.next(), Some((&"key2".to_string(), &mut VecDeque::from(vec![Rc::new("value2".to_string())]))));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `IterMut<'_, T, VecDeque<Rc<U>>>` - The mutable iterator over the HashMap of references.
  /// 
  /// ## Notes
  /// - The order of the iteration is arbitrary (non-deterministic), therefore, you should take the `example` as a demonstration of how the iteration works.
  /// - The test might fail.
  pub fn iter_mut_ref(&mut self)->IterMut<'_, T, VecDeque<Rc<U>>>{
    self.hash_ref.iter_mut()
  }

  /// # `iter_mut_something`
  /// Returns a mutable iterator over the HashMap of some types of elements.
  /// 
  /// # Example 
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use std::collections::hash_map::IterMut;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key2".to_string(), 20);
  /// let mut iter: IterMut<'_, String, VecDeque<i32>> = map.iter_mut_something();
  /// assert_eq!(iter.next(), Some((&"key2".to_string(), &mut VecDeque::from(vec![20]))));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `IterMut<'_, T, VecDeque<F>>` - The mutable iterator over the HashMap of some types of elements.
  /// 
  /// ## Notes
  /// - The order of the iteration is arbitrary (non-deterministic), therefore, you should take the `example` as a demonstration of how the iteration works.
  /// - The test might fail.
  pub fn iter_mut_something(&mut self)->IterMut<'_, T, VecDeque<F>>{
    self.hash_something.iter_mut()
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `iter`
  /// Returns an iterator over the HashMap of copies.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use std::collections::hash_map::Iter;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key2".to_string(), &"value2".to_string());
  /// let mut iter: Iter<'_, String, VecDeque<String>> = map.iter();
  /// assert_eq!(iter.next(), Some((&"key2".to_string(), &VecDeque::from(vec!["value2".to_string()]))));
  /// 
  /// ```
  ///   
  /// # Returns
  /// * `Iter<'_, T, VecDeque<U>>` - The iterator over the HashMap of copies.
  /// 
  /// ## Notes
  /// - The order of the iteration is arbitrary (non-deterministic), therefore, you should take the `example` as a demonstration of how the iteration works.
  /// - The test might fail.
  pub fn iter(&self)->Iter<'_, T, VecDeque<U>>{
    self.hash.iter()
  }

  /// # `iter_ref`
  /// Returns an iterator over the HashMap of references.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use std::collections::hash_map::Iter;
  /// use std::rc::Rc;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key2".to_string(), "value2".to_string());
  /// let mut iter: Iter<'_, String, VecDeque<Rc<String>>> = map.iter_ref();
  /// assert_eq!(iter.next(), Some((&"key2".to_string(), &VecDeque::from(vec![Rc::new("value2".to_string())]))));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `Iter<'_, T, VecDeque<Rc<U>>>` - The iterator over the HashMap of references.
  /// 
  /// ## Notes
  /// - The order of the iteration is arbitrary (non-deterministic), therefore, you should take the `example` as a demonstration of how the iteration works.
  /// - The test might fail.
  pub fn iter_ref(&self)->Iter<'_, T, VecDeque<Rc<U>>>{
    self.hash_ref.iter()
  }
    
  /// # `iter_something`
  /// Returns an iterator over the HashMap of some types of elements.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use std::collections::VecDeque;
  /// use std::collections::hash_map::Iter;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key2".to_string(), 20);
  /// let mut iter: Iter<'_, String, VecDeque<i32>> = map.iter_something();
  /// assert_eq!(iter.next(), Some((&"key1".to_string(), &VecDeque::from(vec![10]))));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `Iter<'_, T, VecDeque<F>>` - The Iterator over the HashMap of some types of elements.
  /// 
  /// ## Notes
  /// - The order of the iteration is arbitrary (non-deterministic), therefore, you should take the `example` as a demonstration of how the iteration works.
  /// - The test might fail.
  pub fn iter_something(&self)->Iter<'_, T, VecDeque<F>>{
    self.hash_something.iter()
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `get_mut_element`
  /// Gets a mutable reference for a specific element in the queue of the key in the HashMap of copies.
  /// 
  /// # Arguments
  /// * `key: &T` - Key for search the element.
  /// * `index:usize` - Index of the element.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"element1".to_string());
  /// map.insert(&"key1".to_string(), &"element2".to_string());
  /// if let Some(elem) = map.get_mut_element(&"key1".to_string(), 1){
  ///      *elem = "modified_element2".to_string();
  /// }
  /// assert_eq!(map.get_element(&"key1".to_string(), 1), Some(&"modified_element2".to_string()));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `None` - If the key does not exist or index are greater than of the queue length.
  /// * `Some(&mut U)` - A mutable reference of the element.
  pub fn get_mut_element(&mut self, key:&T, index:usize)->Option<&mut U>{
    if let Some(n) = self.hash.get_mut(key){
      return n.get_mut(index);
    }else {return None;}
  }
  
  /// # `get_mut_element_ref`
  /// Gets a mutable reference for a specific element in the queue of the key in the HashMap of refs.
  /// 
  /// # Arguments
  /// * `key: &T` - Key for search the element.
  /// * `index:usize` - Index of the element.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use std::rc::Rc;
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "element1".to_string());
  /// map.insert_ref(&"key1".to_string(), "element2".to_string());
  /// if let Some(elem) = map.get_mut_element_ref(&"key1".to_string(), 1){
  ///    *elem = Rc::new("modified_element2".to_string());
  /// }
  /// assert_eq!(map.get_element_ref(&"key1".to_string(), 1), Some(&"modified_element2".to_string()));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `None` - If the key does not exist or index are greater than of the queue length.
  /// * `Some(&mut Rc<U>)` - A mutable reference of the element.
  pub fn get_mut_element_ref(&mut self, key:&T, index:usize)->Option<&mut Rc<U>>{
    if let Some(n) = self.hash_ref.get_mut(key){
      return n.get_mut(index);
    }else {return None;}
  }

  /// # `get_mut_element_something`
  /// Gets a mutable reference for a specific element in the queue of the key in the HashMap of random elements.
  /// 
  /// # Arguments
  /// * `key: &T` - Key for search the element.
  /// * `index:usize` - Index of the element.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key1".to_string(), 20);
  /// if let Some(elem) = map.get_mut_element_something(&"key1".to_string(), 1){
  ///    *elem = 30;
  /// }
  /// assert_eq!(map.get_element_something(&"key1".to_string(), 1), Some(&30));
  /// 
  /// ```
  /// 
  /// # Returns 
  /// * `None` - If the key does not exist or index are greater than of the queue length.
  /// * `Some(&mut F)` - A mutable reference of the element.
  pub fn get_mut_element_something(&mut self, key:&T, index:usize)->Option<&mut F>{
    if let Some(n) = self.hash_something.get_mut(key){
      return n.get_mut(index);
    }else {return None;}
  }
  //----------------------------------------------

  //----------------------------------------------
  /// # `remove_element`
  /// Removes a specific element in the queue of the key in the HashMap of copies. (starts in 0)
  /// 
  /// # Arguments
  /// * `key:&T` - Key for search the element.
  /// * `index:usize` - Index of the element to be removed.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"element1".to_string());
  /// map.insert(&"key1".to_string(), &"element2".to_string());
  /// map.remove_element(&"key1".to_string(), 0);
  /// assert_eq!(map.get_element(&"key1".to_string(), 0), Some(&"element2".to_string()));
  /// 
  /// ```
  pub fn remove_element(&mut self, key:&T, index:usize){
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

  /// # `remove_element_from_ref`
  /// Removes a specific element in the queue of the key in the HashMap of refs. (starts in 0)
  /// 
  /// # Arguments
  /// * `key:&T` - Key for search the element.
  /// * `index:usize` - Index of the element to be removed.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "element1".to_string());
  /// map.insert_ref(&"key1".to_string(), "element2".to_string());
  /// map.remove_element_from_ref(&"key1".to_string(), 0);
  /// assert_eq!(map.get_element_ref(&"key1".to_string(), 0), Some(&"element2".to_string()));
  /// 
  /// ```
  pub fn remove_element_from_ref(&mut self, key:&T, index:usize){
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

  /// # `remove_element_from_something`
  /// Removes a specific element in the queue of the key in the HashMap of some types of elements. (starts in 0)
  /// 
  /// # Arguments
  /// * `key:&T` - Key for search the element.
  /// * `index:usize` - Index of the element to be removed.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key1".to_string(), 20);
  /// map.remove_element_from_something(&"key1".to_string(), 0);
  /// assert_eq!(map.get_element_something(&"key1".to_string(), 0), Some(&20));
  /// 
  /// ```
  pub fn remove_element_from_something(&mut self, key:&T, index:usize){
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
  /// Resets all the struct to the orignial values, like a new instance of the struct.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.reset_all();
  /// assert_eq!(map.is_empty(), true);
  /// 
  /// ``` 
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
  /// Indicates if the HashMaps in the struct are empty (excluding the HashMaps used to save the insertion order).
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// assert_eq!(map.is_empty(), true);
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `true` - If all the HashMaps are empty.
  /// * `false` - If some HashMap has some element.
  pub fn is_empty(&self)->bool{
      return self.hash.is_empty() && self.hash_ref.is_empty() && self.hash_something.is_empty(); 
  }
  
  /// # `clear`
  /// Clears all the HashMaps in the struct (including the HashMaps used to save the insertion order) and reset the insertions counter.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.clear();
  /// assert_eq!(map.is_empty(), true);
  /// 
  /// ```
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
  /// Gets the length of the HashMap of copies.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert(&"key2".to_string(), &"value2".to_string());
  /// assert_eq!(map.len(), 2);
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `usize` - The length of the HashMap of copies.
  pub fn len(&self)->usize{
    self.hash.len()
  }
  
  /// # `len_ref`
  /// Gets the length of the HashMap of references.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_ref(&"key1".to_string(), "value1".to_string());
  /// map.insert_ref(&"key2".to_string(), "value2".to_string());
  /// assert_eq!(map.len_ref(), 2);
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `usize` - The length of the HashMap of references.
  pub fn len_ref(&self)->usize{
    self.hash_ref.len()
    }

  /// # `len_something`
  /// Gets the length of the HashMap of some types of elements.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert_something(&"key1".to_string(), 10);
  /// map.insert_something(&"key2".to_string(), 20);
  /// assert_eq!(map.len_something(), 2);
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `usize` - Length of the HashMap of some types of elements.
  pub fn len_something(&self)->usize{
    self.hash_something.len()
    }
  
  /// # `total_len`
  /// Gets the length of the all HashMaps (excluding HashMaps used for save the insertion order).
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert_ref(&"key2".to_string(), "value2".to_string());
  /// map.insert_something(&"key3".to_string(), 10);  
  /// assert_eq!(map.total_len(), 3);
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `usize` - Total length of all the HashMaps.
  pub fn total_len(&self)->usize{
    self.hash.len() + self.hash_ref.len() + self.hash_something.len()
    }
  
  /// # `order_len`
  /// Gets the length of the HashMaps used to save the insertion order.
  /// 
  /// # Example
  /// ```rust
  /// 
  /// use PTHome::main_code::utilities::general;
  /// let mut map:general::Map<String, String, i32> = general::Map::new();
  /// map.enable_global_order(false, false);
  /// map.insert(&"key1".to_string(), &"value1".to_string());
  /// map.insert_ref(&"key2".to_string(), "value2".to_string());
  /// map.insert_something(&"key3".to_string(), 10);
  /// assert_eq!(map.order_len(), (3, 3));
  /// 
  /// ```
  /// 
  /// # Returns
  /// * `(usize, usize)` - A tuple with the lengths of both HashMaps.
  /// * `usize` - Length of the HashMap that stores keys with their insertion numbers .
  /// * `usize` - Length of the HashMap that stores insertion numbers with their keys.
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

/// # Module `remove_comments`
/// Provides functions to remove comments from a string.   
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
    /// * `manage_close: bool` - Check the close of the ignore_content_between tuple 
    /// # Return
    /// Returns an `Option<String>`:
    /// * `Some(String)` - If the simple comments were successfully removed, returns `Some(new_content)`.
    /// * `None` - If there is an error, returns `None` with an error message.
    /// # Example
    /// ```rust 
    /// 
    /// use PTHome::main_code::utilities::remove_comments;
    /// let content = "let x = 10; /*H*/ // This is a comment /*Hello*/\nlet y = 20; // Another comment \"Hello\"";
    /// let delimiter = "//";
    /// let ignore_content_between = ( &vec!['"', '\''], &vec!["/*", "*/"] );
    /// let scape_characters = vec!['\\'];
    /// let manage_close = true;
    /// let result = remove_comments::simple_comments(content, delimiter, ignore_content_between, &scape_characters, manage_close);
    /// assert_eq!(result, Some("let x = 10; /*H*/ \nlet y = 20; \n".to_string()));
    /// 
    /// ```
    /// # Errors
    /// If content or delimiter is empty go to panic.
    /// ## Notes
    /// - The function will remove everything after the first occurrence of the comment delimiter in each line even if that content is inside ignore delimiters.
    /// - The function sometimes can add a void line in the processed content.
    
     pub fn simple_comments(content: &str, delimiter: &str, ignore_content_between: (&Vec<char>, &Vec<&str>), scape_characters:&Vec<char>,manage_close: bool)-> Option<String>{
       use crate::main_code::utilities::general;
        println!("REMOVING SIMPLE COMMENTS FROM CONTENT: {}", content);
        // Check the input delimiters.
        // They cannot has space 'cause this is a reserved character used in the processing of the content to ignore. 
        if delimiter.is_empty() || delimiter.contains(" "){
            panic!("Error: The delimiter cannot be an empty string or contains a space (' ').");
        }
        if content.is_empty(){
          panic!("Error: The content cannot be an empty string.");
        }
        // The comprobation of the delimiters (scape characters, delimiters for ignore content and the simple comment delimiter).
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

    /// process the line with comment delimiters, management the secuence
    /// # Arguments
    /// * `delimiters_array_char: &Vec<char>` - Array of chars to indicate pairs that indicate a start and end delimiter of a conent must be are ignored (can be empty)
    /// * `delimiters_array_str: &Vec<&str>` - Array of Strings to indicate pairs that indicate a start and end delimiter of a conent must be are ignored (can be empty)
    /// * `scape_characters:&Vec<char>` - A vector of chars for define the scape characters for ignore end delimiters  (can be empty)
    /// * `delimiter:&str` - comment delimiter
    /// * `line: &str` - line to process
    /// # Return
    /// A tuple with 3 elements 
  
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
    /// * `Err(1)` - If some parameter are corrupted
    /// * `Err(2)` - If there is a ignore content without close
    /// * `Err(3)` - If there is a block comment without an end delimiter.
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
    /// # `RemovalError`
    /// Enum para representar errores durante la eliminación de comentarios.
    /// # Variants
    /// * `InvalidDelimiters` - Indica que los delimitadores de inicio y fin son iguales o inválidos.
    /// * `UnclosedComment { line: usize, depth: usize }` - Indica que un comentario de bloque no fue cerrado correctamente. Contiene el número de línea donde comenzó el comentario y la profundidad de anidamiento.
    /// * `UnclosedIgnore { line: usize, expected: String }` - Indica que un contenido ignorado no fue cerrado correctamente. Contiene el número de línea donde comenzó el contenido ignorado y el delimitador esperado para cerrarlo.
    pub enum RemovalError {
    InvalidDelimiters,
    UnclosedComment { line: usize, depth: usize },
    UnclosedIgnore { line: usize, expected: String },
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
    ///   - `1` - If the start and end delimiters are the same or content vector is empty.
    ///   - `2` - If the block comment are not closed and arrive to the end of content, with an error message indicating the line number and content of the line.
    ///   - `3` - If some ignore start delimiter are not closed and arrive to the end of the content.
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

        
          match manage_close{
          ManageClose::Both=>{
               // if some ignore are open after process all the file, print an error
              if in_ignore{
                println!("Error in the line: '{}': '{}'. missing close delimiter: {}", line_num, line_content, delimiter_expected);
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
                println!("Error in the line: '{}': '{}'. missing close delimiter: {}", line_num, line_content, delimiter_expected);
                return Err(1);
              }
          }, 
          ManageClose::None=>{},
          _ => {panic!("¡FATAL ERROR!: The enum can be 'Ignore', 'Comment' or 'Both'");},
         };

         return Ok(new_content);             
    }

//------------------------------------------------------------------------------------------
    /// # `LexerState`
    /// Estados de la máquina de estados del lexer de comentarios.
    /// 
    /// ## Transiciones de Estado
    /// ```text
    /// Code --[inicio string]--> InString --[fin string]--> Code
    ///   |                                                     ^
    ///   +--[inicio /*]---> InComment --[fin */]-------------+
    ///                         |                              |
    ///                         +--[inicio /*]-->(depth++)-----+
    /// ```
    /// 
    /// ## Principio de Funcionamiento
    /// En lugar del enfoque anterior (buscar índices y emparejarlos después), 
    /// esta máquina de estados rastrea el contexto DURANTE el escaneo:
    /// - Code: Escribimos todo al output, buscamos inicio de comentario/string
    /// - InComment: NO escribimos al output, rastreamos profundidad de anidamiento
    /// - InString: Escribimos al output, ignoramos delimitadores de comentario
    #[derive(Debug, Clone)]
    enum LexerState {
        /// Procesando código normal
        Code,
        /// Dentro de un comentario de bloque con profundidad de anidamiento
        InComment { depth: usize },
        /// Dentro de un string literal (contenido a ignorar)
        InString { delimiter: char },
    }

//------------------------------------------------------------------------------------------
    /// # `CommentLexer`
    /// Lexer for removing block comments with dynamic delimiters and with O(n) time complexity and O(1) spacial complexity.
    /// 
    /// # Fields
    /// * `chars` - An iterator over the characters of the input string.
    /// * `state` - The current state of the lexer (Code, InComment, InString).
    /// * `output` - The resulting string after comment removal.
    /// * `delimiter_start` - The starting delimiter for block comments.
    /// * `delimiter_end` - The ending delimiter for block comments.
    /// * `ignore_delimiters` - A vector of characters that mark the start/end of content to ignore (e.g., string delimiters).
    /// * `escape_chars` - A vector of characters that escape the next character.
    /// * `line_num` - The current line number.
    /// * `line_start` - The starting index of the current line for error reporting.
    struct CommentLexer<'a> {
        chars: std::iter::Peekable<std::str::Chars<'a>>,
        state: LexerState,
        output: String,
        delimiter_start: &'a str,
        delimiter_end: &'a str,
        ignore_delimiters: &'a Vec<String>,
        ignore_del_len: Vec<usize>,
        escape_chars: &'a Vec<char>,
        line_num: usize,
        line_start: usize,
    }

    impl<'a> CommentLexer<'a> {
        /// Crea un nuevo CommentLexer con delimitadores dinámicos
        /// 
        /// # Argumentos
        /// * `input` - El texto a procesar
        /// * `delimiter_start` - Delimitador de inicio de comentario (ej: "/*")
        /// * `delimiter_end` - Delimitador de fin de comentario (ej: "*/")
        /// * `ignore_delimiters` - Caracteres que marcan inicio/fin de contenido a ignorar
        /// * `escape_chars` - Caracteres de escape que invalidan el siguiente caracter
        fn new(
            input: &'a str, 
            delimiter_start: &'a str, 
            delimiter_end: &'a str,
            ignore_delimiters: &'a Vec<String>,
            escape_chars: &'a Vec<char>
        ) -> Self {
          let mut j = 0;
          let mut ignore_del_len = Vec::new();
          while j <= ignore_delimiters.len(){
            if j < ignore_delimiters.len(){
              ignore_del_len.push((ignore_delimiters[j].len(), j));
            }
            j +=1; 
          }
            CommentLexer {
                chars: input.chars().peekable(),
                state: LexerState::Code,
                output: String::with_capacity(input.len()),
                delimiter_start,
                delimiter_end,
                ignore_delimiters,
                ignore_del_len,
                escape_chars,
                line_num: 1,
                line_start: 0,
            }
        }

        /// Procesa el input completo y retorna el resultado
        fn process(&mut self) -> Result<String, RemovalError> {
            while let Some(c) = self.chars.next() {
                if c == '\n' {
                    self.line_num += 1;
                }

                match &self.state {
                    LexerState::Code => self.handle_code(c)?,
                    LexerState::InComment { depth } => {
                        let depth = *depth;
                        self.handle_comment(c, depth)?
                    },
                    LexerState::InString { delimiter } => {
                        let delim = *delimiter;
                        self.handle_string(c, delim)?
                    },
                }
            }

            self.finalize()
        }

        /// Maneja caracteres cuando estamos en código normal
        fn handle_code(&mut self, c: char) -> Result<(), RemovalError> {
            // Detectar inicio de contenido a ignorar (strings, etc.)
            let mut buffer = String::new();
            let mut j = 0;
            if !self.ignore_delimiters.is_empty(){
             while buffer.len() <= self.ignore_del_len[j] && j <= self.ignore_del_len.len(){
              buffer.push(c);
             }
              
            if self.ignore_delimiters.contains(&c.to_string()) {
                self.state = LexerState::InString { delimiter: c };
                self.output.push(c);
                return Ok(());
            }
          }
            // Detectar inicio de comentario
            if c == self.delimiter_start.chars().nth(0).unwrap() {
                if self.matches_delimiter_start() {
                    self.consume_delimiter(self.delimiter_start);
                    self.state = LexerState::InComment { depth: 1 };
                    self.output.push(' '); // Preservar espacio donde estaba el comentario
                    return Ok(());
                }
            }

            // Si no es ninguno de los anteriores, escribir al output
            self.output.push(c);
            Ok(())
        }

        /// Maneja caracteres cuando estamos dentro de un comentario
        fn handle_comment(&mut self, c: char, depth: usize) -> Result<(), RemovalError> {
            // Detectar inicio de comentario anidado
            if c == self.delimiter_start.chars().nth(0).unwrap() {
                if self.matches_delimiter_start() {
                    self.consume_delimiter(self.delimiter_start);
                    self.state = LexerState::InComment { depth: depth + 1 };
                    return Ok(());
                }
            }

            // Detectar fin de comentario
            if c == self.delimiter_end.chars().nth(0).unwrap() {
                if self.matches_delimiter_end() {
                    self.consume_delimiter(self.delimiter_end);
                    if depth == 1 {
                        self.state = LexerState::Code;
                    } else {
                        self.state = LexerState::InComment { depth: depth - 1 };
                    }
                    return Ok(());
                }
            }

            // Dentro del comentario, no escribimos nada
            Ok(())
        }

        /// Maneja caracteres cuando estamos dentro de un string
        fn handle_string(&mut self, c: char, delimiter: char) -> Result<(), RemovalError> {
            self.output.push(c);
            if self.escape_chars.contains(&c)
            if c == '\\' {
                if let Some(&next_char) = self.chars.peek() {
                    self.chars.next();
                    self.output.push(next_char);
                    if next_char == '\n' {
                        self.line_num += 1;
                    }
                }
                return Ok(());
            }

            // Detectar fin de string
            if c == delimiter {
                self.state = LexerState::Code;
            }

            Ok(())
        }

        /// Verifica si los siguientes caracteres coinciden con delimiter_start
        fn matches_delimiter_start(&mut self) -> bool {
            let start_chars: Vec<char> = self.delimiter_start.chars().collect();
            if start_chars.len() == 1 {
                return true;
            }

            let mut peek_iter = self.chars.clone();
            for i in 1..start_chars.len() {
                if let Some(ch) = peek_iter.next() {
                    if ch != start_chars[i] {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        }

        /// Verifica si los siguientes caracteres coinciden con delimiter_end
        fn matches_delimiter_end(&mut self) -> bool {
            let end_chars: Vec<char> = self.delimiter_end.chars().collect();
            if end_chars.len() == 1 {
                return true;
            }

            let mut peek_iter = self.chars.clone();
            for i in 1..end_chars.len() {
                if let Some(ch) = peek_iter.next() {
                    if ch != end_chars[i] {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        }

        /// Consume los caracteres restantes de un delimitador multi-caracter
        fn consume_delimiter(&mut self, delimiter: &str) {
            for _ in 1..delimiter.len() {
                self.chars.next();
            }
        }

        /// Finaliza el procesamiento y verifica el estado
        fn finalize(&self) -> Result<String, RemovalError> {
            match &self.state {
                LexerState::InComment { depth } => {
                    Err(RemovalError::UnclosedComment {
                        line: self.line_num,
                        depth: *depth,
                    })
                }
                _ => Ok(self.output),
            }
        }
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
       // Validaciones básicas
       if content.is_empty(){
        println!("Error: The content vector is empty");
        return Err(-1);
       }
       if delimiter_start == delimiter_end{
        println!("Error: The start and end delimiters are the same.");
        return Err(-1);
       }
       
       // Construir vector de delimitadores de ignore desde char y str vectors
       let mut all_ignore_delimiters: Vec<char> = ignore_content_between.0.clone();
       // Por ahora solo tomamos los chars, las strings multi-caracter requieren 
       // lógica adicional que se puede agregar en futuras iteraciones
       
       let mut lexer = CommentLexer::new(
           content, 
           delimiter_start, 
           delimiter_end,
           &all_ignore_delimiters,
           scape_characters
       );
       
       let mut lexer = CommentLexer::new(content, delimiter_start, delimiter_end);
       
       match lexer.process() {
           Ok(result) => Ok(result),
           Err(RemovalError::UnclosedComment { line, depth }) => {
               println!("Error: Block comment without end delimiter at line {}\n MISSING COMMENTS TO CLOSE: {}", line, depth);
               Err(2)
           },
           Err(RemovalError::InvalidDelimiters) => {
               println!("Error: Invalid delimiters");
               Err(-1)
           },
           Err(RemovalError::UnclosedIgnore { line, expected }) => {
               println!("Error in the line: {}. missing close delimiter: {}", line, expected);
               Err(1)
           }
       }
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
  /// ## Notes
  /// - Use in internal functions, not recommended edit
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
        let scape:Vec<char> = vec!['\\']; 
       let vec_str:Vec<&str> = vec!["'", "'"];
       let vec_char:Vec<char> = vec![];
       let ignore = (&vec_char, &vec_str); 
       assert_eq!("Not remove this 'this is a string// \\'' \nother \n".to_string(), super::simple_comments(str, "//", ignore, &scape, true).unwrap());
      }

      #[test]
      /// # [`super::simple_comments`] Test 3
      /// test where the ignore delimiter is not closed, expect an error
      fn test_3_simple_comments(){
        let str = "Not remove this 'this is a string// \\' //remove this\nother // abcdefghijklm";
        let scape:Vec<char> = vec!['\\']; 
       let vec_str:Vec<&str> = vec!["'", "'"];
       let vec_char:Vec<char> = vec![];
       let ignore = (&vec_char, &vec_str);
       assert_eq!(None, super::simple_comments(str, "//", ignore, &scape, true));
      }

      #[test]
      /// # [`super::simple_comments`] Test 4
      /// test where the ignore delimiter is not closed, but not manage this error
      fn test_4_simple_comments(){
        let str = "Not remove this 'this is a string// \\' //remove this\nother // abcdefghijklm";
        let scape:Vec<char> = vec!['\\']; 
       let vec_str:Vec<&str> = vec!["'", "'"];
       let vec_char:Vec<char> = vec![];
       let ignore = (&vec_char, &vec_str); 
       assert_eq!("Not remove this 'this is a string// \\' //remove this\nother // abcdefghijklm\n", super::simple_comments(str, "//", ignore, &scape, false).unwrap());
      }

      #[test]
      /// # [`super::simple_comments`] Test 5
      /// test where the ignore delimiters aren't correctly structured
      fn test_5_simple_comments(){
        let str = "Not remove this 'this is a string// \\' //remove this\nother // abcdefghijklm";
        let scape:Vec<char> = vec!['\\']; 
       let vec_str:Vec<&str> = vec!["'"];
       let vec_char:Vec<char> = vec![];
       let ignore = (&vec_char, &vec_str); 
       assert_eq!(None, super::simple_comments(str, "//", ignore, &scape, false));
      }

      #[test]
      /// # [`super::simple_comments`] Test 6
      /// test where the delimiter ignore contains space trigger an error
      fn test_6_simple_comments(){
        let str = "Not remove this 'this is a string// \\' //remove this\nother // abcdefghijklm";
        let scape:Vec<char> = vec!['\\']; 
       let vec_str:Vec<&str> = vec!["' ", "'"];
       let vec_char:Vec<char> = vec![];
       let ignore = (&vec_char, &vec_str); 
       assert_eq!(None, super::simple_comments(str, "//", ignore, &scape, true));
      }

      #[test]
      #[should_panic]
      /// # [`super::simple_comments`] Test 7
      /// test where the delimiter contains space trigger an error
      fn test_7_simple_comments(){
        let str = "Not remove this 'this is a string// \\' //remove this\nother // abcdefghijklm";
        let scape:Vec<char> = vec!['\\'];
       let vec_str:Vec<&str> = vec!["'", "'"];
       let vec_char:Vec<char> = vec![];
       let ignore = (&vec_char, &vec_str); 
       assert_eq!(None, super::simple_comments(str, "// ", ignore, &scape, true));
      }

      #[test]
      /// # [`super::simple_comments`] Test 8
      /// test where the scape character contains space trigger an error
      fn test_8_simple_comments(){
        let str = "Not remove this 'this is a string// \\' //remove this\nother // abcdefghijklm";
        let scape:Vec<char> = vec!['\\', ' '];
       let vec_str:Vec<&str> = vec!["'", "'"];
       let vec_char:Vec<char> = vec![];
       let ignore = (&vec_char, &vec_str);
       assert_eq!(None, super::simple_comments(str, "//", ignore, &scape, true));
      }

      #[test]
      /// # [`super::content_between`] Test
      fn test_content_between(){
        let str = "Not remove this 'this is a string// \\'' //remove this";
        let scape:Vec<char> = vec!['\\'];
       let vec_str:Vec<&str> = vec!["'", "'"];
       let vec_char:Vec<char> = vec![];
       assert_eq!(("".to_string(), false, "Not remove this 'this is a string// \\'' ".to_string()), super::content_between(&vec_char, &vec_str, &scape, "//", str));
      }

      #[test]
      /// # [`super::content_between`] Test 2
      /// test where the ignore delimiter is not closed
      fn test_2_content_between(){
        let str = "Not remove this 'this is a string// \\' //remove this";
        let scape:Vec<char> = vec!['\\'];
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
        let scape:Vec<char> = vec!['\\'];
       let vec_str:Vec<&str> = vec!["'", "' "];
       let vec_char:Vec<char> = vec![];
       assert_eq!(("'".to_string(), true, "Not remove this 'this is a string// \\' //remove this".to_string()), super::content_between(&vec_char, &vec_str, &scape, "", str));
      }

      #[test]
      /// # [`super::content_between`] Test 4
      /// test where the ignore delimiters aren't correctly close after the delimiter to search
      fn test_4_content_between(){
        let str = "Not remove this // 'this is a string// \\' //remove this";
        let scape:Vec<char> = vec!['\\'];
        let vec_str:Vec<&str> = vec!["'", "'"];
        let vec_char:Vec<char> = vec![];
        assert_eq!(("".to_string(), false, "Not remove this ".to_string()), super::content_between(&vec_char, &vec_str, &scape, "//", str));
      }
      #[test]
      /// # [`super::block_comments`] Test 
      fn test_block_comments(){
        let str = "Code before /* This is a block comment \n that spans multiple lines */ Code after";
        let scape:Vec<char> = vec!['\\'];
        let vec_str:Vec<&str> = vec!["'", "'"];
        let vec_char:Vec<char> = vec![];
        let ignore = (&vec_char, &vec_str);
        assert_eq!("Code before  Code after\n".to_string(), super::block_comments(str, "/*", "*/", ignore, &scape, ModeBlock::Single,ManageClose::Both).unwrap());
      }

      #[test]
      /// # [`super::block_comments`] Test 2
      /// test where the block comment is not closed (expect an error)
      fn test_2_block_comments(){
        let str = "Code before /* This is a block comment \n that spans multiple lines Code after";
        let scape:Vec<char> = vec!['\\'];
        let vec_str:Vec<&str> = vec!["'", "'"];
        let vec_char:Vec<char> = vec![];
        let ignore = (&vec_char, &vec_str);
        assert_eq!(Err(2), super::block_comments(str, "/*", "*/", ignore, &scape, ModeBlock::Single,ManageClose::Both));
      }

      #[test]
      /// # [`super::block_comments`] Test 3
      /// test where the ignore delimiter is not closed (expect an error)
      fn test_3_block_comments(){
        let str = "Code before /* This is a block comment \n that spans*/ 'multiple lines */ Code after";
        let scape:Vec<char> = vec!['\\'];
        let vec_str:Vec<&str> = vec!["'", "'"];
        let vec_char:Vec<char> = vec![];
        let ignore = (&vec_char, &vec_str);
        assert_eq!(Err(1), super::block_comments(str, "/*", "*/", ignore, &scape, ModeBlock::Single,ManageClose::Both));
      }

      #[test]
      /// # [`super::block_comments`] Test 4
      /// test where both the block comment and the ignore delimiter are not closed (expect an error)
      /// In this case, the function should prioritize reporting the ignore delimiter not closed error
      fn test_4_block_comments(){
        let str = "Code before /* This is a block comment \n that spans 'multiple lines */ Code after";
        let scape:Vec<char> = vec!['\\'];
        let vec_str:Vec<&str> = vec!["'", "'"];
        let vec_char:Vec<char> = vec![];
        let ignore = (&vec_char, &vec_str);
        assert_eq!(Err(1), super::block_comments(str, "/*", "*/", ignore, &scape, ModeBlock::Single,ManageClose::Both));
      }

      #[test]
      /// # [`super::block_comments`] Test 5
      /// test where both the block comment and the ignore delimiter are not closed (expect an error)
      /// In this case, the function should prioritize reporting the ignore delimiter error
      fn test_5_block_comments(){
        let str = "Code before /* This is a block comment \n that spans*/ 'multiple lines /*Code after";
        let scape:Vec<char> = vec!['\\'];
        let vec_str:Vec<&str> = vec!["'", "'"];
        let vec_char:Vec<char> = vec![];
        let ignore = (&vec_char, &vec_str);
        assert_eq!(Err(1), super::block_comments(str, "/*", "*/", ignore, &scape, ModeBlock::Single,ManageClose::Both));
      }

      #[test]
      /// # [`super::block_comments`] Test 6
      /// test where the block comment is nested but mode is single
      fn test_6_block_comments(){
        let str = "Code before /* This is a block comment /* nested comment */ still in comment */ Code after";
        let scape:Vec<char> = vec!['\\'];
        let vec_str:Vec<&str> = vec!["'", "'"];
        let vec_char:Vec<char> = vec![];
        let ignore = (&vec_char, &vec_str);
        assert_eq!("Code before  still in comment */ Code after\n".to_string(), super::block_comments(str, "/*", "*/", ignore, &scape, ModeBlock::Single,ManageClose::Both).unwrap());
      }

      #[test]
      /// # [`super::block_comments`] Test 7
      /// test where is been alocated content between comments
      fn test_7_block_comments(){
        let str = "Code before /* This is a block comment ''/* not a comment */ still in comment */ /*Code after*/\nOther *//*comment*/ continue /*\n '*/l' closed*//*\n*/l";
        let scape:Vec<char> = vec!['\\'];
        let vec_str:Vec<&str> = vec!["'", "'"];
        let vec_char:Vec<char> = vec![];
        let ignore = (&vec_char, &vec_str);
        assert_eq!("Code before  still in comment */ \nOther */ continue l\n".to_string(), super::block_comments(str, "/*", "*/", ignore, &scape, ModeBlock::Single,ManageClose::Both).unwrap());
      }

      #[test]
      /// # [`super::block_comments`] Test 8
      /// test where does exist conflicts between end and start delimiter
      fn test_8_block_comments(){
        let str = "Code before /*/* This is a block comment /*/ nested comment */\n/* still in comment'*/'/*\n*/ Code after";
        let scape:Vec<char> = vec!['\\'];
        let vec_str:Vec<&str> = vec!["'", "'"];
        let vec_char:Vec<char> = vec![];
        let ignore = (&vec_char, &vec_str);
        assert_eq!("Code before  nested comment */\n Code after\n".to_string(), super::block_comments(str, "/*", "*/", ignore, &scape, ModeBlock::Single,ManageClose::Both).unwrap());
      }

      #[test]
      /// # [`super::block_comments`] Test 9
      /// Use the nested mode
      fn test_9_block_comments(){
        let str = "Code before /*/* This is a block comment /*/ nested comment */\nbetween/* still in comment'*/'/*\n*/ Code after*/";
        let scape:Vec<char> = vec!['\\'];
        let vec_str:Vec<&str> = vec!["'", "'"];
        let vec_char:Vec<char> = vec![];
        let ignore = (&vec_char, &vec_str);
        assert_eq!("Code before \nbetween\n", super::block_comments(str, "/*", "*/", ignore, &scape, ModeBlock::Nested,ManageClose::Both).unwrap());
      }
      #[test]
      /// # [`super::block_comments`] Test 10
      /// Use the nested mode
      fn test_10_block_comments(){
        let str = "Code before /*/* This is a block comment /*/ nested comment */bt/*\nbetween/* still in comment'*/'\n*/ Code after*/";
        let scape:Vec<char> = vec!['\\'];
        let vec_str:Vec<&str> = vec!["'", "'"];
        let vec_char:Vec<char> = vec![];
        let ignore = (&vec_char, &vec_str);
        assert_eq!("Code before bt\n", super::block_comments(str, "/*", "*/", ignore, &scape, ModeBlock::Nested,ManageClose::Both).unwrap());
      }

      #[test]
      /// # [`super::block_comments`] Test 11
      /// Use the nested mode
      fn test_11_block_comments(){
        let str = "Code before /*/* This is a block comment /*/\n nested comment '*\n/'*/bt/*\nbetween/* still in comment'*/'\n*/ Code after*/";
        let scape:Vec<char> = vec!['\\'];
        let vec_str:Vec<&str> = vec!["'", "'"];
        let vec_char:Vec<char> = vec![];
        let ignore = (&vec_char, &vec_str);
        assert_eq!("Code before bt\n", super::block_comments(str, "/*", "*/", ignore, &scape, ModeBlock::Nested,ManageClose::Both).unwrap());
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
  /// ## Notes
  /// - The properties and characteristics of a `strict formats` are in the `formats` folder at the repository where you found this crate
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
    /// ## Notes
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
      /// ## Notes
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
      /// ## Notes
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
      /// ## Notes
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
  /// ## Notes
  /// - The properties and characteristics of a `flexible formats` are in the `formats` folder at the repository where you found this crate
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
