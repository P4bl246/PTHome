mod main_code;
fn main() {
    use crate::main_code::utilities::{remove_comments, general};
    general::remove_empty_lines("C:/Users/cr1pt/repositorios VSCode clonados/PTHome/source_code/src/example.txt");

    let n = remove_comments::block_comments("C:/Users/cr1pt/repositorios VSCode clonados/PTHome/source_code/src/example.txt", "/*", "*/", remove_comments::ModeBlock::Single);

    println!("Processing completed for file: {}", n);
}
