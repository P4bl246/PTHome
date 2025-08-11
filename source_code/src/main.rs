mod main_code;
fn main() {
  use crate::main_code::utilities::remove_comments;
    let file_path = "C:/Users/cr1pt/Vscode_clonados/PTHome/source_code/src/example.txt";
    let ignore_this = ['"', '"'].to_vec();
    let ignore_this2 = ["'", "'"].to_vec();
    let ignore = (&ignore_this, &ignore_this2);
    let n = remove_comments::simple_comments(file_path, "//", ignore, true);
    match n{
        None => println!("Error"),
        _ => println!("{:#?}", n)
    }
    println!("Number of comments removed: {:#?}", n);
}