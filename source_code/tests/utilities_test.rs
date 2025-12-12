
#[cfg(test)]
///# general_tests
/// Tests for the general utility functions and NumLines struct.
  mod general_tests{
    use PTHome::main_code::utilities::general::*;
      #[test]
       /// # [`remove_empty_lines`] Test
       ///The result expected have a '\n' at the end because within the function in each iteration push the content of the line and add '\n' on the end 
        fn test_remove_empty_lines(){
          let str= &"This is my test\n            \n The test is made for ensure the function \n \n remove_empyt_lines".to_string();
          let expected ="This is my test\n The test is made for ensure the function \n remove_empyt_lines\n".to_string();
          assert_eq!(expected, remove_empty_lines(str));
        }

      #[test]
      /// # [`NumLines::numerate_lines`] Test
       fn test_numerate_lines(){
        let new_instance = NumLines::new("This is the content''\n to numerate'\n this is the line three'\n", "");
        assert_eq!("1 This is the content''\n2  to numerate'\n3  this is the line three'\n".to_string(), new_instance.numerate_lines());
      }

      #[test]
      /// # [`NumLines::remove_num_lines`] Test
       fn test_remove_num_lines(){
        let mut new_instance = NumLines::new("This is the content''\n to numerate'\n this is the line three'\n", "-");
        let numerated = new_instance.numerate_lines();
        new_instance.set_content(&numerated);
        let expected = "This is the content''\n to numerate'\n this is the line three'\n".to_string();
        assert_eq!(expected, new_instance.remove_num_lines());
      }
      #[test]
      /// # [`NumLines::remove_num_lines`] Test 2
      /// Here we test the function with a string not numerated before and without de delimiter
      fn test_2_remove_num_lines(){
        let  new_instance = NumLines::new("This is the content''\n to numerate'\n this is the line three'\n", "-");
        //Not set the content
        let expected = "This is the content''\n to numerate'\n this is the line three'\n".to_string();
        assert_eq!(expected, new_instance.remove_num_lines());
      }
      #[test]
      /// # [`NumLines::remove_num_lines`] Test 3
      /// Here we test the function with a delimiter just like space (' '), so the function remove all after and the first delmiter appear found for each line
        fn test_3_remove_num_lines(){
        let  new_instance = NumLines::new("This is the content''\n to numerate'\n this is the line three'\n", "");
        //Not set the content
        let expected = "is the content''\nto numerate'\nthis is the line three'\n".to_string();
        assert_eq!(expected, new_instance.remove_num_lines());
      }

      #[test]
      /// # [`NumLines::skip_num_line`] Test
       fn test_skip_num_line(){
        let  new_instance = NumLines::new("This is the content''\n to numerate'\n this is the line three'\n", "");
        let numerated = new_instance.numerate_lines();
        let mut num_line_skiped = String::new();
        for line in numerated.lines(){
          num_line_skiped.push_str(&new_instance.skip_num_line(line));
          num_line_skiped.push('\n');
        }
        assert_eq!("This is the content''\n to numerate'\n this is the line three'\n".to_string(),num_line_skiped);
      }
      #[test]
      /// # [`NumLines::skip_num_line`] Test 2
      /// Here we test the function with a string without end delimiter use
       fn test_2_skip_num_line(){
        let new_instance = NumLines::new("This is the content''\n to numerate'\n this is the line three'\n", "-");
      
        let mut num_line_skiped = String::new();
        for line in new_instance.get_content().lines(){
          num_line_skiped.push_str(&new_instance.skip_num_line(line));
          num_line_skiped.push('\n');
        }
        assert_eq!("This is the content''\n to numerate'\n this is the line three'\n".to_string(),num_line_skiped);
      }

      #[test]
      /// # [`NumLines::get_num_line`] Test
       fn test_get_num_line(){
        let new_instance = NumLines::new("This is the content''\n to numerate'\n this is the line three'\n", "-");
         let numerated = new_instance.numerate_lines();
        let mut num_line:Vec<i32> = Vec::new();
        for line in numerated.lines(){
          num_line.push(new_instance.get_num_line(line));
        }
        assert_eq!([1, 2, 3].to_vec(),num_line);
      }
      #[test]
      /// # [`NumLines::get_num_line`] Test  2
      /// test the function without the content numerated before
      /// expected -1 at the end of the vector, because not found in the string a delimiter expected '-'
       fn test_2_get_num_line(){
        let new_instance = NumLines::new("1-This is the content''\n2- to numerate'\n3 this is the line three'\n", "-");
        let mut num_line:Vec<i32> = Vec::new();
        for line in new_instance.get_content().lines(){
          num_line.push(new_instance.get_num_line(line));
        }
        assert_eq!([1, 2, -1].to_vec(),num_line);
      }
      #[test]
      #[should_panic]
      /// # [`NumLines::get_num_line`] Test  3
      /// test the function without the content numerated before
      /// expected panic because can convert 'This' to i32
       fn test_3_get_num_line(){
        let new_instance = NumLines::new("This is the content''", "");
        assert_eq!(1,new_instance.get_num_line("This is the content''"));
      }

      #[test]
      /// # [`NumLines::get_content`] Test
     fn test_get_content(){
        let new_instance = NumLines::new("1-This is the content''", "");
        assert_eq!("1-This is the content''".to_string(), new_instance.get_content());
      }

      #[test]
      /// # [`NumLines::get_delimiter`] Test
       fn test_get_delimiter(){
        let new_instance = NumLines::new("1-This is the content''", "");
        assert_eq!("".to_string(), new_instance.get_delimiter());
      }
      #[test]
      /// # [`NumLines::set_content`] Test
       fn test_set_content(){
        let mut new_instance = NumLines::new("1-This is the content''", "");
        new_instance.set_content("Set To This");
        assert_eq!("Set To This".to_string(), new_instance.get_content());
      }
      
      #[test]
      /// # [`NumLines::set_delimiter`] Test
       fn test_set_delimiter(){
        let mut new_instance = NumLines::new("1-This is the content''", "");
        new_instance.set_delimiter("-");
        assert_eq!("-".to_string(), new_instance.get_delimiter());
      }

      #[test]
      /// # [`all_appears_index`] Test
     fn test_all_appears_index(){
        let vec:Vec<usize> = vec![5, 7, 10];
        assert_eq!(vec, all_appears_index("This - -  -" , "-"));
      }

      #[test]
      /// # [`str_of_n_str`] Test
       fn test_str_of_n_str(){
        assert_eq!("------".to_string(),str_of_n_str("-", 6));
      }

      #[test]
      /// # [`sub_vec`] Test
       fn test_sub_vec(){
        assert_eq!([6, 8].to_vec(), sub_vec(&[-1, 5, 6, 8].to_vec(), 3, 2));
      }
      
      #[test]
      /// # [`replace_index`] Test
       fn test_replace_index(){
        let str = "Edit This? 'hi'";
        let index = str.find("?").unwrap();
        assert_eq!("Edit This/Hello 'hi'".to_string(), replace_index(str, "/Hello", index));
      }
      #[test]
      /// # [`replace_index`] Test 2
      /// test when the index is greather than the str.len()-1
       fn test_2_replace_index(){
        let str = "Edit This? 'hi'";
        assert_eq!("Edit This? 'hi'".to_string(), replace_index(str, "/Hello", str.len()));
      }
      #[test]
      /// # [`ordered_combination`] Test
       fn test_ordered_combinations(){
        let vec = (&vec!["a".to_string(), "b".to_string(), "c".to_string()], &vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        let combs = ordered_combination(vec);
        let expected = ["aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc"].to_vec();
        assert_eq!(expected, combs);
      }

  }

   #[cfg(test)]
   /// # map_tests
   /// Tests for the general::Map struct and its methods
   /// ## IMPORTANT
   /// Some tests use HashMap internally. Because HashMap iteration order is arbitrary
   /// (non-deterministic), those tests may fail if they assume a particular order, so
   /// they are ignored by default.
  mod map_tests{
    use PTHome::main_code::utilities::general;
    use std::collections::VecDeque;
    use std::rc::Rc;
    use std::collections::HashMap;
   #[test]
   /// # [general::Map::insert] Test
   fn test_insert(){
     let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
     n.insert(&3,& "hello".to_string());
     n.insert(&3,& "world".to_string());
     n.insert(&3, &"chao".to_string());
     assert_eq!(["hello".to_string(), "world".to_string(), "chao".to_string()].to_vec(), n.get_all(&3));
   }

    #[test]
    /// # [general::Map::insert_ref] Test
   fn test_insert_ref(){
    let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
     n.insert_ref(&3, "hello".to_string());
     n.insert_ref(&3, "world".to_string());
     n.insert_ref(&3, "chao".to_string());
     assert_eq!(["hello".to_string(), "world".to_string(), "chao".to_string()].to_vec(), n.get_all_ref(&3));
   }
    #[test]
    /// # [`general::Map::insert_something`] Test
    fn test_insert_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       let mut s = VecDeque::new();
       s.push_back(Some(10));
       s.push_back(Some(20));
       s.push_back(Some(30));
       assert_eq!(s, *n.get_ref_to_all_something(&3).unwrap());
    }
       #[test]
    /// # [`general::Map::get`] Test
    fn test_get(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       assert_eq!("hello".to_string(), n.get(&3).unwrap().clone());
    }
    #[test]
    /// # [`general::Map::get`] Test 2
    /// Here we test the function when the key not exist
    fn test_2_get(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       assert_eq!(None, n.get(&5));
    }
    #[test]
    /// # [`general::Map::get`] Test 3
    /// Here we test the funciton when the vecdeque for the key is empty
    fn test_3_get(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
      let s = n.get_mut_ref_to_all(&3).unwrap();
        s.clear();
       assert_eq!(true, n.contains_key(&3));
       assert_eq!(None, n.get(&3));
       assert_eq!(true, n.contains_key(&3));
    }
    #[test]
    /// # [`general::Map::get_ref`] Test
    fn test_get_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       assert_eq!("hello".to_string(), n.get_ref(&3).unwrap().clone());
    }
    #[test]
    /// # [`general::Map::get_ref`] Test 2
    /// Here we test the function when the key not exist
    fn test_2_get_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       assert_eq!(None, n.get_ref(&5));
    }
    #[test]
    /// # [`general::Map::get_ref`] Test 3
    /// Here we test the funciton when the vecdeque for the key is empty
    fn test_3_get_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
      let s = n.get_mut_ref_to_all_ref(&3).unwrap();
        s.clear();
       assert_eq!(true, n.contains_key_ref(&3));
       assert_eq!(None, n.get_ref(&3));
       assert_eq!(true, n.contains_key_ref(&3));
    }

    #[test]
    /// # [`general::Map::get_something`] Test
    fn test_get_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       assert_eq!(Some(10), n.get_something(&3).unwrap().clone());
    }
    #[test]
    /// # [`general::Map::get_something`] Test 2
    /// Here we test the function when the key not exist
    fn test_2_get_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       assert_eq!(None, n.get_something(&5));
    }
    #[test]
    /// # [`general::Map::get_something`] Test 3
    /// Here we test the funciton when the vecdeque for the key is empty
    fn test_3_get_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
      let s = n.get_mut_ref_to_all_something(&3).unwrap();
        s.clear();
       assert_eq!(true, n.contains_key_something(&3));
       assert_eq!(None, n.get_something(&3));
       assert_eq!(true, n.contains_key_something(&3));
    }
    #[test]
    /// # [`general::Map::remove`] Test
    fn test_remove(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       n.remove(&3);
       assert_eq!("world".to_string(), n.get(&3).unwrap().clone());
    }
    #[test]
    /// # [`general::Map::remove`] Test 2
    /// Here we test the function when the key not exist
    fn test_2_remove(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       n.remove(&5);
       assert_eq!("hello".to_string(), n.get(&3).unwrap().clone());
    }
    #[test]
    /// # [`general::Map::remove`] Test 3
    /// Here we test the funciton when the key has just 1 element
    fn test_3_remove(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.remove(&3);
       assert_eq!(None, n.get(&3));
    }
    #[test]
    /// # [`general::Map::remove_ref`] Test
    fn test_remove_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       n.remove_ref(&3);
       assert_eq!("world".to_string(), n.get_ref(&3).unwrap().clone());
    }
    #[test]
    /// # [`general::Map::remove_ref`] Test 2
    /// Here we test the function when the key not exist
    fn test_2_remove_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       n.remove_ref(&5);
       assert_eq!("hello".to_string(), n.get_ref(&3).unwrap().clone());
    }
    #[test]
    /// # [`general::Map::remove_ref`] Test 3
    /// Here we test the funciton when the key has just 1 element
    fn test_3_remove_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.remove_ref(&3);
       assert_eq!(None, n.get_ref(&3));
    }
    #[test]
    /// # [`general::Map::remove_something`] Test
    fn test_remove_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       n.remove_something(&3);
       assert_eq!(Some(20), n.get_something(&3).unwrap().clone());
    }
    #[test]
    /// # [`general::Map::remove_something`] Test 2
    /// Here we test the function when the key not exist
    fn test_2_remove_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       n.remove_something(&5);
       assert_eq!(Some(10), n.get_something(&3).unwrap().clone());
    }
    #[test]
    /// # [`general::Map::remove_something`] Test 3
    /// Here we test the funciton when the key has just 1 element
    fn test_3_remove_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.remove_something(&3);
       assert_eq!(None, n.get_something(&3));
    }
    #[test]
    /// # [`general::Map::remove_all`] Test
    fn test_remove_all(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       n.remove_all(&3);
       assert_eq!(None, n.get(&3));
    }
    #[test]
    /// # [`general::Map::remove_all`] Test 2
    /// Here we test the function when the key not exist
    fn test_2_remove_all(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       n.remove_all(&5);
       assert_eq!("hello".to_string(), n.get(&3).unwrap().clone());
    }
    #[test]
    /// # [`general::Map::remove_all_ref`] Test
    fn test_remove_all_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       n.remove_all_ref(&3);
       assert_eq!(None, n.get_ref(&3));
    }
    #[test]
    /// # [`general::Map::remove_all_ref`] Test 2
    /// Here we test the function when the key not exist
    fn test_2_remove_all_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       n.remove_all_ref(&5);
       assert_eq!("hello".to_string(), n.get_ref(&3).unwrap().clone());
    }
    #[test]
    /// # [`general::Map::remove_all_something`] Test
    fn test_remove_all_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       n.remove_all_something(&3);
       assert_eq!(None, n.get_something(&3));
    }
    #[test]
    /// # [`general::Map::remove_all_something`] Test 2
    /// Here we test the function when the key not exist
    fn test_2_remove_all_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       n.remove_all_something(&5);
       assert_eq!(Some(10), n.get_something(&3).unwrap().clone());
    }
    #[test]
    /// # [`general::Map::get_all`] Test
    fn test_get_all(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       assert_eq!(["hello".to_string(), "world".to_string(), "chao".to_string()].to_vec(), n.get_all(&3));
    }
    #[test]
    /// # [`general::Map::get_all`] Test 2
    /// Here we test the function when the key not exist
    fn test_2_get_all(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       assert_eq!(Vec::<String>::new(), n.get_all(&5));
    }
    #[test]
    /// # [`general::Map::get_all`] Test 3
    /// Here we test the funciton when the vecdeque for the key is empty
    fn test_3_get_all(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
      let s = n.get_mut_ref_to_all(&3).unwrap();
        s.clear();
       assert_eq!(true, n.contains_key(&3));
       assert_eq!(Vec::<String>::new(), n.get_all(&3));
       assert_eq!(false, n.contains_key(&3));
    }
    #[test]
    /// # [`general::Map::get_all_ref`] Test
    fn test_get_all_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       assert_eq!(["hello".to_string(), "world".to_string(), "chao".to_string()].to_vec(), n.get_all_ref(&3));
    }
    #[test]
    /// # [`general::Map::get_all_ref`] Test 2
    /// Here we test the function when the key not exist
    fn test_2_get_all_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       assert_eq!(Vec::<String>::new(), n.get_all(&5));
    }
    #[test]
    /// # [`general::Map::get_all_ref`] Test 3
    /// Here we test the funciton when the vecdeque for the key is empty
    fn test_3_get_all_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
      let s = n.get_mut_ref_to_all_ref(&3).unwrap();
        s.clear();
       assert_eq!(true, n.contains_key_ref(&3));
       assert_eq!(Vec::<String>::new(), n.get_all_ref(&3));
       assert_eq!(false, n.contains_key_ref(&3));
    }
    #[test]
    /// # [`general::Map::get_mut_ref_to_all`] Test
    fn test_get_mut_ref_to_all(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3, &"hello".to_string());
       n.insert(&3, &"world".to_string());
       n.insert(&3,&"chao".to_string());
       let vec_mut = n.get_mut_ref_to_all(&3).unwrap();
       vec_mut.push_back("new_value".to_string());
       let mut s = VecDeque::new();
       s.push_back("hello".to_string());
       s.push_back("world".to_string());
        s.push_back("chao".to_string());
        s.push_back("new_value".to_string());
       assert_eq!(s, *n.get_ref_to_all(&3).unwrap());
       assert_eq!(None, n.get_mut_ref_to_all(&5));
    }
    #[test]
    /// # [`general::Map::get_mut_ref_to_all_ref`] Test
    fn test_get_mut_ref_to_all_ref(){
      
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       let vec_mut = n.get_mut_ref_to_all_ref(&3).unwrap();
       vec_mut.push_back(Rc::new("new_value".to_string()));
       let mut s = VecDeque::new();
        s.push_back(Rc::new("hello".to_string()));
        s.push_back(Rc::new("world".to_string()));
        s.push_back(Rc::new("chao".to_string()));
        s.push_back(Rc::new("new_value".to_string()));
       assert_eq!(s, *n.get_ref_to_all_ref(&3).unwrap());
       assert_eq!(None, n.get_mut_ref_to_all_ref(&5));
    }
    #[test]
    /// # [`general::Map::get_mut_ref_to_all_something`] Test
    fn test_get_mut_ref_to_all_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       let vec_mut = n.get_mut_ref_to_all_something(&3).unwrap();
       vec_mut.push_back(Some(40));
       let mut s = VecDeque::new();
        s.push_back(Some(10));
        s.push_back(Some(20));
        s.push_back(Some(30));
        s.push_back(Some(40));
       assert_eq!(s, *n.get_ref_to_all_something(&3).unwrap());
        assert_eq!(None, n.get_mut_ref_to_all_something(&5));
    }
    #[test]
    /// # [`general::Map::get_ref_to_all`] Test
    fn test_get_ref_to_all(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       let mut s = VecDeque::new();
       s.push_back("hello".to_string());
       s.push_back("world".to_string());
       s.push_back("chao".to_string());
       assert_eq!(s, *n.get_ref_to_all(&3).unwrap());
       assert_eq!(None, n.get_ref_to_all(&5));
    }
    #[test]
    /// # [`general::Map::get_ref_to_all_ref`] Test
    fn test_get_ref_to_all_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       let mut s = VecDeque::new();
       s.push_back(Rc::new("hello".to_string()));
       s.push_back(Rc::new("world".to_string()));
       s.push_back(Rc::new("chao".to_string()));
       assert_eq!(s, *n.get_ref_to_all_ref(&3).unwrap());
       assert_eq!(None, n.get_ref_to_all_ref(&5));
    }
    #[test]
    /// # [`general::Map::get_ref_to_all_something`] Test
    fn test_get_ref_to_all_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       let mut s = VecDeque::new();
       s.push_back(Some(10));
       s.push_back(Some(20));
       s.push_back(Some(30));
       assert_eq!(s, *n.get_ref_to_all_something(&3).unwrap());
       assert_eq!(None, n.get_ref_to_all_something(&5));
    }
    #[test]
    /// # [`general::Map::set_value`] Test
    fn test_set_value(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       n.set_value(&3, &["new_world".to_string()].to_vec());
       let mut s = VecDeque::new();
       s.push_back("new_world".to_string());
       assert_eq!(s, *n.get_ref_to_all(&3).unwrap());
  }
  #[test]
  /// # [`general::Map::set_value_ref`] Test
    fn test_set_value_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       n.set_value_ref(&3, ["new_world".to_string()].to_vec());
       let mut s = VecDeque::new();
       s.push_back(Rc::new("new_world".to_string()));
       assert_eq!(s, *n.get_ref_to_all_ref(&3).unwrap());
   }
  #[test]
  /// # [`general::Map::set_value_something`] Test
    fn test_set_value_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       n.set_value_something(&3, [Some(99)].to_vec());
       let mut s = VecDeque::new();
        s.push_back(Some(99));
       assert_eq!(s, *n.get_ref_to_all_something(&3).unwrap());
   }
  #[test]
  /// # [`general::Map::contains_key`] Test
    fn test_contains_key(){
      let mut n:general::Map<usize, usize, Option<i32>> = general::Map::new();
       n.insert(&3,&10);
       n.insert(&3, &20);
       n.insert(&3, &30);
       assert_eq!(true, n.contains_key(&3));
       assert_eq!(false, n.contains_key(&5));
    }
  #[test]
  /// # [`general::Map::contains_key_ref`] Test
    fn test_contains_key_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       assert_eq!(true, n.contains_key_ref(&3));
       assert_eq!(false, n.contains_key_ref(&5));

    }
  #[test]
  /// # [`general::Map::contains_key_something`] Test
    fn test_contains_key_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       assert_eq!(true, n.contains_key_something(&3));
       assert_eq!(false, n.contains_key_something(&5));
    }
  #[test]
  /// # [`general::Map::set_value_element`] Test
    fn test_set_value_element(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       n.set_value_element(&3, 1, &"new_world".to_string());
       let mut s = VecDeque::new();
        s.push_back("hello".to_string());
        s.push_back("new_world".to_string());
        s.push_back("chao".to_string());
       assert_eq!(s, *n.get_ref_to_all(&3).unwrap());
    }
  #[test]
  /// # [`general::Map::set_value_element_ref`] Test
    fn test_set_value_element_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       n.set_value_element_ref(&3, 1, "new_world".to_string());
       let mut s = VecDeque::new();
        s.push_back(Rc::new("hello".to_string()));
        s.push_back(Rc::new("new_world".to_string()));
        s.push_back(Rc::new("chao".to_string()));
       assert_eq!(s, *n.get_ref_to_all_ref(&3).unwrap());
    }
  #[test]
  /// # [`general::Map::set_value_element_something`] Test
    fn test_set_value_element_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       n.set_value_element_something(&3, 1, Some(99));
       let mut s = VecDeque::new();
        s.push_back(Some(10));
        s.push_back(Some(99));
        s.push_back(Some(30));
       assert_eq!(s, *n.get_ref_to_all_something(&3).unwrap());
    }
  #[test]
  /// # [`general::Map::enable_global_order`] Test
    fn test_enable_global_order(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
      n.enable_global_order(false, false);
       n.insert(&3,& "hello".to_string());
       n.insert(&5,& "world".to_string());
       n.insert(&2, &"chao".to_string());
       n.insert_ref(&2, "chao".to_string());
       n.insert_something(&2, Some(10));
       assert_eq!([3,5,2,2,2].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::enable_global_order`] Test 2
    fn test_2_enable_global_order(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_global_order(true, false);
       n.insert(&3,& "hello".to_string());
       n.insert(&5,& "world".to_string());
       n.insert(&2, &"chao".to_string());
       n.insert_ref(&5, "chao".to_string());
       n.insert_something(&3, Some(10));
       assert_eq!([2,5,3].to_vec(), n.get_order());   
    }
  #[test]
  /// # [`general::Map::enable_global_order`] Test 3
    fn test_3_enable_global_order(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_global_order(false, false);
       n.insert(&3,& "hello".to_string());
       n.insert(&5,& "world".to_string());
       n.insert(&2, &"chao".to_string());
       n.enable_global_order(true, false);
       n.insert_ref(&5, "chao".to_string());
       n.insert_something(&3, Some(10));
       assert_eq!([2,5,3].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::enable_order`] Test
    fn test_enable_order(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order(true, false);
       n.insert(&3,& "hello".to_string());
       n.insert(&5,& "world".to_string());
       n.insert(&2, &"chao".to_string());
       n.insert_ref(&5, "chao".to_string());
       n.insert_something(&3, Some(10));
       assert_eq!([3,5,2].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::enable_order`] Test 2
    fn test_2_enable_order(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order(false, false);
       n.insert(&3,& "hello".to_string());
       n.insert(&5,& "world".to_string());
       n.insert(&2, &"chao".to_string());
       n.enable_order(true, true);
       n.insert_ref(&5, "chao".to_string());
       n.insert_something(&3, Some(10));
       n.insert(&3, &"new_value".to_string());
       assert_eq!([3,5,2,3].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::enable_order`] Test 3
    fn test_3_enable_order(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order(false, false);
       n.insert(&3,& "hello".to_string());
       n.insert(&5,& "world".to_string());
       n.insert(&2, &"chao".to_string());
       n.enable_order(true, false);
       n.insert(&5, &"chao".to_string());
       n.insert_something(&3, Some(10));
       assert_eq!([3,2,5].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::enable_order_for_ref`] Test
    fn test_enable_order_for_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order_for_ref(true, false);
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&5, "world".to_string());
       n.insert_ref(&2, "chao".to_string());
       n.insert(&5, &"chao".to_string());
       n.insert_something(&3, Some(10));
       assert_eq!([3,5,2].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::enable_order_for_ref`] Test 2
    fn test_2_enable_order_for_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order_for_ref(false, false);
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&5, "world".to_string());
       n.insert_ref(&2, "chao".to_string());
       n.enable_order_for_ref(true, true);
       n.insert(&5, &"chao".to_string());
       n.insert_something(&3, Some(10));
       n.insert_ref(&3, "new_value".to_string());
       assert_eq!([3,5,2,3].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::enable_order_for_ref`] Test 3
    fn test_3_enable_order_for_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order_for_ref(false, false);
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&5, "world".to_string());
       n.insert_ref(&2, "chao".to_string());
       n.enable_order_for_ref(true, false);
       n.insert_ref(&5, "chao".to_string());
       n.insert_something(&3, Some(10));
       assert_eq!([3,2,5].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::enable_order_for_something`] Test
    fn test_enable_order_for_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order_for_something(true, false);
       n.insert_something(&3, Some(10));
       n.insert_something(&5, Some(20));
       n.insert_something(&2, Some(30));
       n.insert(&5, &"chao".to_string());
       n.insert_ref(&3, "hello".to_string());
       assert_eq!([3,5,2].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::enable_order_for_something`] Test 2
    fn test_2_enable_order_for_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order_for_something(false, false);
       n.insert_something(&3, Some(10));
       n.insert_something(&5, Some(20));
       n.insert_something(&2, Some(30));
       n.enable_order_for_something(true, true);
       n.insert(&5, &"chao".to_string());
       n.insert_ref(&3, "hello".to_string());
       n.insert_something(&3, Some(99));
       assert_eq!([3,5,2,3].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::enable_order_for_something`] Test 3
    fn test_3_enable_order_for_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order_for_something(false, false);
       n.insert_something(&3, Some(10));
       n.insert_something(&5, Some(20));
       n.insert_something(&2, Some(30));
       n.enable_order_for_something(true, false);
       n.insert_something(&5, Some(100));
       n.insert_ref(&3, "Hello".to_string());
       assert_eq!([3,2,5].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::disable_global_order`] Test
    fn test_disable_global_order(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_global_order(true, false);
       n.insert(&3,& "hello".to_string());
       n.insert(&5,& "world".to_string());
       n.insert(&2, &"chao".to_string());
       n.disable_global_order();
       n.insert_ref(&5, "chao".to_string());
       n.insert_something(&3, Some(10));
       assert_eq!([3,5,2].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::disable_order`] Test
    fn test_disable_order(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order(true, false);
       n.insert(&3,& "hello".to_string());
       n.insert(&5,& "world".to_string());
       n.insert(&2, &"chao".to_string());
       n.disable_order();
       n.insert(&5, &"chao".to_string());
       n.insert_something(&3, Some(10));
       assert_eq!([3,5,2].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::disable_order_for_ref`] Test
    fn test_disable_order_for_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order_for_ref(true, false);
       n.insert_ref(&3,"hello".to_string());
       n.insert_ref(&5, "world".to_string());
       n.insert_ref(&2, "chao".to_string());
       n.disable_order_for_ref();
       n.insert_ref(&5, "chao".to_string());
       n.insert_ref(&3, "Hello".to_string());
       assert_eq!([3,5,2].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::disable_order_for_something`] Test
    fn test_disable_order_for_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order_for_something(true, false);
       n.insert_something(&3, Some(10));
       n.insert_something(&5, Some(20));
       n.insert_something(&2, Some(30));
       n.disable_order_for_something();
       n.insert_something(&5, Some(200));
       n.insert_something(&3, Some(100));
       assert_eq!([3,5,2].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::get_order`] Test
    fn test_get_order(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order(false, false);
       n.insert(&3,& "hello".to_string());
       n.insert(&5,& "world".to_string());
       n.insert(&2, &"chao".to_string());
       assert_eq!([3,5,2].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::get_order_ref`] Test
    fn test_get_order_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order_for_ref(false, false);
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&5, "world".to_string());
       n.insert_ref(&2, "chao".to_string());
       let mut r = VecDeque::new();
       r.push_back(0);
       let mut r1 = VecDeque::new();
       r1.push_back(1);
       let mut r2 = VecDeque::new();
       r2.push_back(2);
       let mut s0 = HashMap::new();
       s0.insert(3, r);
       s0.insert(5, r1);
       s0.insert(2, r2);
       let mut s1 = HashMap::new();
       s1.insert(0, 3);
       s1.insert(1, 5);
       s1.insert(2, 2);
       assert_eq!((&s0, &s1), n.get_order_ref());
    }
  #[test]
  /// # [`general::Map::get_order_mut_ref`] Test
    fn test_get_order_mut_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order(false, false);
       n.insert(&3,& "hello".to_string());
       n.insert(&5,& "world".to_string());
       n.insert(&2, &"chao".to_string());
       let order_mut = n.get_order_mut_ref();
       order_mut.0.insert(5, VecDeque::from([0]));
       order_mut.1.insert(0,5);
       assert_eq!([5,5,2].to_vec(), n.get_order());
    }
  #[test]
  /// # [`general::Map::remover_order`] Test
    fn test_remover_order(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order(false, false);
       n.insert(&3,& "hello".to_string());
       n.insert(&5,& "world".to_string());
       n.insert(&2, &"chao".to_string());
       n.remove_order();
       let s:Vec<usize> = Vec::new();
       assert_eq!(s, n.get_order());
    }
  #[test]
  /// # [`general::Map::get_order_num`] Test
    fn test_get_order_num(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order(false, false);
       n.insert(&3,& "hello".to_string());
       n.insert(&5,& "world".to_string());
       n.insert(&2, &"chao".to_string());
       assert_eq!(3, n.get_order_num());
    }
  #[test]
  #[ignore]
  /// # [`general::Map::get_key`] Test
  /// HashMap iteration order is arbitrary (non-deterministic), so the test checks the 'get_key' function without assuming a specific order.
  /// This test can fail due to the arbitrary order of HashMap, consider using a different data structure if ordering matters.
    fn test_get_key(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order(false, false);
       n.insert(&3,& "hello".to_string());
       n.insert(&5,& "world".to_string());
       n.insert(&2, &"hello".to_string());
       assert_eq!([&3,&2].to_vec(), n.get_key(&"hello".to_string()));
    }
  #[test]
  #[ignore]
  /// # [`general::Map::get_key_ref`] Test
  /// HashMap iteration order is arbitrary (non-deterministic), so the test checks the 'get_key_ref' function without assuming a specific order.
  /// This test can fail due to the arbitrary order of HashMap, consider using a different data structure if ordering matters.
    fn test_get_key_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order(false, false);
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&5, "world".to_string());
       n.insert_ref(&2, "hello".to_string());
       assert_eq!([&3,&2].to_vec(), n.get_key_ref("hello".to_string()));
    }
  #[test]
  #[ignore]
  /// # [`general::Map::keys`] Test
  /// HashMap iteration order is arbitrary (non-deterministic), so the test checks the 'keys' function without assuming a specific order.
  /// This test can fail due to the arbitrary order of HashMap, consider using a different data structure if ordering matters.
    fn test_keys(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order(false, false);
       n.insert(&3,& "hello".to_string());
       n.insert(&5,& "world".to_string());
       n.insert(&2, &"chao".to_string());
       let mut s = Vec::new();
       for i in n.keys(){
        s.push(*i);
       }
       assert_eq!([2,3,5].to_vec(), s);
    }
  #[test]
  #[ignore]
  /// # [`general::Map::keys_ref`] Test
  /// HashMap iteration order is arbitrary (non-deterministic), so the test checks the 'keys_ref' function without assuming a specific order.
  /// This test can fail due to the arbitrary order of HashMap, consider using a different data structure if ordering matters.
    fn test_keys_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order(false, false);
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&5, "world".to_string());
       n.insert_ref(&2, "chao".to_string());
       let mut s = Vec::new();
       for i in n.keys_ref(){
        s.push(*i);
       }
       assert_eq!([3,2,5].to_vec(), s);
    }
  #[test]
  #[ignore]
  /// # [`general::Map::keys_something`] Test
  /// HashMap iteration order is arbitrary (non-deterministic), so the test checks the 'keys_something' function without assuming a specific order.
  /// This test can fail due to the arbitrary order of HashMap, consider using a different data structure if ordering matters.
    fn test_keys_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order(false, false);
       n.insert_something(&3, Some(10));
       n.insert_something(&5, Some(20));
       n.insert_something(&2, Some(30));
       let mut s = Vec::new();
       for i in n.keys_something(){
        s.push(*i);
       }
       assert_eq!([2,5,3].to_vec(), s);
    }
  #[test]
  /// # [`general::Map::get_value`] Test
    fn test_get_value(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       assert_eq!(Some(&"hello".to_string()), n.get_value(&3, 0));
       assert_eq!(None, n.get_value(&5, 0));
    }
  #[test]
  /// # [`general::Map::get_value_ref`] Test
    fn test_get_value_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       assert_eq!(Some(&"hello".to_string()), n.get_value_ref(&3, 0));
       assert_eq!(None, n.get_value_ref(&5, 0));
    }
  #[test]
  /// # [`general::Map::get_value_something`] Test
    fn test_get_value_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       assert_eq!(Some(&Some(10)), n.get_value_something(&3, 0));
       assert_eq!(None, n.get_value_something(&5, 0));
    }
  #[test]
  /// # [`general::Map::extract_value`] Test
    fn test_extract_value(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       assert_eq!(Some("hello".to_string()),n.extract_value(&3, 0));
       assert_eq!(Some("world".to_string()), n.extract_value(&3, 0));
       assert_eq!(None, n.get_value(&3, 2));
    }
  #[test]
  /// # [`general::Map::extract_value_ref`] Test
    fn test_extract_value_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       assert_eq!("hello".to_string(),*(n.extract_value_ref(&3, 0).unwrap()));
       assert_eq!("world".to_string(), *(n.extract_value_ref(&3, 0).unwrap()));
       assert_eq!(None, n.get_value_ref(&3, 2));
    } 
  #[test]
  /// # [`general::Map::extract_value_something`] Test
    fn test_extract_value_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       assert_eq!(Some(Some(10)),n.extract_value_something(&3, 0));
       assert_eq!(Some(Some(20)), n.extract_value_something(&3, 0));
       assert_eq!(None, n.get_value_something(&3, 2));
    }
  #[test]
  #[ignore]
  /// # [`general::Map::lifo`] Test
  /// HashMap iteration order is arbitrary (non-deterministic), so the test checks the lifo function without assuming a specific order.
  /// This test can fail due to the arbitrary order of HashMap, consider using a different data structure if ordering matters.
    fn test_lifo(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       assert_eq!("chao".to_string(),n.lifo(&3).unwrap());
       let mut s = VecDeque::new();
        s.push_back("hello".to_string());
        s.push_back("world".to_string());
       assert_eq!(s, *n.get_ref_to_all(&3).unwrap());
       assert_eq!(None, n.lifo(&5));
    }
  #[test]
  #[ignore]
  /// # [`general::Map::lifo_ref`] Test
  ///HashMap iteration order is arbitrary (non-deterministic), so the test checks the lifo_ref function without assuming a specific order.
  /// This test can fail due to the arbitrary order of HashMap, consider using a different data structure if ordering matters.
    fn test_lifo_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       assert_eq!("chao".to_string(),*(n.lifo_ref(&3).unwrap()));
       let mut s = VecDeque::new();
        s.push_back(Rc::new("hello".to_string()));
        s.push_back(Rc::new("world".to_string()));
       assert_eq!(s, *n.get_ref_to_all_ref(&3).unwrap());
       assert_eq!(None, n.lifo_ref(&5));
    }
  #[test]
  #[ignore]
  /// # [`general::Map::lifo_something`] Test
  /// HashMap iteration order is arbitrary (non-deterministic), so the test checks the lifo_something function without assuming a specific order.
  /// This test can fail due to the arbitrary order of HashMap, consider using a different data structure if ordering matters.
    fn test_lifo_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       assert_eq!(Some(30),n.lifo_something(&3).unwrap());
       let mut s = VecDeque::new();
        s.push_back(Some(10));
        s.push_back(Some(20));
       assert_eq!(s, *n.get_ref_to_all_something(&3).unwrap());
       assert_eq!(None, n.lifo_something(&5));
    }
  #[test]
  /// # [`general::Map::peek`]
     fn test_peek(){
        let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
        n.insert(&3,& "hello".to_string());
        n.insert(&3,& "world".to_string());
        n.insert(&3, &"chao".to_string());
        assert_eq!(Some(&"world".to_string()), n.peek(&3));
        assert_eq!(None, n.peek(&5));
    }
  #[test]
  /// # [`general::Map::peek_ref`]
     fn test_peek_ref(){
        let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
        n.insert_ref(&3, "hello".to_string());
        n.insert_ref(&3, "world".to_string());
        n.insert_ref(&3, "chao".to_string());
        assert_eq!(Some(&"world".to_string()), n.peek_ref(&3));
        assert_eq!(None, n.peek_ref(&5));
    }
  #[test]
  /// # [`general::Map::peek_something`]
     fn test_peek_something(){
        let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
        n.insert_something(&3, Some(10));
        n.insert_something(&3, Some(20));
        n.insert_something(&3, Some(30));
        assert_eq!(Some(&Some(20)), n.peek_something(&3));
        assert_eq!(None, n.peek_something(&5));
    }
  #[test]
  /// # [`general::Map::clear`] Test
    fn test_clear(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       n.insert_ref(&5, "hello".to_string());
       n.insert_ref(&5, "world".to_string());
       n.insert_something(&7, Some(10));
       n.insert_something(&7, Some(20));
       n.clear();
       assert_eq!(None, n.get_ref_to_all(&3));
       assert_eq!(None, n.get_ref_to_all_ref(&5));
       assert_eq!(None, n.get_ref_to_all_something(&7));
    }
  #[test]
  /// # [`general::Map::clear_hash`] Test
    fn test_clear_hash(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       n.insert_ref(&5, "hello".to_string());
       n.insert_ref(&5, "world".to_string());
       n.insert_something(&7, Some(10));
       n.insert_something(&7, Some(20));
       n.clear_hash();
       assert_eq!(0, n.len());
       assert_eq!(1, n.len_ref());
       assert_eq!(1, n.len_something());
    }
  #[test]
  /// # [`general::Map::clear_hash_ref`] Test
    fn test_clear_hash_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       n.insert_ref(&5, "hello".to_string());
       n.insert_ref(&5, "world".to_string());
       n.insert_something(&7, Some(10));
       n.insert_something(&7, Some(20));
       n.clear_hash_ref();
       assert_eq!(1, n.len());
       assert_eq!(0, n.len_ref());
       assert_eq!(1, n.len_something());
    }
  #[test]
  /// # [`general::Map::clear_hash_something`] Test
    fn test_clear_hash_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       n.insert_ref(&5, "hello".to_string());
       n.insert_ref(&5, "world".to_string());
       n.insert_something(&7, Some(10));
       n.insert_something(&7, Some(20));
       n.clear_hash_something();
       assert_eq!(1, n.len());
       assert_eq!(1, n.len_ref());
       assert_eq!(0, n.len_something());
    }
  #[test]
  /// # [`general::Map::iter_mut`]
    fn test_iter_mut(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&5, &"chao".to_string());
       for (key, values) in n.iter_mut(){
        if *key == 3{
          for value in values{
            *value = format!("{}{}", *value, "_modified");
          }
        }
       }
       let mut s = VecDeque::new();
        s.push_back("hello_modified".to_string());
        s.push_back("world_modified".to_string());
       assert_eq!(s, *n.get_ref_to_all(&3).unwrap());
       let mut s2 = VecDeque::new();
        s2.push_back("chao".to_string());
       assert_eq!(s2, *n.get_ref_to_all(&5).unwrap());
    }
  #[test]
  /// # [`general::Map::iter_mut_ref`]
    fn test_iter_mut_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&5, "chao".to_string());
       for (key, values) in n.iter_mut_ref(){
        if *key == 3{
          for value in values{
            *value = Rc::new(format!("{}{}", **value, "_modified"));
          }
        }
       }
       let mut s = VecDeque::new();
        s.push_back(Rc::new("hello_modified".to_string()));
        s.push_back(Rc::new("world_modified".to_string()));
       assert_eq!(s, *n.get_ref_to_all_ref(&3).unwrap());
       let mut s2 = VecDeque::new();
        s2.push_back(Rc::new("chao".to_string()));
       assert_eq!(s2, *n.get_ref_to_all_ref(&5).unwrap());
    }
  #[test]
  /// # [`general::Map::iter_mut_something`]
    fn test_iter_mut_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&5, Some(30));
       for (key, values) in n.iter_mut_something(){
        if *key == 3{
          for value in values{
            if let Some(v) = value{
              *value = Some(*v + 100);
            }
          }
        }
       }
       let mut s = VecDeque::new();
        s.push_back(Some(110));
        s.push_back(Some(120));
       assert_eq!(s, *n.get_ref_to_all_something(&3).unwrap());
       let mut s2 = VecDeque::new();
        s2.push_back(Some(30));
       assert_eq!(s2, *n.get_ref_to_all_something(&5).unwrap());
    }  
  #[test]
  #[ignore]
  /// # [`general::Map::iter`] Test
  /// In this test the order is descending because that is the order in the HashMap, because HashMap iteration order is arbitrary (non-deterministic), so the test checks the iterator without assuming a specific order.
  /// This test can fail due to the arbitrary order of HashMap, consider using a different data structure if ordering matters.
    fn test_iter(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&5, &"chao".to_string());
       let mut iter = n.iter();
       let (key, values) = iter.next().unwrap();
       assert_eq!(&5, key);
       let mut s = VecDeque::new();
       s.push_back("chao".to_string());    
       assert_eq!(s, *values);
       let (key2, values2) = iter.next().unwrap();
       assert_eq!(&3, key2);
       let mut s2 = VecDeque::new();
        s2.push_back("hello".to_string());
        s2.push_back("world".to_string());
       assert_eq!(s2, *values2);
    }
  #[test]
  #[ignore]
  /// # [`general::Map::iter_ref`] Test
  /// HashMap iteration order is arbitrary (non-deterministic), so the test checks the iterator without assuming a specific order.
  /// This test can fail due to the arbitrary order of HashMap, consider using a different data structure if ordering matters.
    fn test_iter_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&5, "chao".to_string());
       let mut iter = n.iter_ref();
       let (key, values) = iter.next().unwrap();
       assert_eq!(&3, key);
       let mut s = VecDeque::new();
        s.push_back(Rc::new("hello".to_string()));
        s.push_back(Rc::new("world".to_string()));
       assert_eq!(s, *values);
       let (key2, values2) = iter.next().unwrap();
       assert_eq!(&5, key2);
       let mut s2 = VecDeque::new();
        s2.push_back(Rc::new("chao".to_string()));
       assert_eq!(s2, *values2);
    }
  #[test]
  #[ignore]
  /// # [`general::Map::iter_something`] Test
  /// HashMap iteration order is arbitrary (non-deterministic), so the test checks the iterator without assuming a specific order.
  /// This test can fail due to the arbitrary order of HashMap, consider using a different data structure if ordering matters.
    fn test_iter_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&5, Some(30));
       let mut iter = n.iter_something();
       let (key, values) = iter.next().unwrap();
       assert_eq!(&3, key);
       let mut s = VecDeque::new();
         s.push_back(Some(10));
        s.push_back(Some(20));
       assert_eq!(s, *values);
       let (key2, values2) = iter.next().unwrap();
       assert_eq!(&5, key2);
       let mut s2 = VecDeque::new();
        s2.push_back(Some(30));
       assert_eq!(s2, *values2);
    }
  #[test]
  /// # [`general::Map::get_mut_value`] Test
    fn test_get_mut_value(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       if let Some(value) = n.get_mut_value(&3, 1){
        *value = "modified".to_string();
       }
       assert_eq!(Some(&"modified".to_string()), n.get_value(&3, 1));
       assert_eq!(None, n.get_mut_value(&5, 0));
    }
  #[test]
  /// # [`general::Map::get_mut_value_ref`] Test
    fn test_get_mut_value_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       if let Some(value) = n.get_mut_value_ref(&3, 1){
        *value = Rc::new("modified".to_string());
       }
       assert_eq!(Some(&"modified".to_string()), n.get_value_ref(&3, 1));
       assert_eq!(None, n.get_mut_value_ref(&5, 0));
    }
  #[test]
  /// # [`general::Map::get_mut_value_something`] Test
    fn test_get_mut_value_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       if let Some(value) = n.get_mut_value_something(&3, 1){
        *value = Some(999);
       }
       assert_eq!(Some(&Some(999)), n.get_value_something(&3, 1));
       assert_eq!(None, n.get_mut_value_something(&5, 0));
    }
  #[test]
  /// # [`general::Map::remove_value`] Test
    fn test_remove_value(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       n.insert(&3, &"chao".to_string());
       n.remove_value(&3, 2);
       assert_eq!(None, n.get_value(&3, 2));
    }
  #[test]
  /// # [`general::Map::remove_value_ref`] Test
    fn test_remove_value_from_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       n.insert_ref(&3, "chao".to_string());
       n.remove_value_from_ref(&3, 1);
       assert_eq!("chao".to_string(), *n.get_value_ref(&3, 1).unwrap());
    }
  #[test]
  /// # [`general::Map::remove_value_something`] Test
    fn test_remove_value_from_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       n.insert_something(&3, Some(30));
       n.remove_value_from_something(&3, 0);
       assert_eq!(Some(20), *n.get_value_something(&3, 0).unwrap());
    }
  #[test]
  /// # [`general::Map::reset_all`] Test
    fn test_reset_all(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_global_order(true, true);
       n.insert(&3,& "hello".to_string());
       n.insert_ref(&5, "world".to_string());
       n.insert_something(&7, Some(10));
       n.reset_all();
       n.insert(&2, &"chao".to_string());
       n.insert_ref(&4, "hola".to_string());
       n.insert_something(&6, Some(20));
       println!("{:?}", n.get_order());
       assert_eq!((0,0), n.order_len());
       assert_eq!(0, n.get_order_num());
       assert_eq!(1, n.len());
       assert_eq!(1, n.len_ref());
       assert_eq!(1, n.len_something());
    }
  #[test]
  /// # [`general::Map::is_empty`] Test
    fn test_is_empty(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       assert!(n.is_empty());
       n.insert(&3,& "hello".to_string());
       assert!(!n.is_empty());
    }
  #[test]
  /// # [`general::Map::len`] Test
    fn test_len(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       assert_eq!(0, n.len());
       n.insert(&3,& "hello".to_string());
       n.insert(&3,& "world".to_string());
       assert_eq!(1, n.len());
    }
  #[test]
  /// # [`general::Map::len_ref`] Test
    fn test_len_ref(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       assert_eq!(0, n.len_ref());
       n.insert_ref(&3, "hello".to_string());
       n.insert_ref(&3, "world".to_string());
       assert_eq!(1, n.len_ref());
    }
  #[test]
  /// # [`general::Map::len_something`] Test
    fn test_len_something(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       assert_eq!(0, n.len_something());
       n.insert_something(&3, Some(10));
       n.insert_something(&3, Some(20));
       assert_eq!(1, n.len_something());
    }
  #[test]
  /// # [`general::Map::total_len`] Test
    fn test_total_len(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       assert_eq!(0, n.total_len());
       n.insert(&3,& "hello".to_string());
       n.insert_ref(&5, "world".to_string());
       n.insert_something(&7, Some(10));
       assert_eq!(3, n.total_len());
    }
  #[test]
  /// # [`general::Map::order_len`] Test
    fn test_order_len(){
      let mut n:general::Map<usize, String, Option<i32>> = general::Map::new();
       n.enable_order(false, false);
       assert_eq!((0,0), n.order_len());
       n.insert(&3,& "hello".to_string());
       n.insert(&5,& "world".to_string());
       n.insert(&2, &"chao".to_string());
       assert_eq!((3, 3), n.order_len());
    }

  }