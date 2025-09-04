
mod main_code;
fn main() {
//  use crate::main_code::utilities::formats;
  use crate::main_code::parsing_sintax::parser;
    //  use std::io::Write;
    use std::fs::{self};
    let file_path = "C:/Users/cr1pt/Vscode_clonados/PTHome/source_code/src/example.txt";
    let content = fs::read_to_string(file_path).expect(&format!("Failed to read the file '{}'", file_path));  
    let scape = ['\\'].to_vec();
    let search: Vec<&'static str> = vec!["=", ":"];
    let mut n = parser::slice_classes_equ(&content);
    match n{
     
          None=>{println!("None")},
          Some(i)=>{println!("{:#?}", i)}
        
      
    };
  
}