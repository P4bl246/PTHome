mod main_code;
fn main() {
    use crate::main_code::utilities::remove_comments;
    let file_path = "C:/Users/cr1pt/repositorios VSCode clonados/PTHome/source_code/src/example.txt";
    let n = remove_comments::block_comments(file_path, "/*", "*/", remove_comments::ModeBlock::Nested);
    println!("Number of comments removed: {}", n);
}