mod main_code;
fn main() {
  use crate::main_code::utilities::remove_comments;
      use std::io::Write;
  use std::fs::{self};
    let file_path = "C:/Users/cr1pt/Vscode_clonados/PTHome/source_code/src/example.txt";
    let ignore_this = ['"', '"'].to_vec();
    let ignore_this2 = ["'", "'"].to_vec();
    let ignore = (&ignore_this, &ignore_this2);
    let content = fs::read_to_string(file_path).expect(&format!("Failed to read the file '{}'", file_path));  

    let n = remove_comments::simple_comments(&content, "//", ignore, true);
    
    match n{
        None => println!("Error"),
        Some(i)=> {
          fs::remove_file(file_path).expect(&format!("Error trying to remove the file '{}'",file_path));
            let mut new_file = fs::File::create(file_path).expect(&format!("Error trying to create the file '{}'", file_path));
            new_file.write_all(i.as_bytes()).expect(&format!("Error trying to write in the file '{}'", file_path));
          println!("{:#?}", i);
      }

        
    }
}