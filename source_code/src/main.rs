
mod main_code;
fn main() {
  //use crate::main_code::utilities::remove_comments;
  use crate::main_code::parsing_sintax::parser;
     // use std::io::Write;
  use std::fs::{self};
    let file_path = "C:/Users/cr1pt/Vscode_clonados/PTHome/source_code/src/example.txt";
    let ignore_this = ['"', '"'].to_vec();
    let ignore_this2 = ["'", "'"].to_vec();
    let ignore = (&ignore_this, &ignore_this2);
    let content = fs::read_to_string(file_path).expect(&format!("Failed to read the file '{}'", file_path));  
    let scape = ['\\'].to_vec();
    let search = vec![":", "="];
    let n = parser::filter_lines(&content, &search, &scape, ignore);
    
    match n{
        None => println!("Error"),
        Some(i)=> {
          println!("{:#?}", i);
      }

        
    }
}